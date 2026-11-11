-- Production (ADR 0036): production orders built from the BOM, released by a planner, components
-- reserved at release and backflushed at completion, output into stock. The order status follows.

-- When production expects an order to be ready: the latest planned-ready date of its production orders.
ALTER TABLE sales.orders ADD COLUMN promised_date date;

CREATE SCHEMA production;

-- A team that builds things. Orders are queued per division.
CREATE TABLE production.divisions (
    id         uuid PRIMARY KEY,
    company_id uuid NOT NULL REFERENCES core.companies(id),
    code       text NOT NULL CHECK (btrim(code) <> ''),
    name       text NOT NULL CHECK (btrim(name) <> ''),
    sort_order integer NOT NULL DEFAULT 0,
    active     boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, code)
);

-- The division that normally builds a product (a production order starts there).
CREATE TABLE production.product_divisions (
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    product_id  uuid NOT NULL REFERENCES products.products(id),
    division_id uuid NOT NULL REFERENCES production.divisions(id),
    PRIMARY KEY (company_id, product_id)
);

CREATE TABLE production.production_orders (
    id                 uuid PRIMARY KEY,
    company_id         uuid NOT NULL REFERENCES core.companies(id),
    number             text NOT NULL,
    product_id         uuid NOT NULL REFERENCES products.products(id),
    -- in the stock unit of the product
    quantity           numeric(18,4) NOT NULL CHECK (quantity > 0),
    -- the BOM revision this order was released with; later revisions do not change it
    bom_id             uuid NOT NULL REFERENCES engineering.boms(id),
    bom_revision       integer NOT NULL,
    division_id        uuid REFERENCES production.divisions(id),
    -- the sales order line this builds for; null: build for stock
    order_id           uuid REFERENCES sales.orders(id),
    order_line_id      uuid REFERENCES sales.order_lines(id),
    -- a subassembly order made for a component of another production order
    parent_order_id    uuid REFERENCES production.production_orders(id),
    status             text NOT NULL DEFAULT 'released'
                       CHECK (status IN ('released', 'in_progress', 'completed', 'cancelled')),
    planned_ready_date date,
    started_at         timestamptz,
    completed_at       timestamptz,
    completed_quantity numeric(18,4) CHECK (completed_quantity >= 0),
    notes              text NOT NULL DEFAULT '',
    cancel_reason      text,
    created_by         uuid REFERENCES core.users(id),
    created_at         timestamptz NOT NULL DEFAULT now(),
    updated_at         timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number),
    CHECK ((order_id IS NULL) = (order_line_id IS NULL)),
    CHECK ((status = 'completed') = (completed_at IS NOT NULL))
);
CREATE INDEX production_orders_queue_idx ON production.production_orders (company_id, status, planned_ready_date);
CREATE INDEX production_orders_order_idx ON production.production_orders (order_id);

-- The first-level BOM requirement as it was at release. The stock held for it is a reservation of
-- type production_component whose reference is this row's id.
CREATE TABLE production.order_components (
    id                   uuid PRIMARY KEY,
    production_order_id  uuid NOT NULL REFERENCES production.production_orders(id) ON DELETE CASCADE,
    position             integer NOT NULL,
    component_product_id uuid NOT NULL REFERENCES products.products(id),
    -- BOM quantity x order quantity x (1 + scrap %), rounded up to four decimals
    required_quantity    numeric(18,4) NOT NULL CHECK (required_quantity > 0),
    consumed_quantity    numeric(18,4) NOT NULL DEFAULT 0 CHECK (consumed_quantity >= 0),
    scrap_quantity       numeric(18,4) NOT NULL DEFAULT 0 CHECK (scrap_quantity >= 0),
    UNIQUE (production_order_id, position)
);

-- What completion produced: a stock move per lot, or per unit for serial products.
CREATE TABLE production.order_outputs (
    id                  uuid PRIMARY KEY,
    production_order_id uuid NOT NULL REFERENCES production.production_orders(id) ON DELETE CASCADE,
    quantity            numeric(18,4) NOT NULL CHECK (quantity > 0),
    lot_id              uuid REFERENCES inventory.lots(id),
    serial_unit_id      uuid REFERENCES engineering.serial_units(id),
    stock_move_id       uuid NOT NULL REFERENCES inventory.stock_moves(id)
);

-- The module needs sales, inventory and engineering (core::modules): off where any of them is off.
UPDATE core.companies
SET disabled_modules = array_append(disabled_modules, 'production')
WHERE ('sales' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules)
       OR 'engineering' = ANY (disabled_modules))
  AND NOT ('production' = ANY (disabled_modules));

UPDATE core.user_companies
SET disabled_modules = array_append(disabled_modules, 'production')
WHERE ('sales' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules)
       OR 'engineering' = ANY (disabled_modules))
  AND NOT ('production' = ANY (disabled_modules));
