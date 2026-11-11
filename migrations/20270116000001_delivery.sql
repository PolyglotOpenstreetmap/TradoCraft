-- Delivery (ADR 0035): delivery notes for sales orders, picking from the reservations, shipping,
-- and a per-company policy for planning a delivery on a day the customer is closed.

-- A partly delivered order can no longer be cancelled; a fully delivered order is final.
ALTER TABLE sales.orders DROP CONSTRAINT orders_status_check;
ALTER TABLE sales.orders ADD CONSTRAINT orders_status_check
    CHECK (status IN ('draft', 'sent', 'confirmed', 'in_planning', 'planned', 'in_production',
                      'ready_for_shipping', 'partially_shipped', 'shipped', 'cancelled'));

CREATE SCHEMA delivery;

-- What happens when a delivery is planned on a day the customer is closed (crm.customer_closures):
--   warn      the date is saved and the closure is shown as a warning (default)
--   override  refused unless the user may override (delivery.override_closure) and gives a reason
--   block     always refused; another date has to be chosen
-- No row means the defaults.
CREATE TABLE delivery.settings (
    company_id     uuid PRIMARY KEY REFERENCES core.companies(id),
    closure_policy text NOT NULL DEFAULT 'warn' CHECK (closure_policy IN ('warn', 'override', 'block')),
    updated_at     timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE delivery.delivery_notes (
    id                      uuid PRIMARY KEY,
    company_id              uuid NOT NULL REFERENCES core.companies(id),
    number                  text NOT NULL,
    order_id                uuid NOT NULL REFERENCES sales.orders(id),
    order_revision          integer NOT NULL,
    customer_id             uuid NOT NULL REFERENCES crm.customers(id),
    status                  text NOT NULL DEFAULT 'planned' CHECK (status IN ('planned', 'shipped', 'cancelled')),
    planned_date            date,
    -- the address the goods go to, copied from the customer and editable
    ship_to_name            text NOT NULL,
    ship_to_street          text NOT NULL DEFAULT '',
    ship_to_postal_code     text NOT NULL DEFAULT '',
    ship_to_city            text NOT NULL DEFAULT '',
    ship_to_country_code    char(2) NOT NULL DEFAULT 'BE',
    carrier                 text NOT NULL DEFAULT '',
    tracking_number         text NOT NULL DEFAULT '',
    notes                   text NOT NULL DEFAULT '',
    -- the customer was closed on the planned date when it was saved
    planned_on_closure      boolean NOT NULL DEFAULT false,
    closure_override_reason text,
    shipped_at              timestamptz,
    shipped_by              uuid REFERENCES core.users(id),
    cancelled_at            timestamptz,
    created_by              uuid REFERENCES core.users(id),
    created_at              timestamptz NOT NULL DEFAULT now(),
    updated_at              timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    CHECK ((status = 'shipped') = (shipped_at IS NOT NULL))
);
CREATE INDEX delivery_notes_order_idx ON delivery.delivery_notes (order_id);
CREATE INDEX delivery_notes_list_idx ON delivery.delivery_notes (company_id, status, planned_date);

-- One line per pick: an order line, from one location (and lot); a serial product has one line per unit.
-- Quantities are in the stock unit of the product, like stock moves and reservations.
CREATE TABLE delivery.delivery_note_lines (
    id               uuid PRIMARY KEY,
    delivery_note_id uuid NOT NULL REFERENCES delivery.delivery_notes(id) ON DELETE CASCADE,
    position         integer NOT NULL,
    order_line_id    uuid NOT NULL REFERENCES sales.order_lines(id),
    product_id       uuid NOT NULL REFERENCES products.products(id),
    quantity         numeric(18,4) NOT NULL CHECK (quantity > 0),
    location_id      uuid NOT NULL REFERENCES inventory.locations(id),
    lot_id           uuid REFERENCES inventory.lots(id),
    serial_unit_id   uuid REFERENCES engineering.serial_units(id),
    -- set when the note is shipped: the stock move, and the product as it was then (a shipped note
    -- must not change when the product is renamed); planned lines show the live product
    stock_move_id    uuid REFERENCES inventory.stock_moves(id),
    product_code     text,
    product_name     text,
    unit             text,
    UNIQUE (delivery_note_id, position)
);
CREATE INDEX delivery_note_lines_order_line_idx ON delivery.delivery_note_lines (order_line_id);

-- The module depends on sales and inventory (core::modules): off where either is off.
UPDATE core.companies
SET disabled_modules = array_append(disabled_modules, 'delivery')
WHERE ('sales' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules))
  AND NOT ('delivery' = ANY (disabled_modules));

UPDATE core.user_companies
SET disabled_modules = array_append(disabled_modules, 'delivery')
WHERE ('sales' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules))
  AND NOT ('delivery' = ANY (disabled_modules));
