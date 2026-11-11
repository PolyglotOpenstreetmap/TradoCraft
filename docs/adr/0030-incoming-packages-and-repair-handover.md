# 0030 — Incoming packages, repair hand-over and billing through sales

**Status:** Accepted (design decisions confirmed by the product owner; see "Decisions")

**Context.** A repair today (ADR 0015) starts either as an *announced* repair (portal or ticket) or is
typed in by staff when a product is already on the bench. Nobody registers the *parcel*. In practice
the work is split over different people:

1. someone at the dock registers a parcel from a customer,
2. someone opens it and decides what it is (a repair, or something else),
3. someone processes the repair: photos, what is wrong, or matching it with a repair the customer
   announced earlier,
4. the repair moves on to the right department,
5. the department fixes it and records materials and hours, or declares it dead on arrival (DOA),
6. someone else tells the customer the result and creates the invoice.

What exists and is reused:

* `repairs.repairs` with the workflow `announced → received → routed ⇄ in_progress ⇄
  waiting_customer / waiting_parts → completed → shipped | cancelled`, events in `repairs.events`,
  photos in `repairs.photos` on top of `core.files`, departments with members, a CHECK that a repair in
  a department has one, and `workflow::portal_status` for the customer view (ADR 0015).
* `POST /repairs` with `ticket_id` (customer, person and fault come from the ticket, internal note on
  the ticket). `repairs.warranty` (`unknown | in_warranty | out_of_warranty`) typed by staff.
* `engineering.serial_units` with `warranty_until` (migration `20261111000001`), and the SQL fragment
  `fieldservice::extras::WARRANTY_STATUS_SQL` that turns it into `in_warranty | expired | unknown`
  (ADR 0020). A repair stores the serial *number* as text only.
* Inventory issues with reason `repair_use` (`inventory::moves`, `POST /inventory/issues`), which
  consume stock without a sales order. Sales: `quotations` (+ lines, product required),
  `POST /quotations/{id}/send|accept|convert`, `sales.orders` with revisions (ADR 0016).
* Helpdesk prepared answers in five languages with rendering
  (`POST /tickets/{id}/prepared-answers/{answer_id}/render`), the ticket reply path, and
  `core.mail_outbox` (`core::mail::enqueue`). Contacts carry a language (ADR 0013/0014).
* Roles are a fixed list in `core::auth::ROLES` mapped by `role_grants` (ADR 0004); user administration
  picks from that list (ADR 0028).

What is missing: a parcel entity, a hand-over between the steps, findings/outcome (incl. DOA), labour
and material lines, an optional estimate, a billing link to sales, an automatic warranty lookup with a
department override, and roles that match the people doing each step.

**Decisions already taken by the product owner.**
* Billing goes **through the existing sales flow**: the costs become a quotation (then order, then
  invoice as today). No new finance module. Statutory accounting stays in the accounting package (ADR 0008).
* **New dedicated roles** are added to `ROLES`.
* An **optional estimate**: before work starts the customer may approve or reject a quoted amount;
  it is never mandatory.
* **Automatic warranty check** from the serial unit; the repair department may override it to
  *void (misuse)*; billing follows the final status.

## Decision

### 1. Overview of the flow

```text
 package:  registered ──► opened ──► repair ─────────────► (linked to a repair, closed)
                            │   └──► other / returned / rejected  (closed, no repair)
                            │
 repair:   announced ─┐
                      ├► received ► (intake: photos, findings, match, warranty) ► routed ► in_progress
 (new)     [package]──┘                                                     ▲        ⇄ waiting_*
                                                                            │            │
                                  estimate (optional) ◄─ in_progress/waiting_customer     ▼
                                                                  completed (repaired | doa | unrepairable)
                                                                            │
                                         communicate + bill (quotation → order)  ──► shipped
```

Each step is done by a different role (section 8). The software enforces the *rights* per step, not
that two steps are done by two different persons (see open questions).

### 2. Data model

One new migration, `migrations/20270107000001_incoming_packages.sql`. Committed migrations are never
edited. Tables live in the existing `repairs` schema (packages are only used for repairs now;
non-repair packages stay in `repairs.packages` with `kind = 'other'`; a later move is a rename).
Naming follows the existing style: `uuid` keys generated in code (UUIDv7), `company_id` on every top
level table, `created_at/updated_at timestamptz`, `numeric(18,2|4)` money, text + CHECK instead of
enums.

#### 2.1 Packages

```sql
CREATE TABLE repairs.packages (
    id              uuid PRIMARY KEY,
    company_id      uuid NOT NULL REFERENCES core.companies(id),
    number          text NOT NULL,                       -- PK-2027-00001, same numbering helper as repairs
    status          text NOT NULL DEFAULT 'registered'
                    CHECK (status IN ('registered', 'opened', 'linked', 'closed')),
    -- what it turned out to be; null until opened
    kind            text CHECK (kind IN ('repair', 'other', 'return_to_sender')),
    customer_id     uuid REFERENCES crm.customers(id),   -- null = sender unknown (resolved at triage)
    sender_name     text NOT NULL DEFAULT '',            -- as written on the label, when no customer yet
    carrier         text NOT NULL DEFAULT '',            -- free text with suggestions (DHL, UPS, bpost, ...)
    tracking_number text NOT NULL DEFAULT '',
    parcel_count    integer NOT NULL DEFAULT 1 CHECK (parcel_count BETWEEN 1 AND 99),
    condition       text NOT NULL DEFAULT 'intact'
                    CHECK (condition IN ('intact', 'damaged', 'opened', 'wet')),
    condition_note  text NOT NULL DEFAULT '',
    customer_reference text NOT NULL DEFAULT '',         -- RMA / their note on the slip
    received_by     uuid NOT NULL REFERENCES core.users(id),
    received_at     timestamptz NOT NULL DEFAULT now(),
    opened_by       uuid REFERENCES core.users(id),
    opened_at       timestamptz,
    notes           text NOT NULL DEFAULT '',
    client_id       uuid,                                -- idempotency key from the app, see 6
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    UNIQUE (company_id, client_id),
    CHECK (status = 'registered' OR kind IS NOT NULL OR status = 'opened'),
    CHECK (status <> 'linked' OR kind = 'repair')
);
CREATE INDEX packages_company_status_idx ON repairs.packages (company_id, status, received_at DESC);
CREATE INDEX packages_tracking_idx ON repairs.packages (company_id, tracking_number)
    WHERE tracking_number <> '';

CREATE TABLE repairs.package_photos (            -- same shape as repairs.photos
    id          uuid PRIMARY KEY,
    package_id  uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    file_id     uuid NOT NULL UNIQUE REFERENCES core.files(id),
    caption     text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE repairs.package_events (              -- same shape as repairs.events
    id uuid PRIMARY KEY,
    package_id uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    kind text NOT NULL CHECK (kind IN ('registered', 'status', 'note', 'photo', 'linked', 'unlinked')),
    from_status text, to_status text,
    repair_id uuid REFERENCES repairs.repairs(id),
    note text NOT NULL DEFAULT '',
    user_id uuid REFERENCES core.users(id),
    actor_name text NOT NULL DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT now()
);
```

