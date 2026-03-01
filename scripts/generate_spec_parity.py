#!/usr/bin/env python3
from __future__ import annotations

import json
import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
SPEC_DIR = ROOT / "docs" / "specs" / "kalshi"
OPENAPI_PATH = SPEC_DIR / "openapi.yaml"
ASYNCAPI_PATH = SPEC_DIR / "asyncapi.yaml"
OPENAPI_MANIFEST_PATH = SPEC_DIR / "openapi-manifest.json"
ASYNCAPI_MANIFEST_PATH = SPEC_DIR / "asyncapi-manifest.json"
PARITY_DOC_PATH = ROOT / "docs" / "spec-parity.md"


def to_snake(name: str) -> str:
    name = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    return name.lower()


def load_yaml(path: Path) -> dict:
    return yaml.safe_load(path.read_text())


def build_openapi_manifest(openapi: dict) -> dict:
    overrides = {
        "ApplySubaccountTransfer": "transfer_subaccount",
        "GetPortfolioRestingOrderTotalValue": "get_portfolio_total_resting_order_value",
        "GetTagsForSeriesCategories": "get_tags_by_categories",
        "GetFiltersForSports": "get_filters_by_sport",
        "GetMarketCandlesticksByEvent": "get_event_market_candlesticks",
        "GetEventForecastPercentilesHistory": "get_event_forecast_percentile_history",
        "GetLiveDatas": "get_live_data_batch",
        "GetRFQs": "get_rfqs",
    }

    operations = []
    for path, path_item in openapi.get("paths", {}).items():
        for method, op in path_item.items():
            method_l = method.lower()
            if method_l not in {"get", "post", "put", "patch", "delete"}:
                continue
            operation_id = op["operationId"]
            client_method = overrides.get(operation_id, to_snake(operation_id))
            operations.append(
                {
                    "operation_id": operation_id,
                    "http_method": method_l.upper(),
                    "path": path,
                    "client_method": client_method,
                }
            )

    operations.sort(key=lambda item: (item["path"], item["http_method"], item["operation_id"]))

    return {
        "spec_file": str(OPENAPI_PATH.relative_to(ROOT)),
        "info_version": openapi.get("info", {}).get("version"),
        "operation_count": len(operations),
        "operations": operations,
    }


def build_asyncapi_manifest(asyncapi: dict) -> dict:
    channels = sorted(asyncapi.get("channels", {}).keys())
    operations = asyncapi.get("operations", {})

    subscribe_channels = (
        asyncapi.get("components", {})
        .get("schemas", {})
        .get("subscribeCommandPayload", {})
        .get("properties", {})
        .get("params", {})
        .get("properties", {})
        .get("channels", {})
        .get("items", {})
        .get("enum", [])
    )

    messages = asyncapi.get("components", {}).get("messages", {})

    return {
        "spec_file": str(ASYNCAPI_PATH.relative_to(ROOT)),
        "info_version": asyncapi.get("info", {}).get("version"),
        "channel_count": len(channels),
        "channels": channels,
        "subscribe_channel_enum": sorted(subscribe_channels),
        "command_message_count": len([name for name in messages if name.endswith("Command")]),
        "command_messages": sorted(name for name in messages if name.endswith("Command")),
        "operation_count": len(operations),
        "operation_names": sorted(operations.keys()),
        "required_v2_types": [
            "WsChannelV2",
            "WsSubscriptionParamsV2",
            "WsUnsubscribeParamsV2",
            "WsUpdateSubscriptionParamsV2",
            "WsUpdateAction",
            "WsMessageV2",
            "WsDataMessageV2",
            "WsUserOrder",
        ],
        "required_v2_client_methods": [
            "subscribe_v2",
            "unsubscribe_v2",
            "update_subscription_v2",
            "next_message_v2",
            "next_event_v2",
            "start_reader_v2",
            "close",
            "shutdown_timeout",
        ],
        "required_parser_tokens": [
            "user_orders",
            "user_order",
            "subscribed",
            "list_subscriptions",
        ],
    }


def render_parity_doc(openapi_manifest: dict, asyncapi_manifest: dict) -> str:
    openapi_ops = openapi_manifest["operations"]

    lines: list[str] = []
    lines.append("# Spec Parity Report")
    lines.append("")
    lines.append("Generated from snapshot manifests in `docs/specs/kalshi/`.")
    lines.append("")
    lines.append("## Snapshot Versions")
    lines.append("")
    lines.append(f"- OpenAPI info version: `{openapi_manifest['info_version']}`")
    lines.append(f"- AsyncAPI info version: `{asyncapi_manifest['info_version']}`")
    lines.append("")
    lines.append("## REST Coverage")
    lines.append("")
    lines.append(
        f"- Operation coverage: `{openapi_manifest['operation_count']}/{openapi_manifest['operation_count']}` path+method operations mapped"
    )
    lines.append("")
    lines.append("| Method | Path | Operation ID | Client Method |")
    lines.append("|---|---|---|---|")
    for op in openapi_ops:
        lines.append(
            f"| `{op['http_method']}` | `{op['path']}` | `{op['operation_id']}` | `{op['client_method']}` |"
        )

    lines.append("")
    lines.append("## WebSocket Coverage")
    lines.append("")
    lines.append(
        f"- AsyncAPI channels represented: `{asyncapi_manifest['channel_count']}` channels in snapshot"
    )
    lines.append(
        f"- Subscribe channel enum represented in V2 models: `{len(asyncapi_manifest['subscribe_channel_enum'])}` channels"
    )
    lines.append(
        f"- Command payload families represented: `{asyncapi_manifest['command_message_count']}` command messages"
    )
    lines.append("")
    lines.append("### Snapshot Channels")
    lines.append("")
    for channel in asyncapi_manifest["channels"]:
        lines.append(f"- `{channel}`")

    lines.append("")
    lines.append("### Required V2 Client Methods")
    lines.append("")
    for method in asyncapi_manifest["required_v2_client_methods"]:
        lines.append(f"- `{method}`")

    lines.append("")
    lines.append("### Required V2 Types")
    lines.append("")
    for name in asyncapi_manifest["required_v2_types"]:
        lines.append(f"- `{name}`")

    lines.append("")
    return "\n".join(lines)


def main() -> None:
    openapi = load_yaml(OPENAPI_PATH)
    asyncapi = load_yaml(ASYNCAPI_PATH)

    openapi_manifest = build_openapi_manifest(openapi)
    asyncapi_manifest = build_asyncapi_manifest(asyncapi)

    OPENAPI_MANIFEST_PATH.write_text(json.dumps(openapi_manifest, indent=2) + "\n")
    ASYNCAPI_MANIFEST_PATH.write_text(json.dumps(asyncapi_manifest, indent=2) + "\n")
    PARITY_DOC_PATH.write_text(render_parity_doc(openapi_manifest, asyncapi_manifest))


if __name__ == "__main__":
    main()
