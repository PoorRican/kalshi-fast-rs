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
    /// Present on `metadata_updated` events when the floor strike changed.
    #[serde(default)]
    pub floor_strike: Option<f64>,
    /// Present on `metadata_updated` events when the yes subtitle changed.
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    #[serde(default)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadata>,
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
    /// Present on `metadata_updated` events when the floor strike changed.
    #[serde(default)]
    pub floor_strike: Option<f64>,
    /// Present on `metadata_updated` events when the yes subtitle changed.
    #[serde(default, borrow)]
    pub yes_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadataRef<'a>>,
}

impl<'a> WsMarketLifecycleV2Ref<'a> {
    pub fn into_owned(self) -> WsMarketLifecycleV2 {
        WsMarketLifecycleV2 {
            market_ticker: self.market_ticker.into_owned(),
            event_type: self.event_type,
            open_ts: self.open_ts,
            close_ts: self.close_ts,
            result: self.result.map(Cow::into_owned),
            determination_ts: self.determination_ts,
            settlement_value: self.settlement_value.map(Cow::into_owned),
            settled_ts: self.settled_ts,
            is_deactivated: self.is_deactivated,
            fractional_trading_enabled: self.fractional_trading_enabled,
            price_level_structure: self.price_level_structure.map(Cow::into_owned),
            floor_strike: self.floor_strike,
            yes_sub_title: self.yes_sub_title.map(Cow::into_owned),
            additional_metadata: self
                .additional_metadata
                .map(WsMarketLifecycleAdditionalMetadataRef::into_owned),
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
