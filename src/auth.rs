use crate::error::KalshiError;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::rngs::OsRng;
use rsa::pss::SigningKey as RsaPssSigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};
use sha2::Sha256;
use std::fmt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Parsed private key, built once at construction so signing never re-derives
/// or copies key material.
enum SigningMaterial {
    /// RSA-PSS with SHA-256 (digest-sized salt).
    Rsa(RsaPssSigningKey<Sha256>),
    /// Pure Ed25519 (RFC 8032).
    Ed25519(ed25519_dalek::SigningKey),
}

impl SigningMaterial {
    fn algorithm(&self) -> &'static str {
        match self {
            Self::Rsa(_) => "RSA-PSS-SHA256",
            Self::Ed25519(_) => "Ed25519",
        }
    }

    fn sign(&self, message: &[u8]) -> Vec<u8> {
        match self {
            // RSA-PSS is randomized; use OS RNG. Salt length = digest length.
            Self::Rsa(key) => key.sign_with_rng(&mut OsRng, message).to_bytes().into_vec(),
            Self::Ed25519(key) => ed25519_dalek::Signer::sign(key, message)
                .to_bytes()
                .to_vec(),
        }
    }
}

/// Kalshi API credentials: key id plus a private key used to sign requests.
///
/// The private key algorithm is auto-detected on load: Ed25519 (PKCS#8) or
/// RSA (PKCS#8 / PKCS#1). Cloning is cheap; the parsed key is shared.
#[derive(Clone)]
pub struct KalshiAuth {
    pub key_id: String,
    key: Arc<SigningMaterial>,
}

