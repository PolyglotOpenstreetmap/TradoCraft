# 0002 — Decimal money and a pure, server-side pricing engine

**Status:** Accepted

**Decision.**
* Amounts are `NUMERIC` in PostgreSQL, `rust_decimal::Decimal` in Rust and strings in JSON.
  Never floating point.
* All price calculations live in `crates/pricing`, a crate without I/O. Clients never calculate:
  the app calls `POST /api/v1/pricing/preview` while editing and shows the result.
* Rules (tested in `crates/pricing`):
  * list price = the customer's zone price with the latest `valid_from` ≤ quote date;
  * discounts cascade: customer discount, then line discount;
  * a line total is rounded once to 2 decimals (half away from zero) from the exact value;
  * VAT is computed per VAT rate on the summed line totals.
* Prices are never updated: a price change is a new dated row. Documents copy the prices they used.

**Consequences.** Every screen, PDF and future invoice shows the same numbers. Rule changes are
made, reviewed and tested in one place.
