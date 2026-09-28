/// A unit of currency, using Stripe's own naming for each variant.
///
/// The variant list is intentionally small: this crate covers the
/// currencies the SDK's examples and tests exercise. Adding a currency
/// means adding a variant here and a row in every `match` below, so the
/// compiler surfaces each site that needs the new zero- or two-decimal
/// decision rather than silently defaulting.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Currency {
    /// United States dollar, two decimal places.
    #[default]
    Usd,
    /// Euro, two decimal places.
    Eur,
    /// British pound, two decimal places.
    Gbp,
    /// Japanese yen, zero decimal places.
    Jpy,
    /// Swiss franc, two decimal places.
    Chf,
}
