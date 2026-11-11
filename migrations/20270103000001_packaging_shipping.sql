-- Packaging categories, product dimensions and the shipping-quote workflow (ADR 0025).

-- Packaging categories are master data shared by all companies (like product families).
CREATE TABLE products.packaging_categories (
    id                      uuid PRIMARY KEY,
    code                    text NOT NULL UNIQUE CHECK (btrim(code) <> ''),
    name                    text NOT NULL CHECK (btrim(name) <> ''),
    description             text NOT NULL DEFAULT '',
    -- Products in this category cannot be shipped at a standard rate: a person has to ask the
    -- carriers for a price before a quotation containing them is sent.
    requires_shipping_quote boolean NOT NULL DEFAULT false,
    active                  boolean NOT NULL DEFAULT true,
    created_at              timestamptz NOT NULL DEFAULT now(),
    updated_at              timestamptz NOT NULL DEFAULT now()
);

ALTER TABLE products.products
    ADD COLUMN packaging_category_id uuid REFERENCES products.packaging_categories(id),
    ADD COLUMN length_mm integer CHECK (length_mm > 0),
    ADD COLUMN width_mm  integer CHECK (width_mm > 0),
    ADD COLUMN height_mm integer CHECK (height_mm > 0),
    ADD COLUMN weight_g  integer CHECK (weight_g > 0);
CREATE INDEX products_packaging_idx ON products.products (packaging_category_id)
    WHERE packaging_category_id IS NOT NULL;

-- Derived from the active shipping request, for fast filtering of quotations.
ALTER TABLE sales.quotations
    ADD COLUMN shipping_status text NOT NULL DEFAULT 'not_required'
        CHECK (shipping_status IN ('not_required', 'pending', 'quoted'));

CREATE TABLE sales.shipping_requests (
    id            uuid PRIMARY KEY,
    company_id    uuid NOT NULL REFERENCES core.companies(id),
    quotation_id  uuid NOT NULL REFERENCES sales.quotations(id) ON DELETE CASCADE,
    status        text NOT NULL DEFAULT 'open'
                  CHECK (status IN ('open', 'requested', 'quoted', 'cancelled')),
    carrier       text,
    quoted_amount numeric(18,2) CHECK (quoted_amount >= 0),
    currency      char(3),
    notes         text NOT NULL DEFAULT '',
    requested_at  timestamptz,
    requested_by  uuid REFERENCES core.users(id),
    quoted_at     timestamptz,
    quoted_by     uuid REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now(),
    CHECK (status <> 'quoted' OR (quoted_amount IS NOT NULL AND currency IS NOT NULL))
);
-- At most one live request per quotation; a cancelled one stays as history.
CREATE UNIQUE INDEX shipping_requests_active_idx ON sales.shipping_requests (quotation_id)
    WHERE status <> 'cancelled';
CREATE INDEX shipping_requests_queue_idx ON sales.shipping_requests (company_id, created_at)
    WHERE status IN ('open', 'requested');
