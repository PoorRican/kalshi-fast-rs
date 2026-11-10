use std::borrow::Cow;

/// Borrowed fixed-point dollar string (e.g. "0.5600").
pub type FixedPointDollarsRef<'a> = Cow<'a, str>;

/// Borrowed fixed-point contract count string (e.g. "10.00").
pub type FixedPointCountRef<'a> = Cow<'a, str>;

mod ticker;
pub use ticker::*;

mod trade;
pub use trade::*;

mod orderbook;
pub use orderbook::*;

mod fill;
pub use fill::*;

mod lifecycle;
pub use lifecycle::*;

mod positions;
pub use positions::*;

mod order_groups;
pub use order_groups::*;

mod user_orders;
pub use user_orders::*;

mod communications;
pub use communications::*;

mod cfbenchmarks;
pub use cfbenchmarks::*;

mod feeds;
pub use feeds::*;

mod batches;
pub use batches::*;

mod ebattle;
pub use ebattle::*;

// Serde's blanket Cow deserializer allocates inside containers. The wrapper
// preserves borrowing for unescaped strings and owns escaped strings.
pub(super) fn deserialize_borrowed_strings<'de, D>(
    deserializer: D,
) -> Result<Vec<Cow<'de, str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct BorrowedString<'a>(#[serde(borrow)] Cow<'a, str>);

    struct StringsVisitor;
    impl<'de> serde::de::Visitor<'de> for StringsVisitor {
        type Value = Vec<Cow<'de, str>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("an array of strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(BorrowedString(value)) = seq.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    deserializer.deserialize_seq(StringsVisitor)
}

fn deserialize_borrowed_option<'de, D>(deserializer: D) -> Result<Option<Cow<'de, str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct BorrowedString<'a>(#[serde(borrow)] Cow<'a, str>);

    Option::<BorrowedString<'de>>::deserialize(deserializer).map(|value| value.map(|value| value.0))
}

// Unlike Serde's default Option handling, required nullable fields must be
// present even when their value is null.
fn deserialize_required_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    serde::Deserialize::deserialize(deserializer)
}
