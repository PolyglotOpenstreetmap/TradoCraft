# 0026 — Shipping-quote workflow, packaging categories and quote purpose

**Status:** Accepted

**Context.** Some products (large flat panels, crated assemblies) cannot be shipped at a standard
rate: a person has to ask carriers for a price, and that price must be on the quotation before the
customer sees it. Until now products had no packaging or dimensions, and nothing in the quotation
flow could wait for a human. Quotations and orders also did not say whether the goods are for one
installation or for the customer's own stock.

**Decision.** Migrations `20270102000001_quote_purpose.sql` and `20270103000001_packaging_shipping.sql`.

The header comment of `20270103000001_packaging_shipping.sql` cites ADR 0025, the number this ADR had when the file was first committed. It is frozen on purpose (sqlx checksums); see the [runbook](../runbooks/sqlx-migration-checksums.md).

* **Purpose and delivery country.** `sales.quotations` and `sales.orders` carry `purpose`
  (`installation` default, or `stock`) and a nullable `delivery_country_code`. The country defaults
  to the customer's for an installation; orders inherit both on conversion. Purpose is a label for
  now: it does **not** change the shipping workflow (a stock quotation with a flat panel needs a
  shipping quote like any other). Later steps (grid and compatibility warnings) will read it.
* **Packaging categories** (`products.packaging_categories`, master data without `company_id`, like
  product families): `code` (unique), `name`, `description`, `requires_shipping_quote`, `active`.
  Managed under `/packaging-categories` with `products.read` / `products.write`, audited. A product
  gets a nullable `packaging_category_id` (must be active when set, an already-assigned inactive
  one may stay) and nullable `length_mm`, `width_mm`, `height_mm`, `weight_g`, each `> 0` when set.
  The sales crate asks the products crate (`requiring_shipping_quote`) instead of reading the table.
* **Shipping requests** (`sales.shipping_requests`, with `company_id`): status `open` ->
  `requested` -> `quoted`, or `cancelled`. A partial unique index allows one live (non-cancelled)
  request per quotation; cancelled ones stay as history. `sales.quotations.shipping_status`
  (`not_required` | `pending` | `quoted`) is derived from it for filtering and for the send check.
* **Keeping the request in step.** `shipping::sync` runs inside the transaction of create, update
  and both kinds of revision. If any line's product is in a category that requires a quote and
  there is no live request, an `open` one is created and logistics is notified; if no line needs it
  any more, the live request is cancelled and the `SHIPPING` line removed. The quoted state
  survives re-saving a draft (a PUT replaces all lines, so the `SHIPPING` line is put back). A
  revision with `pricing: keep` copies the request and its quoted cost; `pricing: current`
  reprices and asks again. Changing quantities on an already quoted quotation does **not** reopen
  the request: logistics priced the shipment, and the salesperson can ask for a new quote through a
  revision with current prices. This is a deliberate simplification.
* **Logistics endpoints** (new permission `shipping.manage`, granted to `admin` and the new
  `logistics` role, which also reads customers, products, quotations, orders and stock):
  `GET /shipping-requests` (open and requested by default, `?status=` for others),
  `POST /shipping-requests/{id}/request {carrier, notes}` and
  `POST /shipping-requests/{id}/quote {amount, currency?, carrier?, notes?}`. Both only work while
  the quotation is a draft (409 otherwise). The currency must be the quotation currency.
* **The cost is a line.** Quoting adds (or replaces) a non-product line with code `SHIPPING`
  (`product_id` NULL, one unit, net price, `pricing_type` and `rebate_source` `net`), priced with
  the normal `price_line`, then recomputes the stored totals and VAT groups. Its VAT rate is the
  highest rate among the product lines, since shipping follows the goods. Orders copy the line on
  conversion like any other. `line_inputs` (revisions, repricing comparison) skips lines without a
  product, so the line never goes through the price lists.
* **Preview.** `POST /pricing/preview` takes an optional `quotation_id` (the saved draft being
  edited). If it belongs to the company (otherwise 404) and is a draft, has a `quoted` request and
  at least one previewed product still needs a shipping quote, the response also contains the
  `SHIPPING` line (built by `shipping::shipping_line`, the same function the quote entry uses, so
  the VAT rule is shared) in `lines`, `vat_groups` and the totals. For a quotation that is not a
  draft, or with a pending/no request, the preview is the plain product preview. The editor
  therefore shows the same totals as after saving.
* **Sending.** `POST /quotations/{id}/send` first runs `shipping::sync` (a request may be opened
  or cancelled since the draft was saved; an opened request is kept even when the send is refused),
  then returns 409 while `shipping_status` is `pending`. `?override_shipping=true` always needs
  `shipping.manage` (and `quotations.write`), whether or not a shipment is pending; anyone else
  passing the flag gets 403. The override is audited as `shipping_override` only when it skipped a
  pending shipment.
* **Notification.** A new request queues a mail in `core.mail_outbox` (the same mechanism as the SLA
  escalations) to the company's `logistics` users, else its administrators. The text is English
  only for now.

**Consequences.** A quote with a crated product is slower to send, by design, and logistics needs
someone assigned the role. Because the cost is an ordinary line, PDFs, orders and totals need no
special case. Future work: localise the notification, a logistics screen in the app, reopening a
quoted request when the quantities change a lot.
