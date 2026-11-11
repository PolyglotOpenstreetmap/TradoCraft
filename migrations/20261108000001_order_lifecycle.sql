-- Order lifecycle (ADR 0016): draft -> sent -> confirmed, revisions, customer confirmation,
-- priority with history and customer requests, and mail attachments for the confirmation PDF.

-- ---------------------------------------------------------------------------------------------
-- Orders: wider status list, priority, confirmation record.
-- ---------------------------------------------------------------------------------------------
ALTER TABLE sales.orders DROP CONSTRAINT orders_status_check;
ALTER TABLE sales.orders ADD CONSTRAINT orders_status_check
    CHECK (status IN ('draft', 'sent', 'confirmed', 'in_planning', 'planned', 'in_production',
                      'ready_for_shipping', 'cancelled'));
-- Existing orders were already treated as confirmed; new ones start as drafts.
ALTER TABLE sales.orders ALTER COLUMN status SET DEFAULT 'draft';

ALTER TABLE sales.orders
    ADD COLUMN current_revision     integer NOT NULL DEFAULT 1,
    ADD COLUMN priority             text NOT NULL DEFAULT 'normal'
                                    CHECK (priority IN ('low', 'normal', 'expedite')),
    -- the contact the confirmation was sent to (null: the customer's main address)
    ADD COLUMN contact_id           uuid REFERENCES crm.contacts(id),
    ADD COLUMN confirmed_at         timestamptz,
    -- a core.users id when confirmed_via = 'staff', a crm.contacts id when 'portal'
    ADD COLUMN confirmed_by         uuid,
    ADD COLUMN confirmed_via        text CHECK (confirmed_via IN ('staff', 'portal')),
    ADD COLUMN confirmed_revision   integer,
    ADD COLUMN confirmation_note    text NOT NULL DEFAULT '',
    ADD COLUMN cancelled_at         timestamptz,
    ADD COLUMN cancel_reason        text NOT NULL DEFAULT '',
    ADD COLUMN updated_at           timestamptz NOT NULL DEFAULT now(),
    ADD CONSTRAINT orders_confirmation_complete
        CHECK ((confirmed_at IS NULL) = (confirmed_via IS NULL));

UPDATE sales.orders
SET confirmed_at = created_at, confirmed_by = created_by, confirmed_via = 'staff', confirmed_revision = 1
WHERE status = 'confirmed';

-- ---------------------------------------------------------------------------------------------
-- Revisions: each has its own lines, totals and notes. The order row mirrors the latest one.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE sales.order_revisions (
    id          uuid PRIMARY KEY,
    order_id    uuid NOT NULL REFERENCES sales.orders(id) ON DELETE CASCADE,
    revision    integer NOT NULL CHECK (revision >= 1),
    status      text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'sent', 'confirmed', 'superseded')),
    notes       text NOT NULL DEFAULT '',
    subtotal    numeric(18,2) NOT NULL,
    vat_total   numeric(18,2) NOT NULL,
    total       numeric(18,2) NOT NULL,
    created_by  uuid NOT NULL REFERENCES core.users(id),
    created_at  timestamptz NOT NULL DEFAULT now(),
    sent_at     timestamptz,
    sent_by     uuid REFERENCES core.users(id),
    sent_to     text,
    UNIQUE (order_id, revision)
);

INSERT INTO sales.order_revisions (id, order_id, revision, status, notes, subtotal, vat_total, total, created_by,
                                   created_at, sent_at, sent_by)
SELECT gen_random_uuid(), id, 1, CASE WHEN status = 'confirmed' THEN 'confirmed' ELSE 'sent' END, notes, subtotal,
       vat_total, total, created_by, created_at, created_at, created_by
FROM sales.orders;

ALTER TABLE sales.order_lines
    ADD COLUMN revision     integer NOT NULL DEFAULT 1,
    ADD COLUMN pricing_type text NOT NULL DEFAULT 'list';
ALTER TABLE sales.order_lines DROP CONSTRAINT order_lines_order_id_position_key;
ALTER TABLE sales.order_lines ADD CONSTRAINT order_lines_revision_position_key UNIQUE (order_id, revision, position);
UPDATE sales.order_lines ol SET pricing_type = COALESCE((
    SELECT ql.pricing_type FROM sales.orders o
    JOIN sales.quotation_lines ql ON ql.quotation_id = o.quotation_id AND ql.position = ol.position
    WHERE o.id = ol.order_id), 'list');

-- ---------------------------------------------------------------------------------------------
-- Priority: every change is kept with its reason; customers can ask for a change.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE sales.order_priority_history (
    id            uuid PRIMARY KEY,
    order_id      uuid NOT NULL REFERENCES sales.orders(id) ON DELETE CASCADE,
    old_priority  text NOT NULL CHECK (old_priority IN ('low', 'normal', 'expedite')),
    new_priority  text NOT NULL CHECK (new_priority IN ('low', 'normal', 'expedite')),
    reason        text NOT NULL CHECK (btrim(reason) <> ''),
    changed_by    uuid NOT NULL REFERENCES core.users(id),
    changed_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX order_priority_history_order_idx ON sales.order_priority_history (order_id, changed_at);

CREATE TABLE sales.order_priority_requests (
    id                    uuid PRIMARY KEY,
    company_id            uuid NOT NULL REFERENCES core.companies(id),
    order_id              uuid NOT NULL REFERENCES sales.orders(id) ON DELETE CASCADE,
    requested_priority    text NOT NULL CHECK (requested_priority IN ('low', 'expedite')),
    message               text NOT NULL CHECK (btrim(message) <> ''),
    requested_by_contact  uuid NOT NULL REFERENCES crm.contacts(id),
    status                text NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'applied', 'dismissed')),
    created_at            timestamptz NOT NULL DEFAULT now(),
    resolved_at           timestamptz,
    resolved_by           uuid REFERENCES core.users(id),
    resolution_note       text NOT NULL DEFAULT ''
);
CREATE INDEX order_priority_requests_order_idx ON sales.order_priority_requests (order_id, created_at);

-- ---------------------------------------------------------------------------------------------
-- System mail can carry one attachment (the order confirmation PDF).
-- ---------------------------------------------------------------------------------------------
ALTER TABLE core.mail_outbox
    ADD COLUMN attachment_name text,
    ADD COLUMN attachment_type text,
    ADD COLUMN attachment_data bytea,
    ADD CONSTRAINT mail_outbox_attachment_complete
        CHECK ((attachment_name IS NULL) = (attachment_data IS NULL) AND (attachment_name IS NULL) = (attachment_type IS NULL));
