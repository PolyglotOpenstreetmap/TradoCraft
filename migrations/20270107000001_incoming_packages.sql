-- Incoming packages (ADR 0030, phase 1): a parcel from a customer is registered at the dock, opened,
-- and either matched with a repair the customer announced, turned into a new repair, or closed.
-- Additive only: existing repairs keep working.

CREATE TABLE repairs.packages (
    id                 uuid PRIMARY KEY,
    company_id         uuid NOT NULL REFERENCES core.companies(id),
    number             text NOT NULL,
    status             text NOT NULL DEFAULT 'registered'
                       CHECK (status IN ('registered', 'opened', 'linked', 'closed')),
    -- what it turned out to be; null until opened (a parcel refused unopened is 'return_to_sender')
    kind               text CHECK (kind IN ('repair', 'other', 'return_to_sender')),
    -- null = sender unknown, resolved at triage
    customer_id        uuid REFERENCES crm.customers(id),
    sender_name        text NOT NULL DEFAULT '',
    carrier            text NOT NULL DEFAULT '',
    tracking_number    text NOT NULL DEFAULT '',
    parcel_count       integer NOT NULL DEFAULT 1 CHECK (parcel_count BETWEEN 1 AND 99),
    condition          text NOT NULL DEFAULT 'intact' CHECK (condition IN ('intact', 'damaged', 'opened', 'wet')),
    condition_note     text NOT NULL DEFAULT '',
    customer_reference text NOT NULL DEFAULT '',
    received_by        uuid NOT NULL REFERENCES core.users(id),
    received_at        timestamptz NOT NULL DEFAULT now(),
    opened_by          uuid REFERENCES core.users(id),
    opened_at          timestamptz,
    notes              text NOT NULL DEFAULT '',
    -- idempotency key chosen by the app: registering twice returns the first parcel
    client_id          uuid,
    created_at         timestamptz NOT NULL DEFAULT now(),
    updated_at         timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    UNIQUE (company_id, client_id),
    -- once opened (or closed) the parcel has been given a meaning
    CHECK (status = 'registered' OR kind IS NOT NULL),
    CHECK (status NOT IN ('opened', 'linked') OR kind = 'repair')
);
CREATE INDEX packages_company_status_idx ON repairs.packages (company_id, status, received_at DESC);
CREATE INDEX packages_tracking_idx ON repairs.packages (company_id, tracking_number)
    WHERE tracking_number <> '';
CREATE INDEX packages_customer_idx ON repairs.packages (customer_id);

-- Photos of the box and its condition (photos of the product belong to the repair).
CREATE TABLE repairs.package_photos (
    id          uuid PRIMARY KEY,
    package_id  uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    file_id     uuid NOT NULL UNIQUE REFERENCES core.files(id),
    caption     text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX package_photos_idx ON repairs.package_photos (package_id, created_at);

CREATE TABLE repairs.package_events (
    id          uuid PRIMARY KEY,
    package_id  uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    kind        text NOT NULL CHECK (kind IN ('registered', 'status', 'note', 'photo', 'linked', 'unlinked')),
    from_status text,
    to_status   text,
    repair_id   uuid REFERENCES repairs.repairs(id),
    note        text NOT NULL DEFAULT '',
    user_id     uuid REFERENCES core.users(id),
    actor_name  text NOT NULL DEFAULT '',
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX package_events_idx ON repairs.package_events (package_id, created_at);

-- Many to many: one parcel can hold several products, one repair can arrive in several parcels.
CREATE TABLE repairs.package_repairs (
    package_id uuid NOT NULL REFERENCES repairs.packages(id) ON DELETE CASCADE,
    repair_id  uuid NOT NULL REFERENCES repairs.repairs(id) ON DELETE CASCADE,
    linked_by  uuid REFERENCES core.users(id),
    linked_at  timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (package_id, repair_id)
);
CREATE INDEX package_repairs_repair_idx ON repairs.package_repairs (repair_id);

-- A repair created from a parcel has its own source, and a retry of "create from package" must not
-- create a second repair: the app's key is kept on the repair.
ALTER TABLE repairs.repairs DROP CONSTRAINT repairs_source_check;
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_source_check CHECK (source IN ('staff', 'portal', 'package'));
ALTER TABLE repairs.repairs ADD COLUMN source_client_id uuid;
CREATE UNIQUE INDEX repairs_source_client_idx ON repairs.repairs (company_id, source_client_id)
    WHERE source_client_id IS NOT NULL;

-- The repair's history says which parcel it arrived in.
ALTER TABLE repairs.events DROP CONSTRAINT events_kind_check;
ALTER TABLE repairs.events ADD CONSTRAINT events_kind_check
    CHECK (kind IN ('created', 'status', 'routed', 'note', 'photo', 'package_linked', 'package_unlinked'));
