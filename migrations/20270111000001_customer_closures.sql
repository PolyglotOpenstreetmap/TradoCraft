-- Customer closures: periods in which a customer cannot receive parcels or shipments
-- (holidays, collective leave, inventory, works). Entered by staff first (a secretary); the
-- `source` column leaves room for the customer portal to enter them later.
--
-- A closure describes the customer, not one of our companies, so it is shared across companies
-- like crm.customers and crm.contacts. Dates are inclusive: a one-day closure has
-- starts_on = ends_on.
CREATE TABLE crm.customer_closures (
    id          uuid PRIMARY KEY,
    customer_id uuid NOT NULL REFERENCES crm.customers(id),
    starts_on   date NOT NULL,
    ends_on     date NOT NULL,
    kind        text NOT NULL DEFAULT 'other'
                CHECK (kind IN ('holiday', 'collective_leave', 'inventory', 'works', 'other')),
    notes       text NOT NULL DEFAULT '',
    source      text NOT NULL DEFAULT 'staff' CHECK (source IN ('staff', 'portal')),
    created_by  uuid REFERENCES core.users(id),
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz NOT NULL DEFAULT now(),
    CHECK (ends_on >= starts_on)
);
CREATE INDEX customer_closures_customer_idx ON crm.customer_closures (customer_id, ends_on);
CREATE INDEX customer_closures_period_idx ON crm.customer_closures (starts_on, ends_on);
