# 0003 — Multi-company data model from day one

**Status:** Accepted

**Context.** Even with one legal entity today, a second (holding, sister company, foreign branch)
is common, and adding it later to a populated database is very expensive.

**Decision.**
* `core.companies` are legal entities. Users have memberships with roles **per company**.
* Every transactional table carries `company_id` (quotations, prices, terms, and later orders,
  stock locations, movements, production, invoices, journals). Numbering is per company and year.
* Master data is shared: `crm.customers` and `products.products` have no `company_id`.
  Company-specific conditions live in per-company tables (`crm.customer_terms`: price zone,
  discount, payment terms; `pricing.zone_prices`).
* The active company is chosen per request with `X-Company-Id` (default: the user's default
  membership) and enforced in the data access layer.
* Intercompany flows are out of scope until needed.

**Consequences.** One extra column and join; no migration project when a second entity appears.
Row-level security in PostgreSQL can be added later as defence in depth.
