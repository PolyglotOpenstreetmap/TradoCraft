-- Field service completion: customer signature, parts used, installed base and warranty,
-- service report tracking, and maintenance contracts that generate planned jobs (ADR 0020).

-- ---------------------------------------------------------------------------------------------
-- Warranty on individual units (the installed base, ADR 0011 stays as it is: this only adds facts).
-- ---------------------------------------------------------------------------------------------
ALTER TABLE engineering.serial_units
    ADD COLUMN warranty_until date,
    ADD COLUMN warranty_note  text NOT NULL DEFAULT '';

-- ---------------------------------------------------------------------------------------------
-- Maintenance contracts. A contract covers a customer (and optionally a site and units) and owns
-- visit templates; the worker turns the templates into planned jobs before each due date.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE field.contracts (
    id               uuid PRIMARY KEY,
    company_id       uuid NOT NULL REFERENCES core.companies(id),
    number           text NOT NULL,
    customer_id      uuid NOT NULL REFERENCES crm.customers(id),
    contact_id       uuid REFERENCES crm.contacts(id),
    title            text NOT NULL CHECK (btrim(title) <> ''),
    status           text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'suspended', 'ended')),
    start_date       date NOT NULL,
    end_date         date,
    -- an active contract with an end date is extended by a year when the end date comes near
    auto_renew       boolean NOT NULL DEFAULT false,
    -- service level targets: informational, and used for the priority of the generated jobs
    response_hours   integer CHECK (response_hours > 0),
    resolution_hours integer CHECK (resolution_hours > 0),
    -- the site covered; left empty, the customer's address is used
    site_name        text NOT NULL DEFAULT '',
    street           text NOT NULL DEFAULT '',
    postal_code      text NOT NULL DEFAULT '',
    city             text NOT NULL DEFAULT '',
    country_code     char(2) NOT NULL DEFAULT 'BE',
    notes            text NOT NULL DEFAULT '',
    created_by       uuid REFERENCES core.users(id),
    created_at       timestamptz NOT NULL DEFAULT now(),
    updated_at       timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    CHECK (end_date IS NULL OR end_date >= start_date)
);
CREATE INDEX contracts_company_status_idx ON field.contracts (company_id, status);
CREATE INDEX contracts_customer_idx ON field.contracts (customer_id);

-- The units covered by the contract (must belong to the contract's customer; checked by the API).
CREATE TABLE field.contract_equipment (
    contract_id    uuid NOT NULL REFERENCES field.contracts(id) ON DELETE CASCADE,
    serial_unit_id uuid NOT NULL REFERENCES engineering.serial_units(id),
    PRIMARY KEY (contract_id, serial_unit_id)
);

CREATE TABLE field.contract_visit_templates (
    id                uuid PRIMARY KEY,
    contract_id       uuid NOT NULL REFERENCES field.contracts(id) ON DELETE CASCADE,
    position          integer NOT NULL,
    kind              text NOT NULL DEFAULT 'maintenance'
                      CHECK (kind IN ('installation', 'maintenance', 'repair', 'inspection', 'other')),
    title             text NOT NULL CHECK (btrim(title) <> ''),
    instructions      text NOT NULL DEFAULT '',
    tasks             text[] NOT NULL DEFAULT '{}',
    -- every N weeks, every N months, or in the listed months of every year
    recurrence        text NOT NULL CHECK (recurrence IN ('weeks', 'months', 'months_list')),
    interval_n        integer NOT NULL DEFAULT 1 CHECK (interval_n BETWEEN 1 AND 60),
    months            integer[] NOT NULL DEFAULT '{}',
    -- the first due date; the day of the month of months_list visits is taken from it
    anchor_date       date NOT NULL,
    lead_days         integer NOT NULL DEFAULT 14 CHECK (lead_days BETWEEN 0 AND 180),
    technician_ids    uuid[] NOT NULL DEFAULT '{}',
    estimated_minutes integer CHECK (estimated_minutes > 0),
    active            boolean NOT NULL DEFAULT true,
    CHECK (recurrence <> 'months_list' OR cardinality(months) > 0),
    UNIQUE (contract_id, position)
);

-- ---------------------------------------------------------------------------------------------
-- Jobs: link to a contract, a due date, and what happened to the service report.
-- ---------------------------------------------------------------------------------------------
ALTER TABLE field.jobs
    ADD COLUMN contract_id    uuid REFERENCES field.contracts(id),
    ADD COLUMN due_date       date,
    ADD COLUMN report_sent_at timestamptz,
    ADD COLUMN report_sent_to text NOT NULL DEFAULT '';
CREATE INDEX jobs_contract_idx ON field.jobs (contract_id);

-- One row per generated visit: the unique key makes generation idempotent.
CREATE TABLE field.contract_visits (
    id          uuid PRIMARY KEY,
    contract_id uuid NOT NULL REFERENCES field.contracts(id) ON DELETE CASCADE,
    template_id uuid NOT NULL REFERENCES field.contract_visit_templates(id) ON DELETE CASCADE,
    due_date    date NOT NULL,
    job_id      uuid REFERENCES field.jobs(id),
    created_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE (template_id, due_date)
);
CREATE INDEX contract_visits_contract_idx ON field.contract_visits (contract_id, due_date);

-- ---------------------------------------------------------------------------------------------
-- Equipment worked on, parts used, customer signature.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE field.job_equipment (
    job_id         uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    serial_unit_id uuid NOT NULL REFERENCES engineering.serial_units(id),
    added_by       uuid REFERENCES core.users(id),
    created_at     timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (job_id, serial_unit_id)
);
CREATE INDEX job_equipment_unit_idx ON field.job_equipment (serial_unit_id);

-- Record only: no stock is deducted yet (ADR 0020).
CREATE TABLE field.job_parts (
    id             uuid PRIMARY KEY,
    job_id         uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    product_id     uuid REFERENCES products.products(id),
    description    text NOT NULL CHECK (btrim(description) <> ''),
    quantity       numeric(12,3) NOT NULL CHECK (quantity > 0),
    unit           text NOT NULL DEFAULT 'pcs',
    serial_unit_id uuid REFERENCES engineering.serial_units(id),
    billable       boolean NOT NULL DEFAULT true,
    added_by       uuid REFERENCES core.users(id),
    created_at     timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX job_parts_job_idx ON field.job_parts (job_id, created_at);

-- The customer's signature (a PNG) or the fact that they declined to sign. One per job.
CREATE TABLE field.job_signatures (
    job_id         uuid PRIMARY KEY REFERENCES field.jobs(id) ON DELETE CASCADE,
    file_id        uuid UNIQUE REFERENCES core.files(id),
    signer_name    text NOT NULL DEFAULT '',
    declined       boolean NOT NULL DEFAULT false,
    decline_reason text NOT NULL DEFAULT '',
    captured_by    uuid REFERENCES core.users(id),
    signed_at      timestamptz NOT NULL DEFAULT now(),
    CHECK ((declined AND file_id IS NULL AND btrim(decline_reason) <> '')
        OR (NOT declined AND file_id IS NOT NULL AND btrim(signer_name) <> ''))
);
