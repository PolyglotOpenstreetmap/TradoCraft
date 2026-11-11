# 0032 — Customer closures: when a customer cannot receive deliveries

**Status:** Accepted

**Context.** Parcels and shipments sent to a customer that is closed (holidays, collective leave,
stocktaking, works) are refused or lost. We want to know these periods before a delivery is
planned. Staff, first a secretary, enter them; a customer portal can let customers do it
themselves later.

**Decision.** Migration `20270111000001_customer_closures.sql`, in `crm` (`crates/crm/src/closures.rs`).

* **`crm.customer_closures`**: `customer_id`, inclusive `starts_on` / `ends_on` (a single day has
  both equal), `kind` (holiday, collective_leave, inventory, works, other), `notes`, `source`
  (`staff` now, `portal` reserved) and `created_by`. Like customers and contacts it is shared across
  companies: it describes the customer, not one of our legal entities. Overlapping or touching
  closures are allowed; they simply chain.
* **API**: `GET/POST /customers/{id}/closures`, `PUT/DELETE /customers/{id}/closures/{closure_id}`,
  `GET /closures` (all customers, for planning; both lists default to closures that have not ended
  yet, `from` / `to` narrow them). Reading needs `customers.read`; writing needs the new permission
  `closures.write` (admin, sales, support, dispatcher, logistics; module Customers). Every change is
  audited on the customer.
* **Using it when planning a shipment**: `GET /customers/{id}/closures/check?date=` answers
  `closed`, the covering closure and `next_open_day`. Rust callers use
  `crm::closures::closure_on` and `crm::closures::next_open_day`, never the table. Only recorded
  closures count; weekends and public holidays are not modelled.
* **Used by delivery** ([ADR 0035](0035-delivery.md)): when a delivery date is planned, the closure
  check runs and a company setting decides whether it only warns (default), needs an override
  reason and right, or blocks. Staff list, add, edit and remove closures in the Flutter customer
  form ("Closed days"); the portal entry point is a follow-up.
