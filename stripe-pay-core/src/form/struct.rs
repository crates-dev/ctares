/// A form field destined for Stripe's `application/x-www-form-urlencoded`
/// request body.
///
/// Stripe's REST API takes nested parameters as bracketed keys rather
/// than JSON, so `metadata[order_id]=42` is the wire form. A
/// `FormField` is one such key and value pair; the builder below
/// assembles the whole body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormField {
    /// The bracketed parameter name, such as `amount` or
    /// `metadata[order_id]`.
    pub(super) key: String,
    /// The field's value, already stringified.
    pub(super) value: String,
}

/// An ordered collection of Stripe form parameters.
///
/// Stripe rejects duplicate keys inside one request, so the builder
/// overwrites a key rather than pushing a second entry, keeping the
/// encoding deterministic: the same builder always produces the same
/// body regardless of the order fields were added.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormParams {
    /// The fields in insertion order.
    pub(super) fields: Vec<FormField>,
}
