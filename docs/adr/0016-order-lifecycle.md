# 0016 — Order lifecycle: revisions, customer confirmation and priority

**Status:** Accepted (phase 1; phase 2 planned)

**Context.** Until now an order was a frozen copy of an accepted quotation and counted as confirmed
the moment it existed. In practice the customer confirms the order after seeing an order
confirmation, sometimes asks for changes first, and sales needs to push or hold orders in the
queue once production starts.

**Decision.**
* **States.** `draft → sent → confirmed → in_planning → planned → in_production →
  ready_for_shipping`, and `cancelled` from any state before `in_production`. Delivery
  ([ADR 0035](0035-delivery.md)) adds `partially_shipped` and `shipped` after it; production
  ([ADR 0036](0036-production.md)) now drives `in_planning` to `ready_for_shipping` and a promised
  date. An order in production can be cancelled only while none of its production orders is open
  (released or in progress); from `ready_for_shipping` on it can no longer be cancelled. Phase 1 drives
  `draft`, `sent`, `confirmed` and `cancelled`; the others are already allowed by the database check
  so phase 2 needs no migration of the status column. Converting a quotation
  (`POST /quotations/{id}/convert`) creates the order as revision 1 in `draft`. Orders that existed
  before stay `confirmed`.
* **Revisions** (`sales.order_revisions`, lines carry a `revision`). Each revision has its own
  lines, totals and notes; the order row mirrors the latest one so lists stay simple. A draft
  can be edited (quantity, discount, description per line, notes); prices and rebates stay as agreed
  on the quotation and the server recomputes totals with the pricing crate. A new product is a new
  quotation, not an order edit. Only the latest revision can be sent or confirmed. A *sent* revision
  is replaced with `POST /orders/{id}/revisions`: the new revision is a draft copy and the previous
  one becomes `superseded` and is kept. A confirmed order cannot be revised (409); changes after
  confirmation are a later phase.
* **Order confirmation.** `POST /orders/{id}/send` marks the draft revision sent, renders the
  Typst document `templates/order_confirmation.typ` (nl, fr, en, de, es like the quotation) and queues
  an e-mail with the PDF in `core.mail_outbox` (which gained one attachment) for the worker. The
  recipient is the contact on the ticket linked to the order or its quotation, else the customer's
  main address; without any address the call is a 409 and nothing changes. The language is the
  contact's own or the customer's. `GET /orders/{id}/revisions/{n}/pdf` downloads any revision.
  Internal order notes are never printed.
* **Customer confirmation.** Staff confirm on the customer's behalf
  (`POST /orders/{id}/confirm`, optional note), or the contact does it in the portal
  (`POST /portal/orders/{id}/confirm`, revision required). Only the latest revision, and only while
  it is `sent`, can be confirmed; an older revision, a draft or an already confirmed order is a 409.
  The order records `confirmed_at`, `confirmed_by` (user or contact id), `confirmed_via`
  (`staff` | `portal`), `confirmed_revision` and the note, and is frozen. The audit event
  `order.confirmed` is the only signal to production for now: the helpdesk has no light notification
  to a production team (a production question is a ticket for the salesperson), and building one
  was out of scope.
* **Portal.** `GET /portal/orders[/{id}]` shows the latest revision that was sent (a draft is
  never shown; while a newer revision is prepared the customer still sees the last sent one, marked
  as not confirmable), without notes. Tenant rules follow tickets and repairs: company and customer
  must match; the `customer` scope sees all orders of the customer, `own` only orders that were
  sent to that contact. Everything else is "not found".
* **Priority.** `orders.priority` is `low | normal | expedite` (default `normal`).
  `PUT /orders/{id}/priority` needs a reason, writes `order_priority_history` (old, new, reason,
  who, when) and an audit event; `GET /orders/{id}/priority-history` reads it. Customers can only
  *ask*: `POST /portal/orders/{id}/priority-requests` (expedite or low, with a message) stores an
  `open` request that staff see at `GET /orders/{id}/priority-requests` and either apply (the
  priority call with `request_id` marks it `applied`) or dismiss.
* **Permissions.** Staff use `orders.read` / `orders.write`; the portal has its own session. Every
  query is scoped by company.

**Consequences.**
* Order numbers do not change with revisions; the revision is shown next to the number.
* Mail attachments live in the database row until the worker has sent them.
* Cancelling does not yet close open priority requests.

**Phase 2 (planned).** Confirmation creates production orders per division (planning, workshop,
assembly, ...), driving `in_planning`, `planned`, `in_production` and `ready_for_shipping` from
their progress. Each division gives a date and the promised delivery date of the order is the
latest of them; changing priority then reorders the divisions' queues. Notifying production of a
confirmed order becomes part of that work. Post-confirmation changes need their own approval flow.
