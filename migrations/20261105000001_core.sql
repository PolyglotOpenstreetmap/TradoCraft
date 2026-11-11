-- Core: companies (legal entities), users, memberships, numbering, audit.
CREATE SCHEMA IF NOT EXISTS core;
CREATE SCHEMA IF NOT EXISTS audit;

CREATE TABLE core.companies (
    id            uuid PRIMARY KEY,
    name          text NOT NULL,
    vat_number    text,
    street        text NOT NULL DEFAULT '',
    postal_code   text NOT NULL DEFAULT '',
    city          text NOT NULL DEFAULT '',
    country_code  char(2) NOT NULL DEFAULT 'BE',
    currency      char(3) NOT NULL DEFAULT 'EUR',
    email         text,
    phone         text,
    iban          text,
    created_at    timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE core.users (
    id            uuid PRIMARY KEY,
    email         text NOT NULL,
    display_name  text NOT NULL,
    oidc_subject  text UNIQUE,
    active        boolean NOT NULL DEFAULT true,
    created_at    timestamptz NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX users_email_lower_idx ON core.users (lower(email));

-- A user can work for several legal entities, with roles per entity.
CREATE TABLE core.user_companies (
    user_id     uuid NOT NULL REFERENCES core.users(id),
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    roles       text[] NOT NULL DEFAULT '{}',
    is_default  boolean NOT NULL DEFAULT false,
    PRIMARY KEY (user_id, company_id)
);
CREATE UNIQUE INDEX user_companies_one_default_idx
    ON core.user_companies (user_id) WHERE is_default;

-- Gapless, per-company, per-year document numbering (row lock on increment).
CREATE TABLE core.number_sequences (
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    code        text NOT NULL,
    year        integer NOT NULL,
    next_value  bigint NOT NULL DEFAULT 1,
    PRIMARY KEY (company_id, code, year)
);

-- Append-only business audit trail.
CREATE TABLE audit.events (
    id          bigserial PRIMARY KEY,
    company_id  uuid REFERENCES core.companies(id),
    user_id     uuid REFERENCES core.users(id),
    entity      text NOT NULL,
    entity_id   uuid NOT NULL,
    action      text NOT NULL,
    data        jsonb NOT NULL DEFAULT '{}',
    occurred_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX audit_events_entity_idx ON audit.events (entity, entity_id);