Photos on a package show the box and its condition (damage in transit is evidence against the
carrier); photos of the *product* belong to the repair. `core.files` is reused unchanged
(`core::files`, same checks as ADR 0015).

**Package ↔ repair, many-to-one both ways** needs a link table, not a column on either side:

```sql
CREATE TABLE repairs.package_repairs (
    package_id uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    repair_id  uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    linked_by  uuid REFERENCES core.users(id),
    linked_at  timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (package_id, repair_id)
);
CREATE INDEX package_repairs_repair_idx ON repairs.package_repairs (repair_id);
```

One parcel can hold several products (several repairs); one repair can arrive in several parcels
(product and accessories sent separately). No constraint limits either side. The repair detail shows
its parcels, the package detail its repairs.

#### 2.2 Additions to `repairs.repairs`

```sql
ALTER TABLE repairs.repairs
    ADD COLUMN findings          text NOT NULL DEFAULT '',     -- what was found / what is wrong (intake and bench)
    ADD COLUMN diagnosis         text NOT NULL DEFAULT '',     -- the department's conclusion, goes on the quote
    ADD COLUMN outcome           text CHECK (outcome IN ('repaired', 'doa', 'unrepairable', 'no_fault_found')),
    ADD COLUMN outcome_note      text NOT NULL DEFAULT '',
    ADD COLUMN outcome_by        uuid REFERENCES core.users(id),
    ADD COLUMN outcome_at        timestamptz,
    ADD COLUMN estimate_status   text NOT NULL DEFAULT 'none'
                                 CHECK (estimate_status IN ('none', 'requested', 'sent', 'approved', 'rejected')),
    ADD COLUMN billing_status    text NOT NULL DEFAULT 'none'
                                 CHECK (billing_status IN ('none', 'draft', 'quoted', 'ordered', 'free')),
    ADD COLUMN quotation_id      uuid REFERENCES sales.quotations(id),
    ADD COLUMN order_id          uuid REFERENCES sales.orders(id),
    -- warranty, see 2.5
    ADD COLUMN serial_unit_id            uuid REFERENCES engineering.serial_units(id),
    ADD COLUMN warranty_auto             text NOT NULL DEFAULT 'unknown'
                                         CHECK (warranty_auto IN ('unknown', 'in_warranty', 'out_of_warranty')),
    ADD COLUMN warranty_auto_until       date,
    ADD COLUMN warranty_auto_at          timestamptz,
    ADD COLUMN warranty_override_by      uuid REFERENCES core.users(id),
    ADD COLUMN warranty_override_at      timestamptz,
    ADD COLUMN warranty_override_reason  text NOT NULL DEFAULT '',
    ADD COLUMN warranty_overridden       boolean NOT NULL DEFAULT false;

ALTER TABLE repairs.repairs DROP CONSTRAINT repairs_warranty_check;   -- name to verify against the live schema
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_warranty_check
    CHECK (warranty IN ('unknown', 'in_warranty', 'out_of_warranty', 'void'));
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_outcome_consistent
    CHECK (outcome IS NULL OR status IN ('completed', 'shipped'));
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_override_complete
    CHECK (NOT warranty_overridden OR (warranty = 'void' AND warranty_override_by IS NOT NULL
           AND btrim(warranty_override_reason) <> ''));
CREATE INDEX repairs_serial_unit_idx ON repairs.repairs (serial_unit_id);
```

`repairs.repairs.status` is **not** extended (see 3): the existing nine statuses already cover the
workflow, and the new facts (outcome, estimate, billing) are orthogonal columns. That keeps
`workflow::STATUSES`, `can_change`, `portal_status` and the department CHECK untouched.

The existing `source` CHECK (`staff | portal`) is extended to allow `'package'` for a repair created
from a parcel without an announcement (drop/add constraint in the migration). Existing rows are not
changed; default `warranty_auto` = `unknown`, and a backfill sets `warranty_auto = warranty` for
existing rows so history is not rewritten as "unknown".

#### 2.3 Labour and material lines, estimate lines

```sql
CREATE TABLE repairs.cost_lines (
    id            uuid PRIMARY KEY,
    repair_id     uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    kind          text NOT NULL CHECK (kind IN ('labour', 'material', 'inspection', 'other')),
    phase         text NOT NULL DEFAULT 'actual' CHECK (phase IN ('estimate', 'actual')),
    position      integer NOT NULL,
    description   text NOT NULL CHECK (btrim(description) <> ''),
    -- material: a catalogue product, or free text (product_id null)
    product_id    uuid REFERENCES products.products(id),
    quantity      numeric(18,4) NOT NULL CHECK (quantity > 0),     -- hours for labour
    unit          text NOT NULL DEFAULT 'pcs',                      -- 'h' for labour
    -- cost to us (internal); the price to the customer comes from sales pricing, see 5
    unit_cost     numeric(18,4) NOT NULL DEFAULT 0 CHECK (unit_cost >= 0),
    -- labour: who worked and when
    worked_by     uuid REFERENCES core.users(id),
    worked_on     date,
    -- material taken from stock: the inventory move created for it
    stock_move_id uuid,                                              -- inventory move id (no cross-schema FK, ADR 0019 style)
    billable      boolean NOT NULL DEFAULT true,
    client_id     uuid,
    created_by    uuid REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    UNIQUE (repair_id, client_id),
    UNIQUE (repair_id, phase, position),
    CHECK (kind <> 'labour' OR (worked_by IS NOT NULL AND unit = 'h')),
    CHECK (kind <> 'material' OR product_id IS NOT NULL OR btrim(description) <> '')
);
```

* *Labour*: hours (`quantity`), the hourly **cost rate** (`unit_cost`, default from the department's
  `labour_cost_rate`, else the company default), who (`worked_by`) and the day. Overlaps with `field` work logs are not
  merged: a bench repair has no field job.
* *Material*: a product (then description defaults to its name and `unit_cost` to its standard cost,
  visible only with `inventory.cost`) or free text (a cable bought locally).
* *Stock issue* (optional per line): `POST /repairs/{id}/cost-lines` with `issue_from` (location and
  optional lot/serial) calls the inventory issue service in the **same transaction** with reason
  `repair_use`, `note = repair number`, `reference_type = 'repair'`, `reference_id = repair id`
  (the existing `m.reference_*` columns), and stores the move id. Deleting such a line reverses the
  move (`/inventory/moves/{id}/reverse` logic). Needs `inventory.write`; users without it can still
  record the line, and the warehouse issues later (line shows "not issued").
