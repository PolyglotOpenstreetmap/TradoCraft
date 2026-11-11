# 0020 — Field service: signature, parts, installed base, service report and maintenance contracts

**Status:** Accepted

**Context.** [ADR 0015](0015-repairs-field-service-and-photos.md) gave technicians jobs with tasks,
time and photos. What was missing for a real service business: proof that the customer accepted
the work, the parts that were used, which installed unit was worked on (and whether it is still in
warranty), a document for the customer, and recurring maintenance that plans itself.

**Decision.**

* **Signature** (`field.job_signatures`, one row per job). The Flutter app draws on a canvas and
  uploads a PNG (`POST /field/jobs/{id}/signature?signer_name=`, at most 2 MB, checked by its
  signature bytes). The file is stored through `core::files`; the row holds the signer's name,
  who captured it and when. "Declined to sign" is the same row with `declined` and a mandatory
  reason (`POST .../signature/declined`). A new signature replaces the old one (the old file is
  deleted) until the service report has been sent; after that it is a 409. A job must be started
  (or completed) to be signed. The image is read at `GET /field/jobs/{id}/signature`.
* **Parts used** (`field.job_parts`). Product (optional) or free text, quantity, unit, optional
  serial unit, billable flag, and a client-chosen id so a retried upload from a phone is harmless
  (ADR 0006). Added and removed by the job's technician or a planner while the job is not
  completed; on a completed job only `field.plan` may change them; never on a cancelled job.
  **Record only: no stock is deducted.** There is no stock-move column yet because the inventory
  tables do not exist; when inventory lands, a deduction hook will reference `field.job_parts.id`
  (a `stock_move_id` is added then, together with the choice of warehouse, and a backfill decides
  which existing parts to post).
* **Installed base and warranty.** `engineering.serial_units` gets `warranty_until date` and
  `warranty_note`. The warranty status (`in_warranty`, `expired`, `unknown` when no date) is
  computed when read, never stored, so it cannot go stale. It is edited at
  `PUT /serials/{id}/warranty` (`serials.write`). ADR 0011 is untouched: statuses, genealogy and
  the customer link work as before. `field.job_equipment` links a job to units; a unit must belong
  to the job's customer (409 otherwise). `GET /field/customers/{id}/equipment` lists a customer's
  delivered/installed units with their job history (only the jobs the user may see).
* **Service report** (`templates/service_report.typ`, module `crates/fieldservice/src/report.rs`,
  pattern of [ADR 0009](0009-pdf-with-typst.md)). Five languages, chosen by the job contact's
  language, else the customer's, or `?language=`. It shows number, customer and site, dates,
  technicians, the task checklist with results, work and travel totals, parts, equipment with
  warranty, the completion text, up to six photos and the signature (or the refusal and its
  reason). Not printed, by construction (they are not in the data handed to the template):
  the job instructions, notes on work log entries, priority, ticket and repair links. A job that
  is not completed carries a draft watermark. Photos are embedded as stored (there is no image
  resizing in the stack): JPEG and PNG up to 1.5 MB each and 6 MB in total are printed, others are
  counted ("+2 not printed"). If Typst cannot decode a picture the report is rendered without
  pictures instead of failing. `POST /field/jobs/{id}/report/send` (completed jobs only) queues a
  mail with the PDF through `core.mail_outbox` to the contact's primary address, else the
  customer's main address (409 if there is none) and records `report_sent_at`/`report_sent_to` on
  the job. Sending again is allowed.
* **Maintenance contracts** (`field.contracts`, number `SC{year}-{n}` from `core::numbering` code
  `SC`). Status `draft`, `active`, `suspended`, `ended`; start and optional end date; `auto_renew`
  (an active contract whose end date is within 30 days is extended by a year by the worker);
  `response_hours` and `resolution_hours` are targets: informational, and the response target
  sets the priority of generated jobs (up to 4 h urgent, up to 24 h high, otherwise normal). It
  covers the customer's site (own address or the customer's) and optionally units
  (`field.contract_equipment`, same customer). `field.contract_visit_templates` hold what a visit
  is: kind, title, instructions, checklist, default technicians, estimated minutes, `lead_days`
  and the recurrence: every N weeks, every N months, or the listed months of every year, from an
  anchor date. Due dates are plain calendar dates computed by a pure function
  (`recurrence.rs`): month steps are counted from the anchor and clamped to the end of the month
  (31 January, 28 February, 31 March), so there is no time zone or daylight saving involved.
  Permissions reuse `field.plan` (write) and `field.read` (read); no new permission.
* **Generation.** The worker calls `contracts::generate_due_visits(db, today)` every cycle. For
  each active contract and active template it takes the due dates from `today - 7 days` (catch-up
  after downtime) up to `today + lead_days` and never past the end date, and for each inserts a
  `field.contract_visits` row first; the unique key `(template_id, due_date)` makes this
  idempotent across repeated runs and concurrent workers. Only the run that inserted the row
  creates the job, in the same transaction: a planned job with `contract_id` and `due_date`
  (no appointment; the planner schedules it), the template's instructions and tasks, the
  technicians who still work for the company, and the contract's equipment, plus an audit
  event `generated` with no user. Suspended, ended and draft contracts generate nothing; a visit
  that is more than a week overdue is not generated retroactively.

**Consequences.**
* A planner finds generated jobs by `due_date`, `contract_id` (also `GET /field/contracts/{id}/jobs`)
  and the preview `GET /field/contracts/{id}/upcoming`.
* Replacing a visit template (new id) can generate visits again for dates the old one covered.
  Edit a template by its id to avoid that.
* The report prints dates in UTC calendar days; times of day are not shown.
* Photos in the report are not thumbnails; large phone photos are skipped, not shrunk.
* Parts and warranty are not yet connected to quoting or invoicing (billable is a flag for later).

**Not yet.** (The client-side offline queue is [ADR 0022](0022-technician-offline-queue.md).) Stock deduction and returns (after inventory), invoicing of billable parts and
hours, customer portal access to reports and equipment, contract pricing and renewals with
price indexation, SLA breach tracking on contracts, and photo thumbnails in the report.
