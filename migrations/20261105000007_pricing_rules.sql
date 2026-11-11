-- Pricing rules: product families, pricing types, customer rebates, price date on quotations.

CREATE TABLE products.product_families (
    id          uuid PRIMARY KEY,
    code        text NOT NULL UNIQUE,
    name        text NOT NULL,
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now()
);

-- list:       list price, customer rebates apply
-- net:        fixed net price, never discounted (no rebate, no manual line discount)
-- indicative: list price is a suggestion; the salesperson must confirm the price on each quote
ALTER TABLE products.products
    ADD COLUMN family_id uuid REFERENCES products.product_families(id),
    ADD COLUMN pricing_type text NOT NULL DEFAULT 'list' CHECK (pricing_type IN ('list', 'net', 'indicative'));
CREATE INDEX products_family_idx ON products.products (family_id);

-- Rebates for a specific product or product family, per customer and company.
-- The general rebate (all products) stays in crm.customer_terms.discount_percent.
-- For a product, the most specific valid rule wins: product, then family, then general.
CREATE TABLE crm.customer_rebates (
    id           uuid PRIMARY KEY,
    company_id   uuid NOT NULL REFERENCES core.companies(id),
    customer_id  uuid NOT NULL REFERENCES crm.customers(id),
    scope        text NOT NULL CHECK (scope IN ('product', 'family')),
    product_id   uuid REFERENCES products.products(id),
    family_id    uuid REFERENCES products.product_families(id),
    percent      numeric(5,2) NOT NULL CHECK (percent >= 0 AND percent <= 100),
    valid_from   date,
    valid_to     date,
    notes        text NOT NULL DEFAULT '',
    created_by   uuid NOT NULL REFERENCES core.users(id),
    created_at   timestamptz NOT NULL DEFAULT now(),
    CHECK ((scope = 'product' AND product_id IS NOT NULL AND family_id IS NULL)
        OR (scope = 'family' AND family_id IS NOT NULL AND product_id IS NULL)),
    CHECK (valid_to IS NULL OR valid_from IS NULL OR valid_to >= valid_from)
);
CREATE INDEX customer_rebates_lookup_idx ON crm.customer_rebates (company_id, customer_id);

-- Quotations: prices are taken from the price list valid on price_date (normally the quote date).
ALTER TABLE sales.quotations ADD COLUMN price_date date;
UPDATE sales.quotations SET price_date = quote_date;
ALTER TABLE sales.quotations ALTER COLUMN price_date SET NOT NULL;

-- Lines record which rebate was applied and why, and whether an indicative price was confirmed.
ALTER TABLE sales.quotation_lines
    ADD COLUMN pricing_type text NOT NULL DEFAULT 'list' CHECK (pricing_type IN ('list', 'net', 'indicative')),
    ADD COLUMN rebate_percent numeric(5,2) NOT NULL DEFAULT 0,
    ADD COLUMN rebate_source text NOT NULL DEFAULT 'general'
        CHECK (rebate_source IN ('product', 'family', 'general', 'net')),
    ADD COLUMN price_confirmed boolean NOT NULL DEFAULT true;
UPDATE sales.quotation_lines l SET rebate_percent = q.customer_discount_percent
FROM sales.quotations q WHERE q.id = l.quotation_id;
