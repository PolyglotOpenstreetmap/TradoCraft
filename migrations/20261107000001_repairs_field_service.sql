-- Repairs (products customers send in), field service (jobs for technicians), and photo storage.

-- ---------------------------------------------------------------------------------------------
-- Files (photos). Metadata is separate from the bytes so lists stay cheap, and so the bytes can
-- move to object storage later without touching the rest of the schema.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE core.files (
    id            uuid PRIMARY KEY,
    company_id    uuid NOT NULL REFERENCES core.companies(id),
    filename      text NOT NULL,
    content_type  text NOT NULL CHECK (content_type IN ('image/jpeg', 'image/png', 'image/webp')),
    size_bytes    integer NOT NULL CHECK (size_bytes > 0),
    -- who uploaded: a staff user or (portal) a customer's contact; the name is kept as written
    uploaded_by   uuid REFERENCES core.users(id),
    uploader_name text NOT NULL DEFAULT '',
    created_at    timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE core.file_blobs (
    file_id  uuid PRIMARY KEY REFERENCES core.files(id) ON DELETE CASCADE,
    data     bytea NOT NULL
);

-- ---------------------------------------------------------------------------------------------
-- Repairs
-- ---------------------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS repairs;

-- Departments of the workshop (electronics, electric, mechanical, ...), created by the site admin.
CREATE TABLE repairs.departments (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    code        text NOT NULL CHECK (btrim(code) <> ''),
    name        text NOT NULL CHECK (btrim(name) <> ''),
    sort_order  integer NOT NULL DEFAULT 0,
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, code)
);
CREATE TABLE repairs.department_members (
    department_id uuid NOT NULL REFERENCES repairs.departments(id) ON DELETE CASCADE,
    user_id       uuid NOT NULL REFERENCES core.users(id),
    PRIMARY KEY (department_id, user_id)
);

CREATE TABLE repairs.repairs (
    id                  uuid PRIMARY KEY,
    company_id          uuid NOT NULL REFERENCES core.companies(id),
    number              text NOT NULL,
    -- announced: the customer says it is coming; received: it arrived, not yet routed;
    -- routed: waiting in a department; then the work states; shipped = sent back to the customer
    status              text NOT NULL DEFAULT 'received'
                        CHECK (status IN ('announced', 'received', 'routed', 'in_progress', 'waiting_customer',
                                          'waiting_parts', 'completed', 'shipped', 'cancelled')),
    priority            text NOT NULL DEFAULT 'normal' CHECK (priority IN ('low', 'normal', 'high', 'urgent')),
    customer_id         uuid NOT NULL REFERENCES crm.customers(id),
    contact_id          uuid REFERENCES crm.contacts(id),
    -- the ticket where the repair was announced or discussed
    ticket_id           uuid REFERENCES helpdesk.tickets(id),
    product_id          uuid REFERENCES products.products(id),
    product_description text NOT NULL DEFAULT '',
    serial_number       text NOT NULL DEFAULT '',
    fault_description   text NOT NULL CHECK (btrim(fault_description) <> ''),
    customer_reference  text NOT NULL DEFAULT '',
    warranty            text NOT NULL DEFAULT 'unknown' CHECK (warranty IN ('unknown', 'in_warranty', 'out_of_warranty')),
    department_id       uuid REFERENCES repairs.departments(id),
    assigned_user_id    uuid REFERENCES core.users(id),
    source              text NOT NULL DEFAULT 'staff' CHECK (source IN ('staff', 'portal')),
    created_by          uuid REFERENCES core.users(id),
    received_at         timestamptz,
    completed_at        timestamptz,
    created_at          timestamptz NOT NULL DEFAULT now(),
    updated_at          timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    -- a repair in a department has one; waiting in the department queue or being worked on
    CHECK (status NOT IN ('routed', 'in_progress', 'waiting_customer', 'waiting_parts') OR department_id IS NOT NULL)
);
CREATE INDEX repairs_company_status_idx ON repairs.repairs (company_id, status, updated_at DESC);
CREATE INDEX repairs_department_idx ON repairs.repairs (department_id, status);
CREATE INDEX repairs_customer_idx ON repairs.repairs (customer_id);
CREATE INDEX repairs_ticket_idx ON repairs.repairs (ticket_id);

