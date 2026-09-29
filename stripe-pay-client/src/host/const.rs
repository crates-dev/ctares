/// Name of the JavaScript global Stripe publishes.
pub const STRIPE_GLOBAL: &str = "Stripe";

/// Publish call the payment element mount returns.
pub const ELEMENTS_CREATE: &str = "elements";

/// Client secret key the element needs to render.
pub const CLIENT_SECRET_FIELD: &str = "clientSecret";

/// Diagnostic when a configuration carries no client secret.
pub const MESSAGE_MISSING_CLIENT_SECRET: &str = "payment element needs a client secret";

/// Diagnostic when Stripe.js is not loaded on the page.
pub const MESSAGE_STRIPE_JS_UNAVAILABLE: &str = "Stripe.js is not available on this page";

/// Diagnostic when the page has no element to mount into.
pub const MESSAGE_MISSING_MOUNT_TARGET: &str = "payment element has no mount target";

/// Locale the element uses when the host page does not pick one.
pub const DEFAULT_LOCALE: &str = "en";

/// Locale the element renders its labels in.
pub const LOCALE_FIELD: &str = "locale";
