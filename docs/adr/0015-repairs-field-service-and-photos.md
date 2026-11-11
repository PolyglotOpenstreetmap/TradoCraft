# 0015 — Repairs, workshop departments, field service and photos

**Status:** Accepted

**Context.** Customers send products in for repair, often after announcing them in a ticket or in the
portal. The workshop has several departments (electronics, electric, mechanical) and a repair must
reach the right one, sometimes via a second. Technicians visit customers and need to know where to
go, what to do, and to report time, tick tasks and attach photos from a phone.

**Decision.**
* **Departments** (`repairs.departments`) are created and changed by the site admin only
  (`departments.write`, which only the `admin` role has), with the colleagues who belong to each.
  They are data, not code. A department that still has repairs cannot be switched off.
* **Repair workflow** (`repairs::workflow`, pure and unit-tested):
  `announced → received → routed ⇄ in_progress ⇄ waiting_customer / waiting_parts → completed →
  shipped`, plus `cancelled`; completed can go back into work (rework); a routed repair can be sent
  back to triage. Routing is its own action (it needs a department) and may be repeated to hand a
  repair on. A repair in a department always has one (a database check). Every step is kept in
  `repairs.events` with who did it, where it went and why.
* **Announced repairs.** A customer may announce a repair before the parcel arrives, in the portal
  or in a ticket. `POST /repairs` with a `ticket_id` takes customer, person and fault from the
  ticket, links the two and leaves an internal note on the ticket. A ticket can have several
  repairs. When the product arrives, staff mark it received; only then can it be routed.
* **Who may do what.** Reading needs `repairs.read`. Changing a repair, adding notes or photos
  needs `repairs.write` *and* belonging to its department, having registered it, or `repairs.route`.
  Routing needs `repairs.route` (given to support and sales managers). The `workshop` role works in
  its departments but cannot route, so a repair cannot be pulled out of the triage flow by accident.
* **Field jobs** (`crates/fieldservice`, schema `field`): site address (default: the customer's)
  with a navigation link, optional coordinates, instructions, a task checklist, the technicians,
  work/travel logs and photos. Planners (`field.plan`: dispatcher, sales manager, admin) create and
  assign; a technician (`field.work`) sees **only the jobs assigned to them** (other jobs are "not
  found"), starts, ticks, logs, photographs and completes with a mandatory report. After completion
  ticks and time are closed but photos may still be uploaded (taken on site, sent when back online).
* **Offline-friendly writes** (ADR 0006): ticking a task is idempotent, and work logs and photos
  accept a client-chosen id, so a retried upload returns the first one instead of duplicating it.
  The client-side offline queue is described in [ADR 0022](0022-technician-offline-queue.md).
* **Technicians are staff.** They sign in like other staff (OIDC) and use the "My jobs" section of
  the Flutter app, which shows only the tabs they have rights for. They do not use the customer
  portal, which is for the customer's own people.
* **Photos** (`core::files`): JPEG, PNG and WebP only, recognised by their first bytes (a renamed
  executable, a PDF or an SVG that can carry script is refused), at most 12 MB, file names
  stripped of paths, served with `X-Content-Type-Options: nosniff` and only after the rights check
  of the record they belong to. Bytes live in PostgreSQL (`core.file_blobs`, separate from the
  metadata) so uploads are transactional; they can move to object storage behind the same module.
  Uploads are the raw request body (`application/octet-stream`) with the file name and a client id
  in the query. The app scales phone photos to 2400 px before sending.
* **Portal.** Customers announce repairs, add photos while the repair is not finished, and follow a
  simplified status (`announced`, `in_repair`, `waiting_customer`, `ready`, `shipped`,
  `cancelled`). Notes, departments and colleagues' names are never shown; photos are visible only
  for repairs the customer may see (own, or the whole customer with the `customer` scope).

**Consequences.**
* Photos in the database grow it quickly; plan the move to object storage before volumes become
  large, and back up accordingly.
* Photos cannot be deleted from the UI yet, and are not virus-scanned (they are only ever served
  as images).
* Schema names in OpenAPI are global: two types called `NoteInput` in different crates silently
  overwrote each other. The repairs one is `RepairNoteInput`; keep type names unique across crates.

**Later.** An offline queue in the app, signatures on job completion, parts used and stock,
customer notifications when a repair is ready, repairs linked to serial numbers and warranty
checks, job planning board, deleting photos.
