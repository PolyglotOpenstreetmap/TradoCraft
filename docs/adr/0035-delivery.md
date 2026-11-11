# 0035 — Delivery: delivery notes, picking, shipping and the closure policy

**Status:** Accepted

**Context.** A confirmed sales order had no way to leave the building: nothing planned a delivery,
the stock ledger could only post a bare `shipment` issue by hand, and the customer closures
([ADR 0032](0032-customer-closures.md)) had no delivery date to be checked against. Production
(C1) does not exist yet, so nothing sets an order to `ready_for_shipping` either.

**Decision.** Migration `20270116000001_delivery.sql`, new crate `tradocraft-delivery`, schema
`delivery`, module `delivery` (needs sales and inventory; off where either is off).

* **Delivery notes** (`delivery.delivery_notes`, lines in `delivery.delivery_note_lines`), numbered
  `DN2027-00001` (`core::numbering`, code `DN`). Status `planned` (editable) → `shipped`, or
  `cancelled` (only while planned). A note belongs to one order and the revision that was
  confirmed. It has a planned date, a carrier, a tracking reference, notes and a ship-to address
  copied from the customer (editable).
* **Picking.** Creating a note proposes lines from the order's stock reservations
  (`inventory::reservations::allocations_for`): one line per location and lot, one per unit for
  serial-tracked products. Lines can be changed while the note is planned, also to pick stock that
  is not reserved. Quantities are in the stock unit of the product, like moves and reservations,
  and never exceed what is ordered minus what is shipped or planned on other notes.
* **Shipping** posts one `issue` / `shipment` move per line through `inventory::post_move` to the
  `customer` location, consuming the order line's reservation when it holds the picked stock
  (otherwise free stock is taken), with the note as the move's reference and the note line id as
  the move id (a retry is safe). Serial units become `delivered` to the customer. Lot- and
  serial-tracked lines need their lot and serial number before shipping.
* **Picking details.** A pick takes what the order's reservation holds at its place and lot from
  the reservation (one move that consumes it) and the rest from free stock (a second move). Lines are
  posted in a fixed order (product, place, lot) so two shipments cannot deadlock on the stock
  balances. Lots past their expiry date are not proposed and cannot be shipped. Goods that were
  shipped for the order line outside delivery notes (a plain shipment issue naming the line) count as
  delivered. A serial number is chosen among the units in stock at the pick's location
  (`GET /deliveries/{id}/lines/{line_id}/serial-units`, `delivery.write`, so no right to read
  serial numbers in general is needed). A shipped note keeps the product code, name and unit as they
  were shipped.
* **Concurrent edits.** A note carries `updated_at`; saving with a stale `expected_updated_at` is
  refused (409, `delivery_note_changed`), and the Flutter screen sends it, so shipping never
  overwrites a picking list someone else changed. Moves that shipped a delivery note cannot be
  reversed as plain stock moves.
* **Cancelled orders.** A planned note of an order that was cancelled reads as `cancelled`, cannot
  be changed or shipped, and is no longer listed as planned. Shipping the last open quantity also
  releases what is still reserved for the order.
* **Order status.** Deliveries are allowed for `confirmed`, `ready_for_shipping` and the new
  `partially_shipped`. The first shipment sets `partially_shipped`; shipping the last open quantity
  sets `shipped` (final). Both refuse cancelling and priority changes, like `ready_for_shipping`.
  Delivery calls `sales::orders::{lock_for_delivery, delivery_lines, mark_shipped}`; it never writes
  `sales.*`. B3 does not set `ready_for_shipping`: production (C1) owns that.
* **Closure policy.** Whenever a planned date is set or changed, delivery asks
  `crm::closures::closure_on` / `next_open_day` for the customer. A per-company setting
  (`delivery.settings.closure_policy`, `GET/PUT /delivery/settings`, permission `delivery.admin`,
  audited) decides what happens:
  `warn` (default) saves the date and returns a `closure_warning` (closure and next open day), the
  note remembers `planned_on_closure`; `override` refuses with a 409 (`closure_override_required`)
  unless the user has `delivery.override_closure` and gives a reason, which is stored and audited;
  `block` always refuses with a 409 (`customer_closed`) naming the next open day. Editing something
  else on a date that was already accepted is not refused again, and a closure added after planning
  shows as a live warning on the note. Shipping does not re-check.
* **Permissions** (module Delivery): `delivery.read` (sales, support, dispatcher, logistics,
  warehouse, production, viewer), `delivery.write` plan, pick, edit and cancel (sales, logistics,
  warehouse), `delivery.ship` (warehouse, logistics), `delivery.override_closure` (logistics, sales
  manager) and `delivery.admin` (admin). Planning and shipping are separate so the warehouse and
  sales can be kept apart if a company wants that.
* **API** (tag `delivery`): `GET /deliveries` (status, search, customer, planned period; paged),
  `GET /deliveries/{id}`, `PUT /deliveries/{id}`, `POST /deliveries/{id}/ship`,
  `POST /deliveries/{id}/cancel`, `GET /deliveries/{id}/pdf?lang=`, `POST` and `GET
  /orders/{id}/deliveries`, `GET/PUT /delivery/settings`. Every state change is audited on
  `delivery_note`.
* **PDF**: `templates/delivery_note.typ` in nl/fr/en/de/es (language of the customer unless
  `lang` is given); a planned note is a preview, a shipped one is the final document, with a
  signature line for the receiver.
* **Flutter**: a Deliveries list (with the settings page for the administrator), the delivery note
  screen (planned date with the live closure check, a banner with "use the next open day", the
  reason field in override mode, serial number picker, quantities, ship, cancel, PDF) and a
  Deliveries section on the order screen.

**Not done yet.** Availability (ATP) still counts the full order line as demand after goods were
picked from free stock rather than from the reservation, because inventory only knows consumed
reservations and plain issues naming the order line; a delivery-aware demand figure is a follow-up.
Also: Carrier integrations and label printing, returns and credit notes, a
warehouse per delivery note, picking by barcode, portal visibility of the tracking reference,
invoicing from a shipped note (finance), `ready_for_shipping` driven by production, picking
unreserved stock from the Flutter screen (the API supports it), and weekends and public holidays
in the closure check (ADR 0032).
