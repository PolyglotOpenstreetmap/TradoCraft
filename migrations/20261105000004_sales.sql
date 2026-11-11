-- Sales: quotations are commercial snapshots. Once sent they are immutable;
-- changes are made through a new revision.
CREATE SCHEMA IF NOT EXISTS sales;

CREATE TABLE sales.quotations (
    id                         uuid PRIMARY KEY,
    company_id                 uuid NOT NULL REFERENCES core.companies(id),
    number                     text NOT NULL,
    revision                   integer NOT NULL DEFAULT 1,
    status                     text NOT NULL DEFAULT 'draft'
                               CHECK (status IN ('draft', 'sent', 'accepted', 'rejected', 'superseded')),
    customer_id                uuid NOT NULL REFERENCES crm.customers(id),
    -- snapshot of the customer and commercial conditions at quotation time
    customer_name              text NOT NULL,
    customer_vat_number        text,
    customer_street            text NOT NULL,
    customer_postal_code       text NOT NULL,
    customer_city              text NOT NULL,
    customer_country_code      char(2) NOT NULL,
    customer_language          char(2) NOT NULL,
    price_zone_code            text NOT NULL,
    customer_discount_percent  numeric(5,2) NOT NULL,
    currency                   char(3) NOT NULL,
    quote_date                 date NOT NULL,
    valid_until                date NOT NULL,
    notes                      text NOT NULL DEFAULT '',
    subtotal                   numeric(18,2) NOT NULL DEFAULT 0,
    vat_total                  numeric(18,2) NOT NULL DEFAULT 0,
    total                      numeric(18,2) NOT NULL DEFAULT 0,
    supersedes_id              uuid REFERENCES sales.quotations(id),
    created_by                 uuid NOT NULL REFERENCES core.users(id),
    created_at                 timestamptz NOT NULL DEFAULT now(),
    updated_at                 timestamptz NOT NULL DEFAULT now(),
    sent_at                    timestamptz,
    UNIQUE (company_id, number, revision)
);
CREATE INDEX quotations_company_idx ON sales.quotations (company_id, quote_date DESC);
CREATE INDEX quotations_customer_idx ON sales.quotations (customer_id);

CREATE TABLE sales.quotation_lines (
    id                         uuid PRIMARY KEY,
    quotation_id               uuid NOT NULL REFERENCES sales.quotations(id) ON DELETE CASCADE,
    position                   integer NOT NULL,
    product_id                 uuid REFERENCES products.products(id),
    product_code               text NOT NULL,
    description                text NOT NULL,
    unit                       text NOT NULL,
    quantity                   numeric(18,4) NOT NULL CHECK (quantity > 0),
    list_unit_price            numeric(18,4) NOT NULL,
    line_discount_percent      numeric(5,2) NOT NULL DEFAULT 0,
    net_unit_price             numeric(18,4) NOT NULL,
    line_total                 numeric(18,2) NOT NULL,
    vat_rate                   numeric(5,2) NOT NULL,
    UNIQUE (quotation_id, position)
);
