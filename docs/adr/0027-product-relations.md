# 0027 — Product relations, co-occurrence and quotation suggestions

**Status:** Accepted

**Context.** Some products do not belong together (a hose clamp that does not fit a profile), some
nearly always go together (a pump and its controller). Salespeople only learn this by experience.
The system should flag explicit conflicts and propose additions while quoting, without ever
blocking a quotation.

**Decision.** Migration `20270105000001_product_relations.sql`.

The header comment of `20270105000001_product_relations.sql` cites ADR 0026, the number this ADR had when the file was first committed. It is frozen on purpose (sqlx checksums); see the [runbook](../runbooks/sqlx-migration-checksums.md).

* **Rules** (`products.product_relations`, master data without `company_id`): `kind`
  (`compatible` | `incompatible` | `suggested`); each side is EITHER a product OR a family
  (`a_product_id`/`a_family_id`, `b_product_id`/`b_family_id`, a CHECK demands exactly one per
  side, so product-vs-product, product-vs-family and family-vs-family all work); `bidirectional`
  (default true), `strength` 1 to 100 (default 50, used by `suggested`), `note`, `active`. A unique
  index prevents the same (kind, sides) twice; the API also refuses the mirrored duplicate (409)
  and a product related to itself (422). Managed under `/product-relations` (`GET` with
  `product_id`, `family_id`, `kind`; `POST`; `PUT /{id}`), `products.read` / `products.write`,
  audited. `GET /products/{id}/related` returns the rules that apply to a product and its
  suggestions.
* **Evaluation** (`products::relations::evaluate`, pure and unit-tested):
  * Conflicts come from explicit active `incompatible` rules between two different products of the
    set only. `compatible` rules never produce a conflict and the absence of a rule is never a
    conflict. Both products are present, so the direction of a rule does not matter for conflicts;
    it does for suggestions.
  * `suggested` rules suggest the other side's product (for a family side the first
    `max_family_expansion` = 5 active products by code) with the rule's `strength`; the strongest
    rule wins per product. `compatible` rules do not suggest anything.
  * Never suggested: products already in the set, inactive products, and products that an
    `incompatible` rule excludes next to a member of the set.
  * Computed suggestions (below) come strictly after all manual ones, never repeat a manual
    suggestion (manual wins) and never produce conflicts. At most `max_computed` = 5.
* **Co-occurrence** (`products.product_cooccurrence`): one row per unordered pair stored with
  `product_a_id < product_b_id`: `orders_together`, `orders_with_a`, `orders_with_b`,
  `computed_at`. A pair is suggested from product X when `orders_together >= min_orders_together`
  (default 3) and the confidence `orders_together / orders_with_X >= min_confidence` (default
  0.3); confidence is directional, so a rare product pulls its common partner but not the reverse.
  The thresholds are constants in `relations.rs` and can be overridden with the environment
  variables `RELATIONS_MIN_ORDERS_TOGETHER` and `RELATIONS_MIN_CONFIDENCE` (the repo has no
  settings table; `AppState` only carries the database, auth and documents).
* **Computation over all companies.** Products are shared master data without `company_id`, and a
  per-company table would need `company_id` in the key and starve small companies of data. The
  table is therefore computed over the orders of all companies. This does mean that purchasing
  patterns of one company influence the suggestions another company sees (only as "product X is
  often bought with Y", never customer or order data). If that is unacceptable, add `company_id`
  to the key and filter in the job.
* **The job** (`tradocraft_sales::cooccurrence::recompute`, called by `apps/worker` hourly, or
  every `COOCCURRENCE_SECONDS`): uses orders with `purpose = 'installation'`, `confirmed_at` set
  and status not `cancelled`, only the lines of the order's `current_revision`, only lines with a
  product that is not a service (SHIPPING and other product-less lines are dropped); each order
  counts once per pair. Sales reads its own tables; the pure counting (`count_pairs`) and the
  atomic replacement (`replace_cooccurrence`, delete and insert in one transaction) live in
  products. Re-running it gives the same table.
* **Sales.** `crates/sales` calls `products::relations::check_products` (sales never reads the
  relation tables). The `warnings` array of quotation create/update, `POST /pricing/preview` and
  `POST /orders/{id}/confirm` gets an entry per conflicting pair with `kind:
  "incompatible_products"`, `product_code` (first product), `message` and `details`
  (`product_codes`, `note`, `relation_id`). The same responses get an additive `suggestions` array
  (`product_id`, `product_code`, `product_name`, `reason` = `rule` | `often_bought_together`,
  `strength`, `confidence`, `because_of_code`, `note`). Both are empty for purpose `stock`, are
  never blocking, and are empty on plain `GET`s.

**Consequences.** Rules are only as good as the people who maintain them; the thresholds need
tuning on real data. A family rule applies to every present and future product of the family. The
job loads the product sets of all qualifying orders into memory, which is fine for tens of
thousands of orders; move the counting into SQL if that stops being true.
