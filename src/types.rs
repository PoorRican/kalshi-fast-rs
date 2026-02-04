use serde::{Serialize, Serializer};
use std::fmt;

/// --- Event Status ---

#[derive(Debug, Clone, Copy)]
pub enum EventStatus {
    Open,
    Closed,
    Settled,
}

impl EventStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            EventStatus::Open => "open",
            EventStatus::Closed => "closed",
            EventStatus::Settled => "settled",
        }
    }
}

impl fmt::Display for EventStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for EventStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Market Status ---

#[derive(Debug, Clone, Copy)]
pub enum MarketStatus {
    Unopened,
    Open,
    Paused,
    Closed,
    Settled,
}

impl MarketStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MarketStatus::Unopened => "unopened",
            MarketStatus::Open => "open",
            MarketStatus::Paused => "paused",
            MarketStatus::Closed => "closed",
            MarketStatus::Settled => "settled",
        }
    }
}

impl fmt::Display for MarketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for MarketStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- MVE Filter ---

#[derive(Debug, Clone, Copy)]
pub enum MveFilter {
    Only,
    Exclude,
}

impl MveFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            MveFilter::Only => "only",
            MveFilter::Exclude => "exclude",
        }
    }
}

impl fmt::Display for MveFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for MveFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Position Count Filter ---

#[derive(Debug, Clone, Copy)]
pub enum PositionCountFilter {
    Position,
    TotalTraded,
}

impl PositionCountFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            PositionCountFilter::Position => "position",
            PositionCountFilter::TotalTraded => "total_traded",
        }
    }
}

impl fmt::Display for PositionCountFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// --- Order Status ---

#[derive(Debug, Clone, Copy)]
pub enum OrderStatus {
    Resting,
    Canceled,
    Executed,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderStatus::Resting => "resting",
            OrderStatus::Canceled => "canceled",
            OrderStatus::Executed => "executed",
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for OrderStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Yes/No (Side) ---

#[derive(Debug, Clone, Copy)]
pub enum YesNo {
    Yes,
    No,
}

impl Default for YesNo {
    fn default() -> Self {
        YesNo::Yes
    }
}

impl YesNo {
    pub fn as_str(self) -> &'static str {
        match self {
            YesNo::Yes => "yes",
            YesNo::No => "no",
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for YesNo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Buy/Sell (Action) ---

#[derive(Debug, Clone, Copy)]
pub enum BuySell {
    Buy,
    Sell,
}

impl Default for BuySell {
    fn default() -> Self {
        BuySell::Buy
    }
}

impl BuySell {
    pub fn as_str(self) -> &'static str {
        match self {
            BuySell::Buy => "buy",
            BuySell::Sell => "sell",
        }
    }
}

impl fmt::Display for BuySell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for BuySell {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Order Type ---

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    Limit,
    Market,
}

impl OrderType {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for OrderType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Time In Force ---

#[derive(Debug, Clone, Copy)]
pub enum TimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

impl TimeInForce {
    pub fn as_str(self) -> &'static str {
        match self {
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::GoodTillCanceled => "good_till_canceled",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for TimeInForce {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// --- Self Trade Prevention Type ---

#[derive(Debug, Clone, Copy)]
pub enum SelfTradePreventionType {
    TakerAtCross,
    Maker,
}

impl SelfTradePreventionType {
    pub fn as_str(self) -> &'static str {
        match self {
            SelfTradePreventionType::TakerAtCross => "taker_at_cross",
            SelfTradePreventionType::Maker => "maker",
        }
    }
}

impl fmt::Display for SelfTradePreventionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for SelfTradePreventionType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
