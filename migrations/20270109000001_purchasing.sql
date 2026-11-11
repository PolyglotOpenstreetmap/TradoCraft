-- Purchasing: suppliers, price lists, purchase orders, and goods receipts (ADR 0031).

CREATE SCHEMA IF NOT EXISTS purchasing;

-- Suppliers (vendors) supplying components, materials, or services.
CREATE TABLE purchasing.suppliers (
    id                 uuid PRIMARY KEY,
    company_id         uuid NOT NULL REFERENCES core.companies(id) ON DELETE CASCADE,
    name               text NOT NULL,
    code               text NOT NULL DEFAULT '',
    country            text NOT NULL,
    currency           text NOT NULL DEFAULT 'EUR',
    email              text NOT NULL DEFAULT '',
    phone              text NOT NULL DEFAULT '',
    contact_person     text NOT NULL DEFAULT '',
    payment_terms_days integer NOT NULL DEFAULT 30 CHECK (payment_terms_days >= 0),
    tax_id             text NOT NULL DEFAULT '',
    active             boolean NOT NULL DEFAULT true,
    notes              text NOT NULL DEFAULT '',
    created_at         timestamptz NOT NULL DEFAULT now(),
    updated_at         timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX suppliers_company_code_idx ON purchasing.suppliers (company_id, code) WHERE code <> '';
CREATE INDEX suppliers_company_idx ON purchasing.suppliers (company_id);
CREATE INDEX suppliers_name_idx ON purchasing.suppliers (company_id, name);

-- Supplier product cross-reference, price lists, MOQ, and lead times.
CREATE TABLE purchasing.supplier_products (
    id                    uuid PRIMARY KEY,
    company_id            uuid NOT NULL REFERENCES core.companies(id) ON DELETE CASCADE,
    supplier_id           uuid NOT NULL REFERENCES purchasing.suppliers(id) ON DELETE CASCADE,
    product_id            uuid NOT NULL REFERENCES products.products(id) ON DELETE RESTRICT,
    supplier_product_code text NOT NULL DEFAULT '',
    supplier_product_name text NOT NULL DEFAULT '',
    currency              text NOT NULL DEFAULT 'EUR',
    unit_price            numeric(15, 4) NOT NULL CHECK (unit_price >= 0),
    moq                   numeric(15, 4) NOT NULL DEFAULT 1 CHECK (moq > 0),
    lead_time_days        integer NOT NULL DEFAULT 0 CHECK (lead_time_days >= 0),
    is_preferred          boolean NOT NULL DEFAULT false,
    created_at            timestamptz NOT NULL DEFAULT now(),
    updated_at            timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, supplier_id, product_id)
);

CREATE INDEX supplier_products_supplier_idx ON purchasing.supplier_products (supplier_id);
CREATE INDEX supplier_products_product_idx ON purchasing.supplier_products (product_id);

-- Purchase orders (PO) issued to suppliers.
CREATE TABLE purchasing.purchase_orders (
    id                     uuid PRIMARY KEY,
    company_id             uuid NOT NULL REFERENCES core.companies(id) ON DELETE CASCADE,
    number                 text NOT NULL,
    supplier_id            uuid NOT NULL REFERENCES purchasing.suppliers(id) ON DELETE RESTRICT,
    status                 text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'approved', 'sent', 'partially_received', 'received', 'cancelled')),
    order_date             date NOT NULL DEFAULT CURRENT_DATE,
    expected_delivery_date date,
    currency               text NOT NULL DEFAULT 'EUR',
    subtotal               numeric(15, 2) NOT NULL DEFAULT 0 CHECK (subtotal >= 0),
    tax_total              numeric(15, 2) NOT NULL DEFAULT 0 CHECK (tax_total >= 0),
    total                  numeric(15, 2) NOT NULL DEFAULT 0 CHECK (total >= 0),
    notes                  text NOT NULL DEFAULT '',
    created_by_user_id     uuid NOT NULL REFERENCES core.users(id),
    approved_by_user_id    uuid REFERENCES core.users(id),
    created_at             timestamptz NOT NULL DEFAULT now(),
    updated_at             timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number)
);

