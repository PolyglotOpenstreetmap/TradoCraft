-- CRM: customers are shared master data across legal entities;
-- commercial terms (price zone, discount, payment terms) are per company.
CREATE SCHEMA IF NOT EXISTS crm;

CREATE TABLE crm.customers (
    id            uuid PRIMARY KEY,
    code          text NOT NULL UNIQUE,
    name          text NOT NULL,
    vat_number    text,
    email         text,
    phone         text,
    street        text NOT NULL DEFAULT '',
    postal_code   text NOT NULL DEFAULT '',
    city          text NOT NULL DEFAULT '',
    country_code  char(2) NOT NULL DEFAULT 'BE',
    language      char(2) NOT NULL DEFAULT 'nl',
    active        boolean NOT NULL DEFAULT true,
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX customers_name_idx ON crm.customers (lower(name));
