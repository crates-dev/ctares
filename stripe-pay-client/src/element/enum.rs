/// A mount target for the Stripe payment element.
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
pub enum ElementKind {
    /// The full card form, ready to confirm.
    #[default]
    Payment,
    /// Card details only, for a custom checkout layout.
    Card,
    /// The saved-card picker for a returning customer.
    Setup,
}
