# stripe-pay-client

Browser and WebAssembly client for the `stripe-pay` SDK family.

## Overview

`stripe-pay-client` builds the configuration a checkout page hands to the Stripe payment element. It carries no transport of its own: the browser binds the configuration to Stripe.js at mount time, so the crate stays free of I/O and can be unit tested off-wasm. It depends on [`stripe-pay-core`](../stripe-pay-core) for the shared domain types.

## Payment element configuration

```rust
use stripe_pay_client::{ElementConfig, ElementKind, build_element_options};

let config = ElementConfig::new(client_secret).with_kind(ElementKind::Payment);
let options = build_element_options(&config)?;
```

`ElementConfig` keeps the client secret and the locale, and `effective_locale()` falls back to the crate default when the host page does not pick one.

## Mount preflight

`preflight` checks a page before it mounts and reports the first thing that is missing, in the order a page author would fix them: no client secret, then no mount target, then Stripe.js absent from the page.

```rust
use stripe_pay_client::preflight;

preflight(&config, has_mount_target, has_stripe_js)?;
```

## Target

Builds for `wasm32-unknown-unknown`. The types are transport-free, so the same logic is testable on the host.

## Licence

MIT. See [LICENSE](LICENSE).