CREATE INDEX purchase_orders_company_idx ON purchasing.purchase_orders (company_id);
CREATE INDEX purchase_orders_supplier_idx ON purchasing.purchase_orders (company_id, supplier_id);
CREATE INDEX purchase_orders_status_idx ON purchasing.purchase_orders (company_id, status);

-- Lines within a purchase order.
CREATE TABLE purchasing.purchase_order_lines (
    id                    uuid PRIMARY KEY,
    purchase_order_id     uuid NOT NULL REFERENCES purchasing.purchase_orders(id) ON DELETE CASCADE,
    line_number           integer NOT NULL,
    product_id            uuid NOT NULL REFERENCES products.products(id) ON DELETE RESTRICT,
    supplier_product_code text NOT NULL DEFAULT '',
    description           text NOT NULL DEFAULT '',
    quantity              numeric(15, 4) NOT NULL CHECK (quantity > 0),
    quantity_received     numeric(15, 4) NOT NULL DEFAULT 0 CHECK (quantity_received >= 0),
    unit_price            numeric(15, 4) NOT NULL CHECK (unit_price >= 0),
    line_total            numeric(15, 2) NOT NULL CHECK (line_total >= 0),
    created_at            timestamptz NOT NULL DEFAULT now(),
    UNIQUE (purchase_order_id, line_number)
);

CREATE INDEX po_lines_po_idx ON purchasing.purchase_order_lines (purchase_order_id);
CREATE INDEX po_lines_product_idx ON purchasing.purchase_order_lines (product_id);

-- Goods receipts (GR) recording deliveries against purchase orders.
CREATE TABLE purchasing.goods_receipts (
    id                     uuid PRIMARY KEY,
    company_id             uuid NOT NULL REFERENCES core.companies(id) ON DELETE CASCADE,
    number                 text NOT NULL,
    purchase_order_id      uuid NOT NULL REFERENCES purchasing.purchase_orders(id) ON DELETE RESTRICT,
    warehouse_id           uuid NOT NULL REFERENCES inventory.warehouses(id) ON DELETE RESTRICT,
    location_id            uuid NOT NULL REFERENCES inventory.locations(id) ON DELETE RESTRICT,
    received_date          timestamptz NOT NULL DEFAULT now(),
    received_by_user_id    uuid NOT NULL REFERENCES core.users(id),
    carrier                text NOT NULL DEFAULT '',
    tracking_number        text NOT NULL DEFAULT '',
    supplier_delivery_note text NOT NULL DEFAULT '',
    notes                  text NOT NULL DEFAULT '',
    created_at             timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number)
);

CREATE INDEX goods_receipts_company_idx ON purchasing.goods_receipts (company_id);
CREATE INDEX goods_receipts_po_idx ON purchasing.goods_receipts (company_id, purchase_order_id);

-- Lines within a goods receipt.
CREATE TABLE purchasing.goods_receipt_lines (
    id                uuid PRIMARY KEY,
    goods_receipt_id  uuid NOT NULL REFERENCES purchasing.goods_receipts(id) ON DELETE CASCADE,
    po_line_id        uuid NOT NULL REFERENCES purchasing.purchase_order_lines(id) ON DELETE RESTRICT,
    product_id        uuid NOT NULL REFERENCES products.products(id) ON DELETE RESTRICT,
    quantity_received numeric(15, 4) NOT NULL CHECK (quantity_received > 0),
    lot_number        text,
    created_at        timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX gr_lines_gr_idx ON purchasing.goods_receipt_lines (goods_receipt_id);
CREATE INDEX gr_lines_po_line_idx ON purchasing.goods_receipt_lines (po_line_id);
