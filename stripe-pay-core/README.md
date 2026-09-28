# stripe-pay-core

Shared Stripe domain types for the `stripe-pay` SDK family.

## Overview

`stripe-pay-core` holds the value types every other crate in the family agrees on. It performs no I/O, so it runs anywhere — host binaries, servers, and `wasm32` builds alike.

Amounts are stored as integers in the currency's minor unit. No float ever touches a money value.

## Domain

- **`Money` / `Currency`** — minor-unit amounts, ISO codes, and per-currency decimal places. Zero-decimal currencies such as JPY are handled without a decimal point.
- **`StripeId` / `StripeIdKind`** — prefixed identifiers (`pi_`, `ch_`, `re_`) with kind validation on construction.
- **`PaymentIntent`** — the seven-state machine, including `is_terminal()` and `can_transition_to()`.
- **`Charge` / `Refund`** — status, reason, and amount modelling.
- **`CardBrand` / `CardDetails`** — PCI-safe card data. Only the brand, last four digits, and expiry are modelled; a full PAN or CVC is never stored.
- **`FormParams` / `FormField`** — Stripe form encoding, including RFC 3986 percent-encoding of bracketed `metadata` keys.

## Usage

```rust
use stripe_pay_core::{Currency, Money};

let amount: Money = Money::from_major(1999, Currency::Usd);
assert_eq!(amount.get_amount(), 199_900);
```

## Licence

MIT. See [LICENSE](LICENSE).