-- The history of a repair: status changes, routing between departments, notes, photos.
CREATE TABLE repairs.events (
    id               uuid PRIMARY KEY,
    repair_id        uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    kind             text NOT NULL CHECK (kind IN ('created', 'status', 'routed', 'note', 'photo')),
    from_status      text,
    to_status        text,
    from_department  uuid REFERENCES repairs.departments(id),
    to_department    uuid REFERENCES repairs.departments(id),
    note             text NOT NULL DEFAULT '',
    user_id          uuid REFERENCES core.users(id),
    actor_name       text NOT NULL DEFAULT '',
    created_at       timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX repair_events_idx ON repairs.events (repair_id, created_at);

CREATE TABLE repairs.photos (
    id          uuid PRIMARY KEY,
    repair_id   uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    file_id     uuid NOT NULL UNIQUE REFERENCES core.files(id),
    caption     text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX repair_photos_idx ON repairs.photos (repair_id, created_at);

-- ---------------------------------------------------------------------------------------------
-- Field service: jobs at a customer's site, done by technicians.
--
-- Technicians work from a phone, sometimes offline (ADR 0006): the facts they record (tasks done,
-- work logs, photos) take a client-generated id, so a retried upload is harmless.
-- ---------------------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS field;

CREATE TABLE field.jobs (
    id               uuid PRIMARY KEY,
    company_id       uuid NOT NULL REFERENCES core.companies(id),
    number           text NOT NULL,
    kind             text NOT NULL DEFAULT 'maintenance'
                     CHECK (kind IN ('installation', 'maintenance', 'repair', 'inspection', 'other')),
    status           text NOT NULL DEFAULT 'planned'
                     CHECK (status IN ('planned', 'in_progress', 'completed', 'cancelled')),
    priority         text NOT NULL DEFAULT 'normal' CHECK (priority IN ('low', 'normal', 'high', 'urgent')),
    title            text NOT NULL CHECK (btrim(title) <> ''),
    -- what has to be done there
    instructions     text NOT NULL DEFAULT '',
    customer_id      uuid NOT NULL REFERENCES crm.customers(id),
    -- the person to meet on site
    contact_id       uuid REFERENCES crm.contacts(id),
    ticket_id        uuid REFERENCES helpdesk.tickets(id),
    repair_id        uuid REFERENCES repairs.repairs(id),
    -- where to go
    site_name        text NOT NULL DEFAULT '',
    street           text NOT NULL DEFAULT '',
    postal_code      text NOT NULL DEFAULT '',
    city             text NOT NULL DEFAULT '',
    country_code     char(2) NOT NULL DEFAULT 'BE',
    latitude         double precision CHECK (latitude BETWEEN -90 AND 90),
    longitude        double precision CHECK (longitude BETWEEN -180 AND 180),
    scheduled_start  timestamptz,
    scheduled_end    timestamptz,
    started_at       timestamptz,
    completed_at     timestamptz,
    completion_notes text NOT NULL DEFAULT '',
    created_by       uuid REFERENCES core.users(id),
    created_at       timestamptz NOT NULL DEFAULT now(),
    updated_at       timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    CHECK (scheduled_end IS NULL OR scheduled_start IS NULL OR scheduled_end >= scheduled_start),
    CHECK ((latitude IS NULL) = (longitude IS NULL))
);
CREATE INDEX jobs_company_status_idx ON field.jobs (company_id, status, scheduled_start);
CREATE INDEX jobs_customer_idx ON field.jobs (customer_id);

CREATE TABLE field.job_technicians (
    job_id   uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    user_id  uuid NOT NULL REFERENCES core.users(id),
    PRIMARY KEY (job_id, user_id)
);
CREATE INDEX job_technicians_user_idx ON field.job_technicians (user_id);

-- The checklist: what to do there.
CREATE TABLE field.job_tasks (
    id          uuid PRIMARY KEY,
    job_id      uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    position    integer NOT NULL,
    description text NOT NULL CHECK (btrim(description) <> ''),
    done        boolean NOT NULL DEFAULT false,
    done_by     uuid REFERENCES core.users(id),
    done_at     timestamptz,
    UNIQUE (job_id, position)
);

-- Time and notes of the work done (work or travel).
CREATE TABLE field.work_logs (
    id          uuid PRIMARY KEY,
    job_id      uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    user_id     uuid NOT NULL REFERENCES core.users(id),
    kind        text NOT NULL DEFAULT 'work' CHECK (kind IN ('work', 'travel')),
    started_at  timestamptz NOT NULL,
    ended_at    timestamptz NOT NULL,
    notes       text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now(),
    CHECK (ended_at >= started_at)
);
CREATE INDEX work_logs_job_idx ON field.work_logs (job_id, started_at);

CREATE TABLE field.job_photos (
    id          uuid PRIMARY KEY,
    job_id      uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    file_id     uuid NOT NULL UNIQUE REFERENCES core.files(id),
    caption     text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX job_photos_idx ON field.job_photos (job_id, created_at);
