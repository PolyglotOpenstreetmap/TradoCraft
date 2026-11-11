-- Products (shared master data) and price zones / zone price lists (per company).
CREATE SCHEMA IF NOT EXISTS products;
CREATE SCHEMA IF NOT EXISTS pricing;

CREATE TABLE products.products (
    id            uuid PRIMARY KEY,
    code          text NOT NULL UNIQUE,
    name          text NOT NULL,
    description   text NOT NULL DEFAULT '',
    unit          text NOT NULL DEFAULT 'pcs',
    product_type  text NOT NULL DEFAULT 'purchased'
                  CHECK (product_type IN ('purchased', 'manufactured', 'configurable', 'service')),
    vat_rate      numeric(5,2) NOT NULL DEFAULT 21 CHECK (vat_rate >= 0),
    active        boolean NOT NULL DEFAULT true,
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX products_name_idx ON products.products (lower(name));

CREATE TABLE pricing.price_zones (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    code        text NOT NULL,
    name        text NOT NULL,
    sort_order  integer NOT NULL DEFAULT 0,
    UNIQUE (company_id, code)
);

-- Price history: the valid price at date D is the row with the latest valid_from <= D.
-- Rows are never updated; a price change is a new row.
CREATE TABLE pricing.zone_prices (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    zone_id     uuid NOT NULL REFERENCES pricing.price_zones(id),
    product_id  uuid NOT NULL REFERENCES products.products(id),
    unit_price  numeric(18,4) NOT NULL CHECK (unit_price >= 0),
    valid_from  date NOT NULL,
    created_by  uuid REFERENCES core.users(id),
    created_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE (zone_id, product_id, valid_from)
);
CREATE INDEX zone_prices_lookup_idx ON pricing.zone_prices (product_id, zone_id, valid_from DESC);

-- Per-company commercial terms of a customer.
CREATE TABLE crm.customer_terms (
    customer_id         uuid NOT NULL REFERENCES crm.customers(id),
    company_id          uuid NOT NULL REFERENCES core.companies(id),
    price_zone_id       uuid NOT NULL REFERENCES pricing.price_zones(id),
    discount_percent    numeric(5,2) NOT NULL DEFAULT 0
                        CHECK (discount_percent >= 0 AND discount_percent <= 100),
    payment_terms_days  integer NOT NULL DEFAULT 30 CHECK (payment_terms_days >= 0),
    PRIMARY KEY (customer_id, company_id)
);