* *Estimate lines* are the same table with `phase = 'estimate'`; when the estimate is approved they
  are what the quotation is made from; actual lines supersede them when billing (section 5).

#### 2.4 Estimate

```sql
CREATE TABLE repairs.estimates (
    id              uuid PRIMARY KEY,
    repair_id       uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    revision        integer NOT NULL DEFAULT 1,
    status          text NOT NULL DEFAULT 'draft'
                    CHECK (status IN ('draft', 'sent', 'approved', 'rejected', 'superseded')),
    amount_excl_vat numeric(18,2),                 -- shown to the customer; snapshot when sent
    max_amount_excl_vat numeric(18,2),             -- optional "do not exceed" ceiling
    message         text NOT NULL DEFAULT '',
    sent_at timestamptz, sent_by uuid REFERENCES core.users(id),
    decided_at timestamptz, decided_by_name text NOT NULL DEFAULT '',
    decided_via text CHECK (decided_via IN ('portal', 'mail', 'phone', 'staff')),
    decision_note text NOT NULL DEFAULT '',
    created_by uuid REFERENCES core.users(id), created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (repair_id, revision)
);
```

The estimate is a repair-side record; it is **not** a sales quotation (the quotation is made once,
at billing). `repairs.repairs.estimate_status` mirrors the newest revision. Approval is recorded by
the customer in the portal, or by staff on the customer's behalf (phone/mail) with `decided_via` and
a note; staff approval needs `repairs.bill`.

#### 2.5 Warranty: automatic lookup, override, final status

Three layers, all on the repair row, so the original automatic result is never lost:

| Column | Meaning | Written by |
|---|---|---|
| `serial_unit_id`, `warranty_auto_until`, `warranty_auto` (`unknown / in_warranty / out_of_warranty`), `warranty_auto_at` | The lookup result and when it was made | System only |
| `warranty_overridden`, `warranty_override_by/_at/_reason` | The department's decision that the warranty is void (misuse) | `repairs.warranty_void` |
| `warranty` | The **final** status used everywhere and by billing: `in_warranty`, `out_of_warranty`, `void`, or `unknown` | derived, see below |

Lookup (`repairs::warranty::lookup`, a pure decision function plus one query, unit-tested):

1. Find the serial unit: `engineering.serial_units` with the same `serial_number` (trimmed,
   case-insensitive) **and** (if the repair has `product_id`) the same `product_id` and, if the
   customer is known, `customer_id` null or equal. `UNIQUE (product_id, serial_number)` makes it exact
   when the product is known; with only a serial number, exactly one match is accepted, several
   matches give `unknown` and the intake person picks one (`PUT /repairs/{id}/serial-unit`).
2. `warranty_until IS NULL` or no unit: `warranty_auto = 'unknown'`.
   `warranty_until >= received_at::date`: `in_warranty`; else `out_of_warranty`.
   (**Decision:** compared with the date the product *arrived* in the workshop, not today, so a
   repair announced in time is not penalised by workshop delay. The field-service SQL uses
   `current_date`; this is a deliberate difference.)
3. Runs on creation, when the serial number / product changes, when the repair is received, and on
   demand (`POST /repairs/{id}/warranty/recheck`). Every run that changes `warranty_auto` writes a
   `warranty_auto` event with old and new value.

Final status: `warranty = 'void'` when `warranty_overridden`, otherwise `warranty = warranty_auto`,
**except** that while `warranty_auto = 'unknown'` staff with `repairs.write` may still set
`warranty` by hand to `in_warranty` or `out_of_warranty` (the existing behaviour, for products without
a serial unit; the event records it as a manual decision). The database CHECK
`repairs_override_complete` guarantees a void always has who, when and why. Removing the override
(`DELETE /repairs/{id}/warranty/override`, same permission, reason required) restores
`warranty = warranty_auto`, and keeps both events.

The override can only move to `void`. It cannot turn an out-of-warranty product into in-warranty:
that is a goodwill decision, handled in billing by `billable = false` lines or a discount on the
quotation (section 5), by someone with `repairs.bill`.

### 3. State machines

#### 3.1 Package (`repairs::packages::workflow`, pure, unit-tested like `workflow.rs`)

```text
registered ──► opened ──► linked   (kind = repair; at least one repair linked)
     │            │
     │            └──► closed     (kind = other | return_to_sender, with a note)
     └──────────────► closed      (refused at the dock, sent back unopened; note required)
```

`can_change`: `registered→opened`, `registered→closed`, `opened→linked`, `opened→closed`;
`linked→closed` is **not** needed (a linked package is done; it is shown with its repairs).
`opened→linked` happens automatically when the first repair link is made; removing the last link
moves it back to `opened` (events `linked` / `unlinked`). Opening records `opened_by/at`; the
person who opens cannot be assumed to be the one who registered.

#### 3.2 Repair: fit with `workflow.rs`

No status is added to `STATUSES`, so `portal_status`, `can_route_from`, `IN_DEPARTMENT` and the
department CHECK are unchanged. What changes:

* `announced → received` is now also what **matching a package** does (section 4). A repair created
  from a package without announcement is inserted directly as `received` (as staff-created repairs
  are today), `source = 'package'`.
* **Intake** (step 3) happens in `received`: findings, photos, matching, warranty; no status change.
* **Routing** (step 4) is the existing `POST /repairs/{id}/route` (needs `repairs.route`). New
  precondition, checked in the handler and in a pure helper `workflow::ready_to_route(&Repair)`:
  at least one photo **or** a note `findings`, and a serial/product description (the minimum for the
  workshop). Missing parts return 422 listing them; `repairs.route` holders can pass `force=true`
  with a reason (recorded in the event note).
* **Estimate** (optional): a repair may be `requested` for an estimate only from `in_progress`
  (or `received/routed` for departments that estimate before opening it). When an estimate is `sent`
  the department sets the existing status `waiting_customer` (`in_progress → waiting_customer`); when
  approved the repair returns `waiting_customer → in_progress` automatically; when rejected the
  repair stays in `waiting_customer` and the repair desk chooses: *return unrepaired*
  (`outcome = unrepairable`, `completed`) or *cancel* (`waiting_customer → cancelled`). No new
  transitions are needed. If no estimate is wanted nothing happens: `estimate_status` stays `none`.
  Work cannot be marked `completed` while `estimate_status IN ('requested', 'sent', 'rejected')`.
* **Completion** (step 5) requires an `outcome`: `POST /repairs/{id}/complete` sets
  `in_progress → completed` with `outcome` (`repaired | doa | unrepairable | no_fault_found`) and a
  mandatory note for everything except `repaired`. Plain `PUT /repairs/{id}/status` to `completed`
  without an outcome is refused (422) once this ships; this is the only change to a current endpoint's
  behaviour, called out under rollout.
