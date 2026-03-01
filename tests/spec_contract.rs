use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct SpecInfo {
    version: String,
}

#[derive(Debug, Deserialize)]
struct OpenApiSpec {
    info: SpecInfo,
    paths: BTreeMap<String, BTreeMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
struct OpenApiManifest {
    info_version: String,
    operation_count: usize,
    operations: Vec<OpenApiManifestOperation>,
}

#[derive(Debug, Deserialize)]
struct OpenApiManifestOperation {
    operation_id: String,
    http_method: String,
    path: String,
    client_method: String,
}

#[derive(Debug, Deserialize)]
struct AsyncApiSpec {
    info: SpecInfo,
    channels: BTreeMap<String, Value>,
    operations: BTreeMap<String, Value>,
    components: AsyncComponents,
}

#[derive(Debug, Deserialize)]
struct AsyncComponents {
    messages: BTreeMap<String, Value>,
    schemas: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct AsyncApiManifest {
    info_version: String,
    channel_count: usize,
    channels: Vec<String>,
    subscribe_channel_enum: Vec<String>,
    command_message_count: usize,
    command_messages: Vec<String>,
    operation_count: usize,
    operation_names: Vec<String>,
    required_v2_types: Vec<String>,
    required_v2_client_methods: Vec<String>,
    required_parser_tokens: Vec<String>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn openapi_manifest_has_full_operation_coverage() {
    let root = repo_root();

    let openapi: OpenApiSpec = serde_yaml::from_str(
        &fs::read_to_string(root.join("docs/specs/kalshi/openapi.yaml")).expect("read openapi"),
    )
    .expect("parse openapi yaml");

    let manifest: OpenApiManifest = serde_json::from_str(
        &fs::read_to_string(root.join("docs/specs/kalshi/openapi-manifest.json"))
            .expect("read openapi manifest"),
    )
    .expect("parse openapi manifest");

    assert_eq!(openapi.info.version, manifest.info_version);

    let mut spec_ops = BTreeSet::new();
    for (path, item) in &openapi.paths {
        for (method, operation) in item {
            let method_l = method.to_ascii_lowercase();
            if !matches!(
                method_l.as_str(),
                "get" | "post" | "put" | "patch" | "delete"
            ) {
                continue;
            }
            let operation_id = operation
                .get("operationId")
                .and_then(Value::as_str)
                .expect("operationId in openapi")
                .to_string();
            spec_ops.insert((operation_id, method_l.to_ascii_uppercase(), path.clone()));
        }
    }

    let manifest_ops: BTreeSet<(String, String, String)> = manifest
        .operations
        .iter()
        .map(|op| {
            (
                op.operation_id.clone(),
                op.http_method.clone(),
                op.path.clone(),
            )
        })
        .collect();

    assert_eq!(spec_ops.len(), 77, "snapshot should contain 77 operations");
    assert_eq!(manifest.operation_count, spec_ops.len());
    assert_eq!(
        manifest_ops, spec_ops,
        "manifest must cover all path+method ops"
    );

    let rest_client_src = fs::read_to_string(root.join("src/rest/client.rs")).expect("read client");
    for operation in &manifest.operations {
        let needle = format!("pub async fn {}(", operation.client_method);
        assert!(
            rest_client_src.contains(&needle),
            "missing client method mapping for {}: {}",
            operation.operation_id,
            operation.client_method
        );
    }
}

#[test]
fn asyncapi_manifest_matches_snapshot_and_v2_surface() {
    let root = repo_root();

    let asyncapi: AsyncApiSpec = serde_yaml::from_str(
        &fs::read_to_string(root.join("docs/specs/kalshi/asyncapi.yaml")).expect("read asyncapi"),
    )
    .expect("parse asyncapi yaml");

    let manifest: AsyncApiManifest = serde_json::from_str(
        &fs::read_to_string(root.join("docs/specs/kalshi/asyncapi-manifest.json"))
            .expect("read asyncapi manifest"),
    )
    .expect("parse asyncapi manifest");

    assert_eq!(asyncapi.info.version, manifest.info_version);

    let spec_channels: Vec<String> = asyncapi.channels.keys().cloned().collect();
    assert_eq!(spec_channels.len(), manifest.channel_count);
    assert_eq!(spec_channels, manifest.channels);

    let mut subscribe_channels = asyncapi
        .components
        .schemas
        .get("subscribeCommandPayload")
        .and_then(|schema| schema.get("properties"))
        .and_then(|v| v.get("params"))
        .and_then(|v| v.get("properties"))
        .and_then(|v| v.get("channels"))
        .and_then(|v| v.get("items"))
        .and_then(|v| v.get("enum"))
        .and_then(Value::as_array)
        .expect("subscribe channel enum")
        .iter()
        .map(|item| item.as_str().expect("enum string").to_string())
        .collect::<Vec<_>>();
    subscribe_channels.sort();
    assert_eq!(subscribe_channels, manifest.subscribe_channel_enum);

    let mut command_messages = asyncapi
        .components
        .messages
        .keys()
        .filter(|name| name.ends_with("Command"))
        .cloned()
        .collect::<Vec<_>>();
    command_messages.sort();
    assert_eq!(command_messages.len(), manifest.command_message_count);
    assert_eq!(command_messages, manifest.command_messages);

    let mut operation_names = asyncapi.operations.keys().cloned().collect::<Vec<_>>();
    operation_names.sort();
    assert_eq!(operation_names.len(), manifest.operation_count);
    assert_eq!(operation_names, manifest.operation_names);

    let ws_types_src = fs::read_to_string(root.join("src/ws/types.rs")).expect("read ws types");
    let ws_client_src = fs::read_to_string(root.join("src/ws/client.rs")).expect("read ws client");

    for channel in &manifest.subscribe_channel_enum {
        assert!(
            ws_types_src.contains(&format!("\"{channel}\"")),
            "ws v2 channel missing from source: {channel}"
        );
    }

    for type_name in &manifest.required_v2_types {
        let type_alias = format!("type {type_name}");
        let struct_decl = format!("struct {type_name}");
        let enum_decl = format!("enum {type_name}");
        assert!(
            ws_types_src.contains(&type_alias)
                || ws_types_src.contains(&struct_decl)
                || ws_types_src.contains(&enum_decl),
            "missing required ws v2 type: {type_name}"
        );
    }

    for method_name in &manifest.required_v2_client_methods {
        assert!(
            ws_client_src.contains(&format!("fn {method_name}(")),
            "missing required ws v2 client method: {method_name}"
        );
    }

    for token in &manifest.required_parser_tokens {
        assert!(
            ws_types_src.contains(token),
            "missing parser token for asyncapi behavior: {token}"
        );
    }
}
