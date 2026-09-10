use serde::Deserialize;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Market lifecycle message (type: "market_lifecycle_v2")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleV2 {
    pub market_ticker: String,
    #[serde(default)]
    pub event_type: Option<WsMarketLifecycleEventType>,
    /// Identifier for the exchange shard the market lives on. Per the
    /// AsyncAPI this key exists **only** on `created` events. Added
    /// 2026-07-30; also carried on `multivariate_market_lifecycle` (shares
    /// this type) and `event_lifecycle` messages.
    #[serde(default)]
    pub exchange_index: Option<u32>,
    #[serde(default)]
    pub open_ts: Option<i64>,
    #[serde(default)]
    pub close_ts: Option<i64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub determination_ts: Option<i64>,
    #[serde(default)]
    pub settlement_value: Option<String>,
    #[serde(default)]
    pub settled_ts: Option<i64>,
    #[serde(default)]
    pub is_deactivated: Option<bool>,
    #[serde(default)]
    pub fractional_trading_enabled: Option<bool>,
    #[serde(default)]
    pub price_level_structure: Option<String>,
    /// The market's valid-price grid as `{start, end, step}` bands in fixed-point dollars,
    /// same shape as the REST market object's `price_ranges`. Per the AsyncAPI this key is
    /// only present on events that also carry `price_level_structure` (`created` and
    /// `price_level_structure_updated`). Added 2026-07-02.
    #[serde(default)]
    pub price_ranges: Option<Vec<crate::rest::PriceRange>>,
    /// Determines how `floor_strike` / `cap_strike` are interpreted (e.g.
    /// "between" uses both, "greater" uses `floor_strike` only, "less" uses
    /// `cap_strike` only). Per the AsyncAPI this key exists **only** on
    /// `metadata_updated` events. Added 2026-06-18.
    #[serde(default)]
    pub strike_type: Option<String>,
    /// Top-level updated floor strike. Per the AsyncAPI this key exists **only**
    /// on `metadata_updated` events and is distinct from
    /// `additional_metadata.floor_strike` (which is emitted on market creation).
    #[serde(default)]
    pub floor_strike: Option<f64>,
    /// Top-level updated cap (upper bound) strike. Per the AsyncAPI this key
    /// exists **only** on `metadata_updated` events. Added 2026-06-18.
    #[serde(default)]
    pub cap_strike: Option<f64>,
    /// Top-level updated custom/structured strike. Per the AsyncAPI this key
    /// exists **only** on `metadata_updated` events with a custom or
    /// structured strike type. Added 2026-06-18.
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    /// Top-level updated yes subtitle. Per the AsyncAPI this key exists **only**
    /// on `metadata_updated` events.
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    #[serde(default)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadata>,
    /// Catches any other top-level keys the exchange attaches to a lifecycle
    /// event (e.g. future `metadata_updated` fields beyond floor_strike /
    /// yes_sub_title).
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WsMarketLifecycleEventType {
    Created,
    Activated,
    Deactivated,
    CloseDateUpdated,
    Determined,
    Settled,
    FractionalTradingUpdated,
    PriceLevelStructureUpdated,
    /// Fires when market metadata (name, title, subtitles, etc.) changes. Added 2026-05-11.
    MetadataUpdated,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleAdditionalMetadata {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    #[serde(default)]
    pub no_sub_title: Option<String>,
    #[serde(default)]
    pub rules_primary: Option<String>,
    #[serde(default)]
    pub rules_secondary: Option<String>,
    #[serde(default)]
    pub can_close_early: Option<bool>,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub expected_expiration_ts: Option<i64>,
    #[serde(default)]
    pub strike_type: Option<String>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Event lifecycle message (type: "event_lifecycle")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycle {
    pub event_ticker: String,
    /// Identifier for the exchange shard the event's markets live on. Added
    /// 2026-07-30.
    #[serde(default)]
    pub exchange_index: Option<u32>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub collateral_return_type: Option<String>,
    #[serde(default)]
    pub series_ticker: Option<String>,
    #[serde(default)]
    pub strike_date: Option<i64>,
    #[serde(default)]
    pub strike_period: Option<String>,
    #[serde(default)]
    pub additional_metadata: Option<WsEventLifecycleAdditionalMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleAdditionalMetadata {
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Market lifecycle message (type: "market_lifecycle_v2")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleV2Ref<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default)]
    pub event_type: Option<WsMarketLifecycleEventType>,
    /// Identifier for the exchange shard the market lives on; present only on
    /// `created` events. Added 2026-07-30.
    #[serde(default)]
    pub exchange_index: Option<u32>,
    #[serde(default)]
    pub open_ts: Option<i64>,
    #[serde(default)]
    pub close_ts: Option<i64>,
    #[serde(default, borrow)]
    pub result: Option<Cow<'a, str>>,
    #[serde(default)]
    pub determination_ts: Option<i64>,
    #[serde(default, borrow)]
    pub settlement_value: Option<Cow<'a, str>>,
    #[serde(default)]
    pub settled_ts: Option<i64>,
    #[serde(default)]
    pub is_deactivated: Option<bool>,
    #[serde(default)]
    pub fractional_trading_enabled: Option<bool>,
    #[serde(default, borrow)]
    pub price_level_structure: Option<Cow<'a, str>>,
    /// See [`WsMarketLifecycleV2::price_ranges`]. Added 2026-07-02.
    #[serde(default)]
    pub price_ranges: Option<Vec<crate::rest::PriceRange>>,
    /// Determines how `floor_strike` / `cap_strike` are interpreted; present
    /// only on `metadata_updated` events. Added 2026-06-18.
    #[serde(default, borrow)]
    pub strike_type: Option<Cow<'a, str>>,
    /// Top-level updated floor strike; present only on `metadata_updated` events.
    #[serde(default)]
    pub floor_strike: Option<f64>,
    /// Top-level updated cap (upper bound) strike; present only on
    /// `metadata_updated` events. Added 2026-06-18.
    #[serde(default)]
    pub cap_strike: Option<f64>,
    /// Top-level updated custom/structured strike; present only on
    /// `metadata_updated` events with a custom or structured strike type.
    /// Added 2026-06-18.
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    /// Top-level updated yes subtitle; present only on `metadata_updated` events.
    #[serde(default, borrow)]
    pub yes_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadataRef<'a>>,
    /// Catches any other top-level lifecycle keys not modeled above.
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl<'a> WsMarketLifecycleV2Ref<'a> {
    pub fn into_owned(self) -> WsMarketLifecycleV2 {
        WsMarketLifecycleV2 {
            market_ticker: self.market_ticker.into_owned(),
            event_type: self.event_type,
            exchange_index: self.exchange_index,
            open_ts: self.open_ts,
            close_ts: self.close_ts,
            result: self.result.map(Cow::into_owned),
            determination_ts: self.determination_ts,
            settlement_value: self.settlement_value.map(Cow::into_owned),
            settled_ts: self.settled_ts,
            is_deactivated: self.is_deactivated,
            fractional_trading_enabled: self.fractional_trading_enabled,
            price_level_structure: self.price_level_structure.map(Cow::into_owned),
            price_ranges: self.price_ranges,
            strike_type: self.strike_type.map(Cow::into_owned),
            floor_strike: self.floor_strike,
            cap_strike: self.cap_strike,
            custom_strike: self.custom_strike,
            yes_sub_title: self.yes_sub_title.map(Cow::into_owned),
            additional_metadata: self
                .additional_metadata
                .map(WsMarketLifecycleAdditionalMetadataRef::into_owned),
            extra: self.extra,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleAdditionalMetadataRef<'a> {
    #[serde(default, borrow)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub yes_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub no_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub rules_primary: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub rules_secondary: Option<Cow<'a, str>>,
    #[serde(default)]
    pub can_close_early: Option<bool>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub expected_expiration_ts: Option<i64>,
    #[serde(default, borrow)]
    pub strike_type: Option<Cow<'a, str>>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl<'a> WsMarketLifecycleAdditionalMetadataRef<'a> {
    pub fn into_owned(self) -> WsMarketLifecycleAdditionalMetadata {
        WsMarketLifecycleAdditionalMetadata {
            name: self.name.map(Cow::into_owned),
            title: self.title.map(Cow::into_owned),
            yes_sub_title: self.yes_sub_title.map(Cow::into_owned),
            no_sub_title: self.no_sub_title.map(Cow::into_owned),
            rules_primary: self.rules_primary.map(Cow::into_owned),
            rules_secondary: self.rules_secondary.map(Cow::into_owned),
            can_close_early: self.can_close_early,
            event_ticker: self.event_ticker.map(Cow::into_owned),
            expected_expiration_ts: self.expected_expiration_ts,
            strike_type: self.strike_type.map(Cow::into_owned),
            floor_strike: self.floor_strike,
            cap_strike: self.cap_strike,
            custom_strike: self.custom_strike,
            extra: self.extra,
        }
    }
}

/// Event lifecycle message (type: "event_lifecycle")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    /// Identifier for the exchange shard the event's markets live on. Added
    /// 2026-07-30.
    #[serde(default)]
    pub exchange_index: Option<u32>,
    #[serde(default, borrow)]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub subtitle: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub collateral_return_type: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub series_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub strike_date: Option<i64>,
    #[serde(default, borrow)]
    pub strike_period: Option<Cow<'a, str>>,
    #[serde(default)]
    pub additional_metadata: Option<WsEventLifecycleAdditionalMetadataRef>,
}

impl<'a> WsEventLifecycleRef<'a> {
    pub fn into_owned(self) -> WsEventLifecycle {
        WsEventLifecycle {
            event_ticker: self.event_ticker.into_owned(),
            exchange_index: self.exchange_index,
            title: self.title.map(Cow::into_owned),
            subtitle: self.subtitle.map(Cow::into_owned),
            collateral_return_type: self.collateral_return_type.map(Cow::into_owned),
            series_ticker: self.series_ticker.map(Cow::into_owned),
            strike_date: self.strike_date,
            strike_period: self.strike_period.map(Cow::into_owned),
            additional_metadata: self
                .additional_metadata
                .map(WsEventLifecycleAdditionalMetadataRef::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleAdditionalMetadataRef {
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl WsEventLifecycleAdditionalMetadataRef {
    pub fn into_owned(self) -> WsEventLifecycleAdditionalMetadata {
        WsEventLifecycleAdditionalMetadata {
            custom_strike: self.custom_strike,
            extra: self.extra,
        }
    }
}

/// Event fee update message (type: "event_fee_update").
///
/// Delivered on the `market_lifecycle_v2` channel when an event's fee override
/// is set or cleared. Both override fields are `null` when the override is
/// cleared. `fee_type_override` is kept as a raw string (values include
/// `quadratic`, `quadratic_with_maker_fees`, `flat`) so unknown variants are
/// preserved losslessly for fee math.
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventFeeUpdate {
    pub event_ticker: String,
    #[serde(default)]
    pub fee_type_override: Option<String>,
    #[serde(default)]
    pub fee_multiplier_override: Option<f64>,
}

/// Event fee update message (type: "event_fee_update").
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventFeeUpdateRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub fee_type_override: Option<Cow<'a, str>>,
    #[serde(default)]
    pub fee_multiplier_override: Option<f64>,
}

impl<'a> WsEventFeeUpdateRef<'a> {
    pub fn into_owned(self) -> WsEventFeeUpdate {
        WsEventFeeUpdate {
            event_ticker: self.event_ticker.into_owned(),
            fee_type_override: self.fee_type_override.map(Cow::into_owned),
            fee_multiplier_override: self.fee_multiplier_override,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Per the AsyncAPI, `metadata_updated` carries the updated values
    /// (`floor_strike`, `yes_sub_title`) at the top level of the payload, not
    /// nested under `additional_metadata`. They must not be silently dropped.
    #[test]
    fn metadata_updated_surfaces_top_level_fields() {
        let json = r#"{
            "event_type": "metadata_updated",
            "market_ticker": "KXHIGHNY-24JAN01-T60",
            "floor_strike": 60.5,
            "yes_sub_title": "Above 60°F",
            "some_future_key": "kept"
        }"#;

        let owned: WsMarketLifecycleV2 = serde_json::from_str(json).unwrap();
        assert_eq!(
            owned.event_type,
            Some(WsMarketLifecycleEventType::MetadataUpdated)
        );
        assert_eq!(owned.floor_strike, Some(60.5));
        assert_eq!(owned.yes_sub_title.as_deref(), Some("Above 60°F"));
        assert_eq!(
            owned.extra.get("some_future_key").and_then(Value::as_str),
            Some("kept")
        );

        // Borrowed path must round-trip to the same surfaced values.
        let borrowed: WsMarketLifecycleV2Ref = serde_json::from_str(json).unwrap();
        let round_tripped = borrowed.into_owned();
        assert_eq!(round_tripped.floor_strike, Some(60.5));
        assert_eq!(round_tripped.yes_sub_title.as_deref(), Some("Above 60°F"));
        assert_eq!(
            round_tripped
                .extra
                .get("some_future_key")
                .and_then(Value::as_str),
            Some("kept")
        );
    }

    /// Per the 2026-06-18 changelog, `metadata_updated` events now also carry
    /// `strike_type` / `cap_strike` / `custom_strike` at the top level,
    /// alongside the pre-existing `floor_strike` / `yes_sub_title`.
    #[test]
    fn metadata_updated_surfaces_new_top_level_strike_fields() {
        let json = r#"{
            "event_type": "metadata_updated",
            "market_ticker": "KXHIGHNY-24JAN01-T60",
            "strike_type": "between",
            "floor_strike": 60.5,
            "cap_strike": 70.5,
            "custom_strike": {"key": "value"},
            "yes_sub_title": "Above 60°F"
        }"#;

        let owned: WsMarketLifecycleV2 = serde_json::from_str(json).unwrap();
        assert_eq!(owned.strike_type.as_deref(), Some("between"));
        assert_eq!(owned.floor_strike, Some(60.5));
        assert_eq!(owned.cap_strike, Some(70.5));
        assert_eq!(
            owned.custom_strike.as_ref().and_then(|m| m.get("key")),
            Some(&"value".to_string())
        );

        let borrowed: WsMarketLifecycleV2Ref = serde_json::from_str(json).unwrap();
        let round_tripped = borrowed.into_owned();
        assert_eq!(round_tripped.strike_type.as_deref(), Some("between"));
        assert_eq!(round_tripped.cap_strike, Some(70.5));
        assert_eq!(
            round_tripped
                .custom_strike
                .as_ref()
                .and_then(|m| m.get("key")),
            Some(&"value".to_string())
        );
    }

    /// Per the 2026-07-30 changelog, `created` events on `market_lifecycle_v2`
    /// (and `multivariate_market_lifecycle`, which shares this type) and
    /// `event_lifecycle` messages now carry `exchange_index`.
    #[test]
    fn created_event_surfaces_exchange_index() {
        let json = r#"{
            "event_type": "created",
            "market_ticker": "KXHIGHNY-24JAN01-T60",
            "exchange_index": 2
        }"#;

        let owned: WsMarketLifecycleV2 = serde_json::from_str(json).unwrap();
        assert_eq!(owned.exchange_index, Some(2));

        let borrowed: WsMarketLifecycleV2Ref = serde_json::from_str(json).unwrap();
        assert_eq!(borrowed.into_owned().exchange_index, Some(2));

        let event_json = r#"{
            "event_ticker": "KXHIGHNY-24JAN01",
            "exchange_index": 2,
            "title": "Title",
            "subtitle": "Subtitle",
            "collateral_return_type": "MECNET",
            "series_ticker": "KXHIGHNY"
        }"#;
        let owned_event: WsEventLifecycle = serde_json::from_str(event_json).unwrap();
        assert_eq!(owned_event.exchange_index, Some(2));
        assert_eq!(
            owned_event.collateral_return_type.as_deref(),
            Some("MECNET")
        );
        assert_eq!(owned_event.series_ticker.as_deref(), Some("KXHIGHNY"));

        let borrowed_event: WsEventLifecycleRef = serde_json::from_str(event_json).unwrap();
        assert_eq!(borrowed_event.into_owned().exchange_index, Some(2));
    }

    /// Per the 2026-07-02 changelog, `created` and `price_level_structure_updated`
    /// events carry a `price_ranges` array alongside `price_level_structure`.
    #[test]
    fn price_level_structure_updated_surfaces_price_ranges() {
        let json = r#"{
            "event_type": "price_level_structure_updated",
            "market_ticker": "INXD-23SEP14-B4487",
            "price_level_structure": "deci_cent",
            "price_ranges": [
                {"start": "0.0000", "end": "1.0000", "step": "0.0010"}
            ]
        }"#;

        let owned: WsMarketLifecycleV2 = serde_json::from_str(json).unwrap();
        assert_eq!(owned.price_level_structure.as_deref(), Some("deci_cent"));
        let ranges = owned.price_ranges.expect("price_ranges must be present");
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start, "0.0000");
        assert_eq!(ranges[0].end, "1.0000");
        assert_eq!(ranges[0].step, "0.0010");

        let borrowed: WsMarketLifecycleV2Ref = serde_json::from_str(json).unwrap();
        let round_tripped = borrowed.into_owned();
        let round_tripped_ranges = round_tripped
            .price_ranges
            .expect("price_ranges must round-trip");
        assert_eq!(round_tripped_ranges[0].step, "0.0010");
    }

    /// Fields introduced after a message type shipped must not become
    /// mandatory: older captured payloads lacking `exchange_index` still
    /// parse.
    #[test]
    fn exchange_index_defaults_when_absent() {
        let json = r#"{"market_ticker": "TEST", "event_type": "activated"}"#;
        let owned: WsMarketLifecycleV2 = serde_json::from_str(json).unwrap();
        assert!(owned.exchange_index.is_none());
    }
}
