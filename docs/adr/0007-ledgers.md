# 0007 — Ledgers with projections for stock and money

**Status:** Accepted for stock, as detailed in [ADR 0019](0019-inventory-stock-ledger.md), which supersedes the stock part.
The money and finance ledgers below remain Proposed (finance release).

**Decision.**
* Stock is an immutable movement ledger (receipt, consumption, production output, transfer,
  shipment, repair use, van replenishment, adjustment). Corrections are reversal movements.
* A balance table per product/location/lot is updated in the same transaction as each movement,
  so availability queries stay fast. It can always be rebuilt from the ledger.
* Posted invoices and financial entries follow the same rule: no edits, only credit notes and
  reversals. The audit trail (`audit.events`) records who did what.