impl fmt::Debug for KalshiAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KalshiAuth")
            .field("key_id", &self.key_id)
            .field("algorithm", &self.key.algorithm())
            .finish_non_exhaustive()
    }
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
    ///
    /// Supports the same key formats as [`KalshiAuth::from_pem_str`].
    pub fn from_pem_file(
        key_id: impl Into<String>,
        pem_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, KalshiError> {
        let pem = std::fs::read_to_string(pem_path)?;
        Self::from_pem_str(key_id, &pem)
    }

    /// Load from a PEM string.
    ///
    /// Supported private keys: Ed25519 PKCS#8 (`BEGIN PRIVATE KEY`, the default
    /// the Kalshi web UI generates), RSA PKCS#8 (`BEGIN PRIVATE KEY`) and RSA
    /// PKCS#1 (`BEGIN RSA PRIVATE KEY`). The algorithm is detected from the key
    /// contents, not the PEM label. Anything else is rejected.
    pub fn from_pem_str(key_id: impl Into<String>, pem: &str) -> Result<Self, KalshiError> {
        let key_id = key_id.into();

        let material = if let Ok(key) = RsaPrivateKey::from_pkcs8_pem(pem) {
            SigningMaterial::Rsa(RsaPssSigningKey::<Sha256>::new(key))
        } else if let Ok(key) = ed25519_dalek::SigningKey::from_pkcs8_pem(pem) {
            SigningMaterial::Ed25519(key)
        } else if let Ok(key) = RsaPrivateKey::from_pkcs1_pem(pem) {
            SigningMaterial::Rsa(RsaPssSigningKey::<Sha256>::new(key))
        } else {
            return Err(KalshiError::Crypto(
                "unsupported or malformed private key: expected Ed25519 PKCS#8, RSA PKCS#8, \
                 or RSA PKCS#1 PEM"
                    .to_string(),
            ));
        };

        Ok(Self {
            key_id,
            key: Arc::new(material),
        })
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
    /// `message = timestamp + METHOD + path_without_query`, signed with the
    /// loaded key, base64 encoded (standard alphabet, raw signature bytes).
    ///
    /// - RSA keys: RSA-PSS with SHA-256 and digest-sized (32 byte) salt.
    /// - Ed25519 keys: pure Ed25519 over the message bytes (RFC 8032).
    pub fn sign(
        &self,
        timestamp_ms: &str,
        method: &str,
        path: &str,
    ) -> Result<String, KalshiError> {
        let message = Self::signing_message(timestamp_ms, method, path);
        Ok(STANDARD.encode(self.key.sign(message.as_bytes())))
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
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rsa::RsaPrivateKey;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs8::{EncodePrivateKey, LineEnding};
    use rsa::pss::{Signature as PssSignature, VerifyingKey as PssVerifyingKey};
    use rsa::signature::Verifier;
    use sha2::Sha256;
    use std::sync::LazyLock;

    // RFC 8032 section 7.1, TEST 1.
    const RFC8032_SEED_HEX: &str =
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
    const RFC8032_PUBLIC_HEX: &str =
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";

    /// PKCS#8 v1 `PrivateKeyInfo` header for an Ed25519 key (OID 1.3.101.112)
    /// followed by `OCTET STRING { OCTET STRING (32) }`: 16 bytes + 32-byte seed.
    const ED25519_PKCS8_PREFIX: [u8; 16] = [
        0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04,
        0x20,
    ];

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
            .collect()
    }

    fn pem_from_der(label: &str, der: &[u8]) -> String {
        let b64 = STANDARD.encode(der);
        let mut pem = format!("-----BEGIN {label}-----\n");
        for chunk in b64.as_bytes().chunks(64) {
            pem.push_str(std::str::from_utf8(chunk).expect("base64 is ascii"));
            pem.push('\n');
        }
        pem.push_str(&format!("-----END {label}-----\n"));
        pem
    }

    fn rfc8032_seed() -> [u8; 32] {
        hex_to_bytes(RFC8032_SEED_HEX)
            .try_into()
            .expect("32-byte seed")
    }

    fn ed25519_pkcs8_pem(seed: &[u8; 32]) -> String {
        let mut der = ED25519_PKCS8_PREFIX.to_vec();
        der.extend_from_slice(seed);
        pem_from_der("PRIVATE KEY", &der)
    }

    /// Deterministic local RSA key (generated once, independent of env files).
    static RSA_FIXTURE: LazyLock<RsaPrivateKey> = LazyLock::new(|| {
        let mut rng = StdRng::seed_from_u64(0x4b41_4c53_4849);
        RsaPrivateKey::new(&mut rng, 2048).expect("generate local test RSA key")
    });

    fn rsa_fixture() -> &'static RsaPrivateKey {
        &RSA_FIXTURE
    }

    fn rsa_pkcs8_pem() -> String {
        rsa_fixture()
            .to_pkcs8_pem(LineEnding::LF)
            .expect("encode pkcs8")
            .to_string()
    }

    fn rsa_pkcs1_pem() -> String {
        rsa_fixture()
            .to_pkcs1_pem(LineEnding::LF)
            .expect("encode pkcs1")
            .to_string()
    }

    /// Load auth for tests. Optionally loads .env.test, supports both
    /// KALSHI_PRIVATE_KEY (content) and KALSHI_PRIVATE_KEY_PATH (file path);
    /// falls back to a local RSA key. Not used by crypto verification tests,
    /// which use fixed local fixtures instead.
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
            KalshiAuth::from_pem_str(key_id, &rsa_pkcs8_pem()).expect("load local RSA fixture")
        }
    }

    fn verify_rsa_pss(public: &rsa::RsaPublicKey, message: &str, signature_b64: &str) {
        let sig_bytes = STANDARD
            .decode(signature_b64.as_bytes())
            .expect("decode signature");
        assert_eq!(sig_bytes.len(), 256, "2048-bit RSA signature is 256 bytes");
        let sig = PssSignature::try_from(sig_bytes.as_slice()).expect("signature");
        PssVerifyingKey::<Sha256>::new(public.clone())
            .verify(message.as_bytes(), &sig)
            .expect("RSA-PSS signature verifies");
    }

    fn verify_ed25519(public: &ed25519_dalek::VerifyingKey, message: &str, signature_b64: &str) {
        let sig_bytes = STANDARD
            .decode(signature_b64.as_bytes())
            .expect("decode signature");
        let sig = ed25519_dalek::Signature::from_slice(&sig_bytes).expect("64-byte signature");
        public
            .verify_strict(message.as_bytes(), &sig)
            .expect("Ed25519 signature verifies");
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

    #[test]
    fn rsa_pkcs8_signature_verifies_and_matches_headers() {
        let auth = KalshiAuth::from_pem_str("rsa-pkcs8-key", &rsa_pkcs8_pem()).expect("load");
        let public = rsa_fixture().to_public_key();

        let headers = auth
            .build_headers("get", "/trade-api/v2/markets?limit=10")
            .expect("build headers");
        assert_eq!(headers.key, "rsa-pkcs8-key");
        assert!(headers.timestamp_ms.parse::<u128>().is_ok());

        let message = format!("{}GET/trade-api/v2/markets", headers.timestamp_ms);
        verify_rsa_pss(&public, &message, &headers.signature);

        // The query string must not be part of the signed text.
        let with_query = format!("{}GET/trade-api/v2/markets?limit=10", headers.timestamp_ms);
        let sig_bytes = STANDARD.decode(&headers.signature).expect("decode");
        let sig = PssSignature::try_from(sig_bytes.as_slice()).expect("signature");
        assert!(
            PssVerifyingKey::<Sha256>::new(public)
                .verify(with_query.as_bytes(), &sig)
                .is_err()
        );
    }

    #[test]
    fn rsa_pkcs1_key_loads_and_signs_verifiably() {
        let pem = rsa_pkcs1_pem();
        assert!(pem.contains("BEGIN RSA PRIVATE KEY"));
        let auth = KalshiAuth::from_pem_str("rsa-pkcs1-key", &pem).expect("load pkcs1");

        let sig = auth
            .sign("1700000000000", "POST", "/trade-api/v2/portfolio/orders")
            .expect("sign");
        verify_rsa_pss(
            &rsa_fixture().to_public_key(),
            "1700000000000POST/trade-api/v2/portfolio/orders",
            &sig,
        );
    }

    #[test]
    fn rsa_signatures_are_randomized_and_clones_share_the_key() {
        let auth = KalshiAuth::from_pem_str("k", &rsa_pkcs8_pem()).expect("load");
        let cloned = auth.clone();
        let public = rsa_fixture().to_public_key();
        let message = "1700000000000GET/trade-api/v2/exchange/status";

        let first = auth
            .sign("1700000000000", "GET", "/trade-api/v2/exchange/status")
            .expect("sign");
        let second = cloned
            .sign("1700000000000", "GET", "/trade-api/v2/exchange/status")
            .expect("sign");
        assert_ne!(first, second, "PSS uses a random salt");
        verify_rsa_pss(&public, message, &first);
        verify_rsa_pss(&public, message, &second);
    }

    #[test]
    fn rsa_signature_rejects_tampered_message() {
        let auth = KalshiAuth::from_pem_str("k", &rsa_pkcs8_pem()).expect("load");
        let sig = auth
            .sign("1700000000000", "GET", "/trade-api/v2/markets")
            .expect("sign");
        let sig_bytes = STANDARD.decode(&sig).expect("decode");
        let sig = PssSignature::try_from(sig_bytes.as_slice()).expect("signature");
        let verifier = PssVerifyingKey::<Sha256>::new(rsa_fixture().to_public_key());
        assert!(
            verifier
                .verify(b"1700000000000POST/trade-api/v2/markets", &sig)
                .is_err()
        );
    }

    #[test]
    fn ed25519_pkcs8_key_signs_canonical_message_deterministically() {
        let seed = rfc8032_seed();
        let pem = ed25519_pkcs8_pem(&seed);
        let auth = KalshiAuth::from_pem_str("ed-key", &pem).expect("load ed25519");

        // The parsed key must be the RFC 8032 key, with the RFC public key.
        let public_bytes: [u8; 32] = hex_to_bytes(RFC8032_PUBLIC_HEX)
            .try_into()
            .expect("32-byte public key");
        let public = ed25519_dalek::VerifyingKey::from_bytes(&public_bytes).expect("public key");

        let signature = auth
            .sign(
                "1700000000000",
                "get",
                "/trade-api/v2/markets?limit=10&cursor=abc",
            )
            .expect("sign");
        verify_ed25519(&public, "1700000000000GET/trade-api/v2/markets", &signature);

        // Ed25519 is deterministic: the signature over the canonical
        // (query-stripped, upper-cased method) message for the RFC 8032 seed is a
        // fixed value, computed independently with a reference RFC 8032 implementation.
        assert_eq!(
            signature,
            "g7bNgdBcZy/JidGi/K61EwhXUtD6xtqGHVj9/NSxvI+e2wFXChRhRrdDxhtABUFRAry83zmI1Ls5X5cIy24TDA=="
        );
        assert_eq!(
            auth.sign("1700000000000", "GET", "/trade-api/v2/markets")
                .expect("sign"),
            signature
        );

        // The query string is not signed: a signature including it must not verify.
        let sig_bytes = STANDARD.decode(&signature).expect("decode");
        assert_eq!(sig_bytes.len(), 64);
        let sig = ed25519_dalek::Signature::from_slice(&sig_bytes).expect("signature");
        assert!(
            public
                .verify_strict(
                    b"1700000000000GET/trade-api/v2/markets?limit=10&cursor=abc",
                    &sig
                )
                .is_err()
        );
    }

    #[test]
    fn ed25519_build_headers_verify_with_public_key() {
        let seed = [7u8; 32];
        let auth = KalshiAuth::from_pem_str("ed-headers", &ed25519_pkcs8_pem(&seed)).expect("load");
        let public = ed25519_dalek::SigningKey::from_bytes(&seed).verifying_key();

        let headers = auth
            .build_headers("DELETE", "/trade-api/v2/portfolio/orders/abc?x=1")
            .expect("build headers");
        assert_eq!(headers.key, "ed-headers");
        let message = format!(
            "{}DELETE/trade-api/v2/portfolio/orders/abc",
            headers.timestamp_ms
        );
        verify_ed25519(&public, &message, &headers.signature);
    }

    #[test]
    fn ed25519_pem_encoded_by_dalek_round_trips() {
        let signing = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
        let pem = signing.to_pkcs8_pem(LineEnding::LF).expect("encode");
        let auth = KalshiAuth::from_pem_str("k", pem.as_str()).expect("load");
        let sig = auth
            .sign("1", "GET", "/trade-api/v2/markets")
            .expect("sign");
        verify_ed25519(&signing.verifying_key(), "1GET/trade-api/v2/markets", &sig);
    }

    #[test]
    fn rejects_malformed_and_unsupported_keys() {
        // Not PEM at all / empty.
        assert!(KalshiAuth::from_pem_str("k", "").is_err());
        assert!(KalshiAuth::from_pem_str("k", "not a pem").is_err());

        // Valid PEM framing, garbage DER.
        let garbage = pem_from_der("PRIVATE KEY", &[0xde, 0xad, 0xbe, 0xef]);
        assert!(KalshiAuth::from_pem_str("k", &garbage).is_err());

        // Truncated Ed25519 PKCS#8 (seed cut short).
        let mut der = ED25519_PKCS8_PREFIX.to_vec();
        der.extend_from_slice(&rfc8032_seed()[..16]);
        assert!(KalshiAuth::from_pem_str("k", &pem_from_der("PRIVATE KEY", &der)).is_err());

        // Well-formed PKCS#8 for an unsupported algorithm: X25519 (1.3.101.110).
        let mut x25519 = ED25519_PKCS8_PREFIX.to_vec();
        x25519[11] = 0x6e;
        x25519.extend_from_slice(&rfc8032_seed());
        assert!(KalshiAuth::from_pem_str("k", &pem_from_der("PRIVATE KEY", &x25519)).is_err());

        // A public key is not a private key.
        let public_pem = pem_from_der("PUBLIC KEY", &hex_to_bytes(RFC8032_PUBLIC_HEX));
        assert!(KalshiAuth::from_pem_str("k", &public_pem).is_err());
    }

    #[test]
    fn debug_output_does_not_leak_key_material() {
        let seed = rfc8032_seed();
        let auth = KalshiAuth::from_pem_str("dbg", &ed25519_pkcs8_pem(&seed)).expect("load");
        let rendered = format!("{auth:?}");
        assert!(rendered.contains("dbg"));
        assert!(rendered.contains("Ed25519"));
        assert!(!rendered.contains(RFC8032_SEED_HEX));
        assert!(!rendered.contains(&format!("{seed:?}")));

        let rsa_auth = KalshiAuth::from_pem_str("dbg", &rsa_pkcs8_pem()).expect("load");
        assert!(format!("{rsa_auth:?}").contains("RSA-PSS-SHA256"));
    }
}