* **Shipped** (step 6): `completed → shipped` additionally requires `billing_status <> 'none'`
  (billed, or explicitly free) unless the caller has `repairs.route` and passes a reason. This is how
  "creating the invoice" is made not to be forgotten without blocking the workshop floor.
* **Rework** `completed → in_progress` clears the outcome and, if a quotation exists and is still a
  draft, marks the repair `billing_status = 'draft'` again; a sent quotation is left and the desk
  revises it through the normal quotation revision.

`workflow.rs` additions: `pub const OUTCOMES`, `pub fn outcome_needs_note(&str)`,
`pub fn can_complete(estimate_status, outcome)`, `pub fn ready_to_route(...)`, `pub fn can_ship(billing_status)`.
Unit tests cover each, plus the invariant that `STATUSES` and the DB CHECK stay equal (a test reads
the migration text, as other repos' tests do for roles; if none exists this is a new small test).

### 4. Matching an announced repair

Done at the *intake* step by someone with `repairs.intake`, from an opened package of kind
`repair`.

* `GET /packages/{id}/repair-suggestions` returns announced repairs (`status = 'announced'`, same
  company) ranked by a score computed in SQL/Rust and shown with the reasons:
  same `customer_id` (+3; if the package customer is not set, the customer of the best match is
  proposed), serial number equal to a serial in the announcement (+5), `customer_reference` equal
  (+3), same ticket (the package carries `customer_reference` or the staff types the ticket number,
  +3), product text similarity (+1), announced within 60 days (+1). Only the top 10 are returned,
  with score and reasons `["serial", "customer", ...]`. Suggestion is **never** automatic.
* `POST /packages/{id}/repairs` with `{ repair_id }` **confirms** the link: the announced repair moves
  `announced → received` (`received_at = now`), a `package_repairs` row is created, the package
  becomes `linked`, events are written on both, and the warranty lookup runs. The package customer is
  set from the repair if empty; a mismatch of customer (package customer ≠ repair customer) is
  refused (409) unless `confirm_customer_mismatch = true` (a colleague of the customer sent it).
* **If nothing matches:** `POST /packages/{id}/repairs` with `{ new_repair: { customer_id,
  product_id?, product_description, serial_number, fault_description?, ticket_id?, contact_id? } }`
  creates a repair directly as `received` (`source = 'package'`). `fault_description` is NOT NULL in
  the current schema (CHECK non-blank); for intake without knowing the fault the API substitutes
  "To be determined at intake" (translated by the app locale) and `findings` is filled later.
  **Assumption:** keeping the NOT NULL is simpler than altering it; flagged.
* One package can be linked repeatedly (several products); one repair can be linked from a second
  package later (`POST` with an existing `received+` repair is allowed, no status change).
* A suggestion list for the other direction (`GET /repairs/{id}/package-suggestions`) lists opened,
  unlinked packages of the same customer, so an announced repair can also be received from the repair
  screen. Both directions use the same confirm endpoint.

### 5. Routing, DOA, warranty and billing

**Routing trigger (step 4).** Not automatic by default, because the department depends on the fault.
The intake screen has a "Route" action that calls the existing route endpoint. Optional convenience:
`repairs.departments` gets `default_for_product_category`? — **not now** (open question). The
route action also posts, if the repair has a `ticket_id`, an internal note on the ticket
("Repair R-… received and routed to Electronics"), reusing how `create_repair` already notes the
ticket.

**Billing goes through sales.** `POST /repairs/{id}/billing/quotation` (permission `repairs.bill`,
and `quotations.write`) builds a **draft quotation** with the existing quotation insertion code
(`sales::service::insert_quotation` + `insert_lines`), pricing through the pricing engine (ADR 0002),
so rebates, VAT and the customer's price zone apply as everywhere else. The repair stores
`quotation_id` and `billing_status = 'draft'`. The desk then uses the normal flow: edit, send,
accept, convert to an order (`/quotations/{id}/convert`), and invoice as the company already does
after an order; when the order exists the repair gets `order_id` and `billing_status = 'ordered'`
(set by the convert handler through a small hook `repairs::billing::on_order_created(quotation_id)`
called in the same transaction, or by the repair screen on refresh; hook preferred, flagged below).
`ticket_id` relations: the quotation `notes` carries "Repair R-…, ticket T-…"; no sales schema change
for the ticket link in phase 3 (the link is kept on the repair side: `repairs.repairs.quotation_id`).

**Mapping of cost lines to quotation lines.** `QuotationLineInput` requires a `product_id`. Therefore
a per-company setting table maps the lines to *service products*:

```sql
CREATE TABLE repairs.billing_settings (
    company_id            uuid PRIMARY KEY REFERENCES core.companies(id),
    labour_product_id     uuid REFERENCES products.products(id),   -- service product, unit h
    inspection_product_id uuid REFERENCES products.products(id),   -- flat inspection / DOA fee
    misc_material_product_id uuid REFERENCES products.products(id),-- free-text materials
    default_labour_cost_rate numeric(18,4) NOT NULL DEFAULT 0,
    doa_billing           text NOT NULL DEFAULT 'inspection_fee'
                          CHECK (doa_billing IN ('free', 'inspection_fee')),
    updated_at            timestamptz NOT NULL DEFAULT now()
);
```

Rules (a pure function `repairs::billing::plan(repair, lines, settings) -> BillingPlan`, unit-tested):

* Material line with `product_id`: one quotation line for that product, quantity as recorded, the
  catalogue price from the pricing engine, description from the repair line.
* Material without product: one line on `misc_material_product_id`, description as typed. Its price
  is the typed amount (`unit_price`); the product must be an "indicative" product so that the existing
  *confirmed unit price* mechanism accepts it (**assumption**; if the product type does not allow
  this, phase 3 first adds a flag). The unit price shown to the customer is entered by the desk, not
  derived from our cost.
* Labour: one line per distinct description group, on the repair department's `labour_product_id`
  (a service product with its own price list, which is how the hourly rate differs per department;
  `billing_settings.labour_product_id` is the fallback), quantity = hours, price
  from that product's price list (**not** from the internal cost rate, which stays internal).
* Lines with `billable = false` appear on the quotation as lines with a 100 % `line_discount_percent`
  and the description "no charge", so the customer sees what was done. (Alternative: leave them off.)
* The quotation's `valid_days` and `purpose` take the existing defaults; purpose is `stock`
  (not an installation, no grid warnings).

**Warranty in billing (follows the final `warranty` status).**

| Final warranty | What is billed |
|---|---|
| `in_warranty` | Nothing. `billing_status = 'free'`, no quotation is created (a zero-value quotation would clutter sales). The cost lines remain for reporting. Exception: lines the desk explicitly marks as *not covered* (accessories, transit damage) can still be billed: a quotation with only those lines is made. |
| `out_of_warranty` | Everything billable: labour, materials, plus the inspection fee when the estimate was rejected or the outcome is not `repaired`. |
| `void` (misuse) | As out of warranty. The quotation notes carry the reason text from the override, in the customer's language, optionally. |
| `unknown` | Cannot bill until someone sets a final status (the billing action returns 422 "warranty undecided"). The intake or the department sets it. |

**DOA semantics (default, to be confirmed).** `outcome = 'doa'` means: dead on arrival / not
repairable *because of what the customer sent* (no fault, unrepairable, or arrived broken in
transit). Default in `billing_settings.doa_billing = 'inspection_fee'`:

* in warranty: **free**;
* out of warranty or void: the **inspection fee** (one line on `inspection_product_id`, quantity 1)
  and no labour/material unless recorded as billable;
* `outcome = 'no_fault_found'`: the same as DOA;
* a company can switch to `free` for DOA in all cases.

*Decision flagged:* a DOA claim that is a *dead on arrival on our delivery* (a new product delivered
dead, within a few days) is a replacement/credit matter handled by sales, not by this flow. The
repair can be marked `doa` with a note and `billing_status = 'free'` by the desk; no credit note logic
is built here.

**Estimate impact on billing.** If an estimate was **approved**, the quotation is created from the
actual lines but the system warns when the total exceeds `max_amount_excl_vat` (and refuses without a
reason). If **rejected**: only the inspection fee (when out of warranty/void), `outcome =
unrepairable`. If no estimate: actual lines.

**Customer communication (step 6).** The desk sees a prepared answer chosen by outcome × warranty
(codes such as `repair_done`, `repair_doa`, `repair_estimate`, `repair_warranty_void`, created by
`answers.write` users like other prepared answers; shipped as seed data in the five languages).
`POST /repairs/{id}/communication/preview` renders the answer in the **customer's language**
(contact's language, else customer's `language`, as the ticket reply does, ADR 0013) with
placeholders extended by repair data (`{{repair.number}}`, `{{repair.outcome}}`, `{{repair.findings}}`,
`{{repair.total}}`, `{{repair.warranty}}`, `{{repair.override_reason}}`); the desk edits the text and
`POST /repairs/{id}/communication/send` sends it:

* if the repair has a `ticket_id`: through the **ticket reply path** (so it is in the thread and the
  customer can answer), with the quotation PDF attached when one exists;
* else: `core::mail::enqueue_with_attachment` into `core.mail_outbox` to the contact's e-mail.
* An event `communicated` is written on the repair (who, to which address, which answer code); the
  quotation `send` remains its own action in sales (sending the quotation is billing; telling the
  customer the result may be a separate mail).

**Portal visibility.** `portal_status` is unchanged. Added to the portal repair detail (customer
scope rules unchanged): the outcome in plain words, the estimate with Approve / Reject buttons while
`estimate_status = 'sent'` (`POST /portal/repairs/{id}/estimate/decision`), and the repair's quotation
through the existing portal order/quotation view once sent. Notes, findings, the warranty override
reason, cost lines and colleagues' names are **not** shown. The warranty status is shown only as
"in warranty / not in warranty" for the repair; `void` shows as "not in warranty" with the reason
text only if the desk included it in the mail.

### 6. Roles and permissions

New permissions (added to `Permission`, `ALL`/`as_str`, strings in the `a.b` style):

| Permission | String | Meaning |
|---|---|---|
| `PackagesRead` | `packages.read` | See parcels and their photos |
| `PackagesWrite` | `packages.write` | Register, edit, photograph; mark refused/closed unopened |
| `PackagesOpen` | `packages.open` | Open a parcel and decide repair / other / return |
| `RepairsIntake` | `repairs.intake` | Match with announced repair, create from package, findings, serial unit, photos, set warranty when unknown |
| `RepairsCost` | `repairs.cost` | Labour and material lines, stock issue (`repair_use`), outcome/DOA, request estimate |
| `RepairsWarrantyVoid` | `repairs.warranty_void` | Mark warranty void (misuse) and remove that override |
| `RepairsBill` | `repairs.bill` | Estimate send/approve for customer, create quotation, communicate, set billing free |

Existing `RepairsRead / RepairsWrite / RepairsRoute` keep their meaning. Cost visibility: hourly cost
rates and `unit_cost` are returned only to holders of `repairs.cost`, `repairs.bill` or
`inventory.cost` (the others get null), same idea as `InventoryCost`.

New roles (appended to `ROLES`, which becomes 15 long; the order drives how user administration
lists roles, so append rather than insert; the `ROLES` length constant and the admin test change):

| Role | Step | Permissions |
|---|---|---|
| `goods_in` | 1, receiving dock | `CustomersRead, PackagesRead, PackagesWrite, RepairsRead` |
| `repair_intake` | 2-4, open, check, photograph, match, route | `CustomersRead, ProductsRead, SerialsRead, PackagesRead, PackagesWrite, PackagesOpen, RepairsRead, RepairsWrite, RepairsIntake, RepairsRoute, HelpdeskRead, HelpdeskWrite` (internal note on ticket), `ContactsWrite` not given |
| `repair_desk` | 6, communicate and bill | `CustomersRead, ProductsRead, PackagesRead, RepairsRead, RepairsBill, QuotationsRead, QuotationsWrite, OrdersRead, OrdersWrite, HelpdeskRead, HelpdeskWrite, HelpdeskReply, KbRead` |

Changes to existing roles:

* `workshop` (step 5): adds `RepairsCost`, `RepairsWarrantyVoid`, `PackagesRead`. It still works only
  in its own departments and cannot route (ADR 0015 unchanged). `InventoryWrite` is **not** added to
  `workshop`: stock issue by a workshop user needs it, so either the department lead role also
  holds `warehouse`, or phase 2 lets the line be recorded un-issued and the warehouse issues it.
  *Decision flagged: recommended is to grant `InventoryWrite` to `workshop` for `repair_use` only,
  by checking in the handler that the reason is `repair_use`.* (The handler path for cost lines uses
  the service function internally, so the user's `inventory.write` is not required for that call; the
  check is `repairs.cost` + department membership. That is the proposed behaviour.)
* `support` and `sales_manager`: add `RepairsIntake`, `RepairsBill`, `PackagesRead`, `PackagesOpen`
  (they triage today). `sales_manager` also `RepairsWarrantyVoid` (escalation owner) and `RepairsCost`.
  `sales`: `PackagesRead`, `RepairsBill`.
* `dispatcher`, `viewer`, `logistics`: `PackagesRead` for dispatcher and viewer; logistics gets
  `PackagesRead, PackagesWrite` as an alternative receiver at sites without a goods-in person.
* `admin` has everything (the `true` arm).

Who does which step: 1 `goods_in` (or logistics); 2 `repair_intake` or support; 3 `repair_intake`;
4 `repair_intake` (route); 5 `workshop` in the department (cost, outcome, warranty void; the
repair **must** be in their department or they must have registered it, the existing
`ensure_can_work` rule); 6 `repair_desk`, sales or sales manager. A user holding several roles (small
company) can do several steps; the system does not forbid it. Role changes are made in the user
administration (ADR 0028) and the module switch for `repairs` also hides packages (`permission_module`
maps the `packages.*` strings to the repairs module).

### 7. API

All under `/api/v1`, OpenAPI via utoipa like the other crates, type names globally unique (ADR 0015
consequence): `PackageInput`, `PackageRepairLinkInput`, `RepairCostLineInput`, `RepairEstimateInput`,
`RepairOutcomeInput`, `WarrantyOverrideInput`, `RepairBillingResult`, `RepairCommunicationInput`.

| Verb and path | Permission | Request / response sketch |
|---|---|---|
| `POST /packages` | `packages.write` | `{ client_id, customer_id?, sender_name, carrier, tracking_number, parcel_count, condition, condition_note, customer_reference, notes }` -> 201 `Package`; same `client_id` again returns the first (200) |
| `GET /packages` | `packages.read` | filters `status`, `customer_id`, `q` (number, tracking, sender), page -> `PackagePage` |
| `GET /packages/{id}` | `packages.read` | `Package` + photos, events, linked repairs |
| `PUT /packages/{id}` | `packages.write` (while `registered`/`opened`) | edit fields |
| `POST /packages/{id}/photos?filename=&client_id=` | `packages.write` | raw body, same as repair photos |
| `GET /packages/{id}/photos/{file_id}` | `packages.read` | image, `nosniff` |
| `POST /packages/{id}/open` | `packages.open` | `{ kind: repair\|other\|return_to_sender, customer_id?, note }` -> `opened` (or `closed` for other/return) |
| `POST /packages/{id}/close` | `packages.write` | `{ note }` (refused at the dock) |
| `GET /packages/{id}/repair-suggestions` | `repairs.intake` | ranked announced repairs with reasons |
| `POST /packages/{id}/repairs` | `repairs.intake` | `{ repair_id }` or `{ new_repair: {...} }`, `client_id`, `confirm_customer_mismatch?` -> `RepairDetail` |
| `DELETE /packages/{id}/repairs/{repair_id}` | `repairs.intake` | unlink (no status change on the repair) |
| `GET /repairs/{id}/package-suggestions` | `repairs.intake` | opened, unlinked packages of the customer |
| `PUT /repairs/{id}/findings` | `repairs.write` + `repairs.intake` or department | `{ findings, diagnosis? }` |
| `PUT /repairs/{id}/serial-unit` | `repairs.intake` | `{ serial_unit_id }` or `{ clear: true }`; reruns lookup |
| `POST /repairs/{id}/warranty/recheck` | `repairs.write` | reruns lookup -> `RepairWarranty` |
| `PUT /repairs/{id}/warranty/override` | `repairs.warranty_void` | `{ reason }` (required, non-blank) -> void; 409 if the repair is `shipped`/`cancelled` or has a sent quotation (the desk must revise it first) |
| `DELETE /repairs/{id}/warranty/override` | `repairs.warranty_void` | `{ reason }` |
| `GET /repairs/{id}/cost-lines` | `repairs.read` (costs hidden without cost rights) | lines grouped by phase |
| `POST /repairs/{id}/cost-lines` | `repairs.cost` + department rule | `{ client_id, kind, phase?, product_id?, description, quantity, unit?, unit_cost?, worked_by?, worked_on?, billable?, issue_from?: {location_id, lot_id?, serial_unit_id?} }` -> 201 line; idempotent on `client_id` |
| `PUT /repairs/{id}/cost-lines/{line_id}`, `DELETE ...` | `repairs.cost` | change/delete (a stock-issued line is reversed on delete; refused once billed) |
| `POST /repairs/{id}/estimate` | `repairs.cost` (create/request) | `{ message, max_amount_excl_vat }` -> estimate `draft` |
| `POST /repairs/{id}/estimate/send` | `repairs.bill` | snapshots amount from estimate lines; sets repair `waiting_customer`; mail to the customer (prepared answer `repair_estimate`) |
| `POST /repairs/{id}/estimate/decision` | `repairs.bill` | `{ decision: approve\|reject, via, note }` (staff on behalf) |
| `POST /portal/repairs/{id}/estimate/decision` | portal customer with access to the repair | `{ decision, note }` |
| `POST /repairs/{id}/complete` | `repairs.cost` + department rule | `{ outcome, note }` -> `completed` |
| `POST /repairs/{id}/billing/quotation` | `repairs.bill` + `quotations.write` | `{ client_id, doa_billing?, valid_days? }` -> `RepairBillingResult { billing_status, quotation_id?, lines[], total_excl_vat }`; for in-warranty returns `billing_status: free` and no quotation; 422 on `unknown` warranty |
| `POST /repairs/{id}/billing/free` | `repairs.bill` | `{ reason }` goodwill/credit; sets `billing_status = free` |
| `POST /repairs/{id}/communication/preview` | `repairs.bill` | `{ answer_id? }` -> `{ language, subject, body, to }` |
| `POST /repairs/{id}/communication/send` | `repairs.bill` | `{ subject, body, attach_quotation, client_id }` |
| `GET /repair-queues/{name}` | `repairs.read` | `name` in `intake \| to_route \| workshop \| to_communicate`: server-side filters used by the screens (alternatively the existing `GET /repairs` filters; **assumption:** a few new filter params on `GET /repairs`, `needs=intake\|bill\|communicate`, are enough and no new route is needed) |

**Idempotency.** Anything the app creates takes a client-chosen `client_id` (uuid): packages
(`UNIQUE (company_id, client_id)`), cost lines (`UNIQUE (repair_id, client_id)`), photos (already),
repair-from-package (stored on the new repair as `source_client_id`, extra column, or the package
link itself makes a retry return the existing link), billing quotation (the repair row holds
`quotation_id`; a second call returns it with 200 instead of creating a second quotation; the
`client_id` is accepted for symmetry), communication (a `client_id` on the `communicated` event,
unique per repair, so a retried send does not mail twice). Same convention as ADR 0006/0022: the
second call returns the first result. A stock issue created by a cost line uses the cost line's
`client_id` as the inventory move id derivation so a retry cannot issue twice (inventory inputs
already take an `id`).

**Concurrency.** Two people matching the same announced repair: the link and the status change run in
one transaction with `SELECT ... FOR UPDATE` on the repair row; the second gets 409.

### 8. Audit and events

* `repairs.events.kind` CHECK is widened (drop and re-add) with: `package_linked`, `package_unlinked`,
  `findings`, `warranty_auto`, `warranty_manual`, `warranty_void`, `warranty_void_removed`,
  `cost_line`, `outcome`, `estimate`, `billing`, `communicated`. Each carries `user_id`, `actor_name`
  and a note; warranty events put `old -> new` and the reason in the note, and `warranty_void` stores
  the reason verbatim. Status changes keep using `status`/`routed`.
* `repairs.package_events` for the parcel's life.
* The events list is the audit trail; there is no separate audit table (consistent with ADR 0015).
  Override who/when/why are *also* columns, because billing and the portal need them without scanning
  events, and a CHECK guarantees they are complete.
* Ticket: internal notes on the ticket for receive, route, outcome and billing (same mechanism as
  `create_repair`). Stock: inventory moves carry `reference_type = 'repair'`.

### 9. Flutter (desktop first, ADR 0010)

Under `app/lib/features/repairs/` (and `packages/` for the receiving screens); navigation: a new
"Repairs" group (the repairs module switch hides it) with sections shown by permission:

* **Receiving** (`packages_page.dart`, `package_screen.dart`; `goods_in`): a fast form (customer
  picker via `search_picker.dart`, carrier dropdown with free text, tracking, parcel count, condition
  chips, photo button using `shared/photos.dart`) that creates with a `client_id` and returns to an
  empty form for the next parcel; list of today's parcels. The scanner/keyboard-wedge friendly
  tracking field is autofocused.
* **Triage queue** (`repair_intake_page.dart`; `repair_intake`): tabs "To open" (registered parcels),
  "To process" (opened, unlinked, or linked repairs in `received`), "To route". Opening asks
  repair / other / return. Selecting "repair" shows the suggestions with reasons and a Confirm
  button, or "Create new repair".
* **Repair processing**: the existing `repair_screen.dart` gets sections (shown by permission):
  Parcels, Findings (text), Photos (existing widget), Warranty card, Route action with the missing
  items listed.
* **Warranty card**: automatic result (with the serial unit and the date it compared against), the
  final status as a badge, and for `repairs.warranty_void` holders a "Mark warranty void (misuse)"
  button that opens a dialog with a mandatory reason; after use the card shows "Automatic: in warranty
  until 2027-03-01. Overridden to void by Jan Peeters on 2027-02-03: reason". A "Remove override"
  action is available. Everyone else sees the card read-only.
* **Costing** (`repair_costing_tab.dart`; `workshop`): a table for labour (hours, who, day) and
  materials (product search or free text, quantity, optional "take from stock" with location), totals
  (cost totals only with cost rights), Request estimate, and the finish dialog (Repaired / DOA /
  Unrepairable / No fault found + required note). Entries use client ids so the offline queue of ADR
  0022 can carry them later (phase 2 does not require offline).
* **Communicate and bill** (`repair_billing_screen.dart`; `repair_desk`): queue of completed repairs
  "to communicate/bill", shows the final warranty and what will be billed (the `plan` output) before
  the quotation is created; "Create quotation" opens the quotation in the existing sales screens;
  "Write to customer" shows the rendered answer in the customer's language, editable, with the quote
  attached.
* **Portal** (the separate portal UI): repair detail adds the outcome sentence and the Approve/Reject
  estimate card.
* i18n: strings in the existing `l10n` mechanism for all five UI languages; statuses displayed through
  the existing label maps.

### 10. Testing

Rust (integration, `apps/api/tests/`, using `common::{call, setup}` and `#[sqlx::test]` like
`repairs.rs`; new file `packages.rs` and additions to `repairs.rs`, `e2e_service.rs`):

* Pure unit tests: package workflow, `ready_to_route`, `can_complete`, `can_ship`, warranty decision
  function (boundary dates: `warranty_until == received date`, null, several serial matches),
  `billing::plan` (warranty × outcome × estimate matrix, 3 x 4 x 3), role mapping in `auth.rs`
  (`goods_in` cannot route, `workshop` can void but not bill, `repair_desk` cannot cost).
* Integration: idempotent registration (same `client_id` twice); photos checks; opening a package
  as `kind=other` closes it; suggestions ranking and no automatic link; confirm moves `announced ->
  received` once and the second confirmation gets 409; customer mismatch refused; create-new
  when no match; many-to-many (one parcel two repairs, one repair two parcels); route refused when
  incomplete; warranty auto from `serial_units.warranty_until` (in, out, null, ambiguous serial);
  override needs the permission and a reason, keeps `warranty_auto`, shows in events; billing with
  `in_warranty` creates no quotation, `void` and `out_of_warranty` do; DOA fee; estimate approve ->
  back to `in_progress`; reject -> only inspection fee; complete without outcome refused; shipped
  blocked before billing decided; cost line with stock issue produces an inventory move with reason
  `repair_use` and reverses on delete; billing quotation twice returns the same one; communication
  writes the mail to `core.mail_outbox` in the contact's language and is not duplicated on retry;
  portal does not leak findings/costs/override reason; `permission_module` hides packages when the
  repairs module is off; migration applies on a database with existing repairs (backfill check).
* Flutter widget tests (`app/test/`, `fake_http.dart`): receiving form submits once with a
  `client_id`; suggestions list and confirm; warranty card shows automatic and override; void dialog
  requires a reason; costing totals hide cost columns without rights; role-based navigation shows the
  right sections for each new role.
* e2e (`e2e_service.rs` extended or `e2e_repair_handover.rs`): announce in the portal -> goods-in
  registers -> intake opens, matches, photographs, routes -> workshop costs and completes -> desk
  quotes, communicates -> convert to order. Six actors with six roles, one per step; plus a variant
  without an announcement, a DOA variant, an estimate-rejected variant, and a warranty-void variant.

### 11. Migration and rollout

* One migration file per phase (section 12), additive only; existing repairs keep working: new
  columns have defaults, the backfill sets `warranty_auto`, no status is renamed.
* Behaviour changes to an existing endpoint, announced in the release notes: (a) `PUT .../status` to
  `completed` is refused without an outcome; (b) `completed -> shipped` needs a billing decision. Both
  ship in **phase 3 and 4** respectively, not earlier, so the earlier phases change nothing for current
  users. A per-company switch is not added (YAGNI); the rule is simple enough.
* New roles are inert until assigned. No existing user changes role automatically; existing
  `support`/`sales_manager` users get the new permissions through the updated mapping (documented
  in the changelog). The roles list in OpenAPI/Dart client is regenerated (ADR 0005).
* Seed: prepared answers in five languages, a demo billing settings row, seed users for the three
  roles for e2e.
* The API and the Dart client are regenerated; photos for packages use the same storage (growth note
  of ADR 0015 applies, worse with parcel photos).

### 12. Phased implementation plan

Each phase is shippable and testable alone.

1. **Packages and the receiving flow.** Migration: `packages`, `package_photos`, `package_events`,
   `package_repairs`, `repairs.source` widened. Rust: `repairs::packages` (workflow, CRUD, photos,
   open/close, link, suggestions). Roles `goods_in`, `repair_intake`; permissions `packages.*`,
   `repairs.intake`. Flutter: receiving and triage screens, package section in the repair screen.
   Touches: `crates/repairs`, `core/auth.rs`, `core/modules.rs`, `admin.rs` role count, app. Result:
   parcels are registered, opened, matched to announced repairs or turned into new ones, and routed.
2. **Findings, serial unit and warranty.** Migration: `findings`, `diagnosis`, serial/warranty
   columns, widened warranty CHECK, backfill, events kinds. Rust: `repairs::warranty`, recheck,
   override endpoints, `RepairsWarrantyVoid`. Flutter: warranty card and findings. Touches: repairs,
   `fieldservice::extras` (share the SQL fragment or its Rust twin), auth, app. Independent of 1 (it
   works on every repair).
3. **Costing and outcome (incl. DOA).** Migration: `cost_lines`, outcome columns. Rust: cost line
   endpoints, `repair_use` stock issue, `complete`, role changes for `workshop`, `RepairsCost`.
   Flutter: costing tab and finish dialog. The "completed needs outcome" rule starts here.
4. **Billing through sales and communication.** Migration: `billing_settings`, billing columns,
   events. Rust: `repairs::billing::plan`, quotation creation via `sales::service`, the order hook,
   free/goodwill, communication preview/send with prepared answers and mail, role `repair_desk`,
   `RepairsBill`, "shipped needs billing decision". Flutter: communicate-and-bill screen. Touches:
   `crates/sales` (a hook or an exported creation function only), `helpdesk` (placeholders), `core::mail`.
5. **Optional estimate.** Migration: `estimates`, `estimate_status`. Rust: estimate endpoints and rules
   (waiting_customer round trip, completion blocked), portal decision endpoint and view, prepared answer
   `repair_estimate`. Flutter: estimate in costing, portal card. Feeds billing rules (approved ceiling,
   rejected = inspection fee).
6. **Hardening.** Queue filters, reporting (open parcels older than N days, repairs waiting for billing),
   notifications to the next role (e.g. a watcher/notification when a package is registered), full e2e
   variants, docs (README row, ARCHITECTURE update).

Order rationale: 1 and 2 can run in parallel; 3 needs 2 (warranty is on the cost screen); 4 needs 3; 5
needs 4 (it reuses billing and mail).

## Consequences

* The sixfold hand-over is explicit in data (parcel, findings, outcome, billing status) and in rights;
  the repair workflow in `workflow.rs` keeps its nine statuses, so the portal and the department
  CHECK are untouched.
* Billing stays in sales, so rebates, VAT, PDF and order lifecycle are reused; the price of that is the
  mapping through service products and the "indicative price" assumption for free-text lines.
* Three more roles and seven permissions grow the fixed role table; reviewing `role_grants` stays easy
  but the user administration list gets longer.
* Warranty correctness depends on serial units having `warranty_until` filled in; products without a
  serial unit stay `unknown` and need a manual decision.
* Parcel photos increase database size.

## Non-goals

* No new finance module, invoices, credit notes or payment tracking here; no direct accounting/Peppol link
  (ADR 0008 holds). No credit-note flow for DOA on new deliveries.
* No carrier integrations (labels, tracking lookups, pick-up booking), no barcode label printing, no
  return shipping labels.
* No stock reservation of parts for a repair, no automatic purchase of missing parts (waiting_parts
  stays manual).
* No SLA/timers for repairs, no automatic department routing by rules, no technician offline queue for
  these screens in phases 1-5 (ADR 0022 can be extended later).
* No four-eyes enforcement (see open questions), no deleting photos (as in ADR 0015).
* No change to field jobs; a repair that needs a site visit still uses `field.jobs.repair_id`.

## Assumptions made

1. Next free ADR number 0030 (0029 absent) and migration name `20270107000001_...` for phase 1 (one
   file per phase, later dates).
2. `repairs_warranty_check` is the auto-generated constraint name; verify before writing the migration.
3. Warranty is compared with the arrival date, not today.
4. `QuotationLineInput` needs a `product_id`, so free text maps to service products; the indicative
   unit-price mechanism can price the free-text material line.
5. Quotations are created by calling the sales service functions in-process (the crate dependency
   direction repairs -> sales is acceptable; otherwise through a trait or an HTTP-free service in
   `core`).
6. The convert-to-order hook may live in sales calling repairs, or repairs may read
   `sales.orders.quotation_id` on demand; the second avoids a dependency loop and is the fallback.
7. The reply path of a ticket can be called as a service function with an attachment; not verified in
   the code I read.
8. `inventory.write` can be bypassed for `repair_use` inside a department check; needs a security
   review.
9. Keeping `fault_description NOT NULL` with a placeholder rather than relaxing it.
10. The test conventions (`#[sqlx::test]`, `common::call`, `apps/api/tests/*.rs`) and the `app/test`
    structure are as seen in the file lists.
11. `receiving` for non-repair parcels uses the same table (`kind = 'other'`).

## Decisions

Confirmed by the product owner on 2026-11-08:

1. **DOA billing:** free in warranty. Out of warranty or warranty void: the flat inspection fee
   (`billing_settings.inspection_product_id`) is billed. "No fault found" is billed the same way as DOA.
2. **Labour rate: per department.** `repairs.departments` gets `labour_cost_rate numeric(18,4)` (internal
   cost) and `labour_product_id` (the service product whose price list is the billed hourly rate). The
   company defaults in `billing_settings` are the fallback when a department has none. Departments are
   edited by admins (`DepartmentsWrite`) as today.
3. **Discount on a warranty-void repair:** the repair desk may propose a discount; it only takes effect
   after a user with the `sales_manager` role (or admin) approves it. The proposal and the approval are
   events on the repair. Until approved the quotation is created at the full price.
4. **Four-eyes:** not enforced. One person may do several steps on a repair. The events record the acting
   user per step, and a repair whose steps were all done by one user is flagged in the event list so
   audits can find it.
5. **Warranty date:** compared with the arrival date, not today.

## Open questions

The earlier open questions 1-5 are resolved above. Still open (none block phases 1-2):

1. **Packages for other purposes** (returns of sold goods, supplier parcels): same table later or a
   separate goods-receipt module? Phase 1 keeps `kind = 'other'` on the same table.
2. **Customer language when only the sender name is known:** fall back to the company default.
3. **Estimate threshold:** automatic estimate required above an amount? (Not mandatory now.)
4. **Department default routing** by product category.
5. Does anyone besides `workshop` need `warranty_void` (for example the repair desk after a phone call)?
