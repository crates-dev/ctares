# stripe-pay-server

Server-side transport for the `stripe-pay` SDK family.

## Overview

`stripe-pay-server` verifies that a webhook payload really came from Stripe, and builds the form bodies the Stripe API expects. It depends on [`stripe-pay-core`](../stripe-pay-core) for the shared domain types.

## Webhook verification

Stripe signs `"{timestamp}.{payload}"` with HMAC-SHA256 and delivers the hex digest in the `Stripe-Signature` header alongside the timestamp. `stripe-pay-server` parses that header, enforces a timestamp tolerance, and compares digests in **constant time** so a forged request cannot learn how many leading bytes matched by timing the response.

```rust
use stripe_pay_server::{DEFAULT_TOLERANCE_SECONDS, verify_webhook};

let event = verify_webhook(header, payload, secret, now, DEFAULT_TOLERANCE_SECONDS)?;
```

Rejections are typed: `MissingSignature`, `TimestampOutOfTolerance`, `EmptySecret`, and `SignatureMismatch`.

## Request bodies

`encode_create_payment_intent`, `encode_create_refund`, `encode_confirm_payment_intent`, and `encode_create_customer` produce percent-encoded bodies matching Stripe's `application/x-www-form-urlencoded` contract.

## Usage

```rust
use stripe_pay_server::{WebhookEvent, DEFAULT_TOLERANCE_SECONDS};

let digest = WebhookEvent::new(payload, timestamp, String::new())
    .compute_signature(secret)?;
```

## Licence

MIT. See [LICENSE](LICENSE).
