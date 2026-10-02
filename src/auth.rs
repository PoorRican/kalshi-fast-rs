use crate::error::KalshiError;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::Signer as _;
use rand::rngs::OsRng;
use rsa::pss::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

/// Private key material; the signing algorithm follows the key type.
#[derive(Clone)]
enum SigningKeyKind {
    /// RSA-PSS with SHA-256 (the key is pre-wrapped once; cloning is cheap).
    Rsa(SigningKey<Sha256>),
    /// Ed25519 (RFC 8032) over the same pre-sign text.
    Ed25519(ed25519_dalek::SigningKey),
}

impl std::fmt::Debug for SigningKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print key material.
        match self {
            Self::Rsa(_) => f.write_str("Rsa(..)"),
            Self::Ed25519(_) => f.write_str("Ed25519(..)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KalshiAuth {
    pub key_id: String,
    key: SigningKeyKind,
}

/// Convenience container for the three auth headers.
#[derive(Debug, Clone)]
pub struct KalshiAuthHeaders {
    pub key: String,
    pub timestamp_ms: String,
    pub signature: String,
}

impl KalshiAuth {
    /// Load a `.key` PEM file (Kalshi UI downloads a private key as `.key`).
    pub fn from_pem_file(
        key_id: impl Into<String>,
        pem_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, KalshiError> {
        let pem = std::fs::read_to_string(pem_path)?;
        Self::from_pem_str(key_id, &pem)
    }

    /// Load from a PEM string. Supports RSA (PKCS#8 or PKCS#1) and Ed25519
    /// (PKCS#8, `-----BEGIN PRIVATE KEY-----`) keys; the signing algorithm is
    /// selected from the key type.
    pub fn from_pem_str(key_id: impl Into<String>, pem: &str) -> Result<Self, KalshiError> {
        let key_id = key_id.into();

        let key = if let Ok(rsa) = RsaPrivateKey::from_pkcs8_pem(pem) {
            SigningKeyKind::Rsa(SigningKey::<Sha256>::new(rsa))
        } else if let Ok(ed) = ed25519_dalek::SigningKey::from_pkcs8_pem(pem) {
            SigningKeyKind::Ed25519(ed)
        } else {
            let rsa = RsaPrivateKey::from_pkcs1_pem(pem)
                .map_err(|e| KalshiError::Crypto(format!("unsupported private key PEM: {e}")))?;
            SigningKeyKind::Rsa(SigningKey::<Sha256>::new(rsa))
        };

        Ok(Self { key_id, key })
    }

    /// Milliseconds since UNIX epoch, as required by Kalshi auth headers.
    pub fn now_timestamp_ms() -> String {
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_millis();
        ms.to_string()
    }

    /// Create signature for a request:
    /// `message = timestamp + METHOD + path_without_query`,
    /// `signature = RSA-PSS(SHA256)` for RSA keys or `Ed25519` for Ed25519 keys,
    /// base64 encoded.
    pub fn sign(
        &self,
        timestamp_ms: &str,
        method: &str,
        path: &str,
    ) -> Result<String, KalshiError> {
        let message = Self::signing_message(timestamp_ms, method, path);
        let message_bytes = message.as_bytes();

        match &self.key {
            SigningKeyKind::Rsa(signing_key) => {
                // RSA-PSS is randomized; use OS RNG. PSS with SHA256 (salt
                // length = digest length) per Kalshi docs.
                let mut rng = OsRng;
                let signature = signing_key.sign_with_rng(&mut rng, message_bytes);
                Ok(STANDARD.encode(signature.to_bytes()))
            }
            SigningKeyKind::Ed25519(signing_key) => {
                let signature = signing_key.sign(message_bytes);
                Ok(STANDARD.encode(signature.to_bytes()))
            }
        }
    }

    /// Build the canonical signing message (timestamp + METHOD + path_without_query).
    pub fn signing_message(timestamp_ms: &str, method: &str, path: &str) -> String {
        let method = method.to_uppercase();
        let path_without_query = path.split('?').next().unwrap_or(path);
        format!("{timestamp_ms}{method}{path_without_query}")
    }

    /// Build the three headers required by Kalshi authenticated endpoints.
    pub fn build_headers(
        &self,
        method: &str,
        path: &str,
    ) -> Result<KalshiAuthHeaders, KalshiError> {
        let timestamp_ms = Self::now_timestamp_ms();
        let signature = self.sign(&timestamp_ms, method, path)?;

        Ok(KalshiAuthHeaders {
            key: self.key_id.clone(),
            timestamp_ms,
            signature,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::KalshiAuth;
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use rand::rngs::OsRng;
    use rsa::RsaPrivateKey;
    use rsa::pss::{Signature, VerifyingKey};
    use rsa::signature::Verifier;
    use sha2::Sha256;

    /// Load auth for tests. Optionally loads .env.test, supports both
    /// KALSHI_PRIVATE_KEY (content) and KALSHI_PRIVATE_KEY_PATH (file path).
    pub fn load_test_auth() -> KalshiAuth {
        dotenvy::from_filename(".env.test").ok();

        let key_id = std::env::var("KALSHI_KEY_ID").unwrap_or_else(|_| "test-key-id".to_string());

        if let Ok(pem_content) = std::env::var("KALSHI_PRIVATE_KEY") {
            let pem_content = pem_content.replace("\\n", "\n");
            KalshiAuth::from_pem_str(key_id, &pem_content)
                .expect("load auth from KALSHI_PRIVATE_KEY")
        } else if let Ok(pem_path) = std::env::var("KALSHI_PRIVATE_KEY_PATH") {
            KalshiAuth::from_pem_file(key_id, pem_path)
                .expect("load auth from KALSHI_PRIVATE_KEY_PATH")
        } else {
            let mut rng = OsRng;
            let private_key =
                RsaPrivateKey::new(&mut rng, 2048).expect("generate local test private key");
            KalshiAuth {
                key_id,
                key: super::SigningKeyKind::Rsa(super::SigningKey::<Sha256>::new(private_key)),
            }
        }
    }

    #[test]
    fn signing_message_strips_query() {
        let msg = KalshiAuth::signing_message(
            "1700000000000",
            "get",
            "/trade-api/v2/markets?limit=10&cursor=abc",
        );
        assert_eq!(msg, "1700000000000GET/trade-api/v2/markets");
    }

    fn test_ed25519_key() -> ed25519_dalek::SigningKey {
        use rand::RngCore;
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);
        ed25519_dalek::SigningKey::from_bytes(&secret)
    }

    #[test]
    fn rsa_signature_verifies_with_private_key() {
        let private_key =
            RsaPrivateKey::new(&mut OsRng, 2048).expect("generate local test private key");
        let auth = KalshiAuth {
            key_id: "test-key-id".to_string(),
            key: super::SigningKeyKind::Rsa(super::SigningKey::<Sha256>::new(private_key.clone())),
        };
        let headers = auth
            .build_headers("GET", "/trade-api/v2/markets")
            .expect("build headers");
        let message =
            KalshiAuth::signing_message(&headers.timestamp_ms, "GET", "/trade-api/v2/markets");
        let sig_bytes = STANDARD
            .decode(headers.signature.as_bytes())
            .expect("decode signature");
        let sig = Signature::try_from(sig_bytes.as_slice()).expect("signature");
        let verifying_key = VerifyingKey::<Sha256>::new(private_key.to_public_key());
        verifying_key
            .verify(message.as_bytes(), &sig)
            .expect("signature verifies");
    }

    #[test]
    fn rsa_pkcs8_and_pkcs1_pems_load() {
        use rsa::pkcs1::EncodeRsaPrivateKey;
        use rsa::pkcs8::EncodePrivateKey;
        let private_key =
            RsaPrivateKey::new(&mut OsRng, 2048).expect("generate local test private key");
        let pkcs8 = private_key.to_pkcs8_pem(Default::default()).unwrap();
        let pkcs1 = private_key.to_pkcs1_pem(Default::default()).unwrap();
        for pem in [pkcs8.as_str(), pkcs1.as_str()] {
            let auth = KalshiAuth::from_pem_str("k", pem).expect("load rsa pem");
            assert!(matches!(auth.key, super::SigningKeyKind::Rsa(_)));
        }
    }

    #[test]
    fn ed25519_pkcs8_pem_signs_and_verifies() {
        use ed25519_dalek::pkcs8::EncodePrivateKey;
        use ed25519_dalek::{Signature, Verifier};

        let signing_key = test_ed25519_key();
        let pem = signing_key.to_pkcs8_pem(Default::default()).unwrap();
        let auth = KalshiAuth::from_pem_str("ed-key", pem.as_str()).expect("load ed25519 pem");
        assert!(matches!(auth.key, super::SigningKeyKind::Ed25519(_)));

        let headers = auth
            .build_headers("POST", "/trade-api/v2/portfolio/orders?x=1")
            .expect("build headers");
        assert_eq!(headers.key, "ed-key");
        let message = KalshiAuth::signing_message(
            &headers.timestamp_ms,
            "POST",
            "/trade-api/v2/portfolio/orders",
        );
        let sig_bytes = STANDARD.decode(headers.signature.as_bytes()).unwrap();
        assert_eq!(sig_bytes.len(), 64);
        let sig = Signature::from_slice(&sig_bytes).unwrap();
        signing_key
            .verifying_key()
            .verify(message.as_bytes(), &sig)
            .expect("ed25519 signature verifies");
    }

    #[test]
    fn debug_does_not_leak_key_material() {
        let signing_key = test_ed25519_key();
        let auth = KalshiAuth {
            key_id: "k".into(),
            key: super::SigningKeyKind::Ed25519(signing_key),
        };
        assert!(format!("{auth:?}").contains("Ed25519(..)"));
    }
}
