-- Admin-defined configurators (ADR 0034): models with revisioned definitions (draft → active →
-- obsolete, like BOM revisions), saved configurations that become quotations, and typed product
-- attributes that formulas and product pickers read.

-- Typed attributes of a product (shared master data, like the products themselves).
CREATE TABLE products.product_attributes (
    product_id    uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    key           text NOT NULL CHECK (key ~ '^[A-Za-z_][A-Za-z0-9_]*$'),
    kind          text NOT NULL CHECK (kind IN ('number', 'text', 'boolean')),
    value_number  numeric,
    value_text    text,
    value_bool    boolean,
    unit          text,
    updated_at    timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (product_id, key),
    CONSTRAINT product_attributes_typed CHECK (
        (kind = 'number'  AND value_number IS NOT NULL AND value_text IS NULL AND value_bool IS NULL) OR
        (kind = 'text'    AND value_text   IS NOT NULL AND value_number IS NULL AND value_bool IS NULL) OR
        (kind = 'boolean' AND value_bool   IS NOT NULL AND value_number IS NULL AND value_text IS NULL))
);

CREATE SCHEMA configurator;

CREATE TABLE configurator.models (
    id            uuid PRIMARY KEY,
    company_id    uuid NOT NULL REFERENCES core.companies(id),
    code          text NOT NULL CHECK (btrim(code) <> ''),
    name          text NOT NULL CHECK (btrim(name) <> ''),
    description   text NOT NULL DEFAULT '',
    -- Names per language: {"nl": {"label": "..."}}.
    translations  jsonb NOT NULL DEFAULT '{}',
    -- The configurable product this model configures, if any.
    product_id    uuid REFERENCES products.products(id),
    active        boolean NOT NULL DEFAULT true,
    created_by    uuid REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, code)
);

CREATE TABLE configurator.model_revisions (
    id            uuid PRIMARY KEY,
    model_id      uuid NOT NULL REFERENCES configurator.models(id) ON DELETE CASCADE,
    revision      integer NOT NULL CHECK (revision > 0),
    status        text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'obsolete')),
    -- The definition document, exactly as exported and imported.
    definition    jsonb NOT NULL,
    notes         text NOT NULL DEFAULT '',
    created_by    uuid REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now(),
    activated_at  timestamptz,
    UNIQUE (model_id, revision)
);
CREATE UNIQUE INDEX model_revisions_one_active_idx ON configurator.model_revisions (model_id) WHERE status = 'active';
CREATE UNIQUE INDEX model_revisions_one_draft_idx ON configurator.model_revisions (model_id) WHERE status = 'draft';

-- A saved run of a configurator. It stays on its own revision.
CREATE TABLE configurator.configurations (
    id            uuid PRIMARY KEY,
    company_id    uuid NOT NULL REFERENCES core.companies(id),
    model_id      uuid NOT NULL REFERENCES configurator.models(id),
    revision_id   uuid NOT NULL REFERENCES configurator.model_revisions(id),
    customer_id   uuid REFERENCES crm.customers(id),
    name          text NOT NULL DEFAULT '',
    -- Field values per scope and pinned overrides.
    inputs        jsonb NOT NULL DEFAULT '{}',
    -- The evaluation when last saved or quoted (lines, issues), a snapshot for display.
    result        jsonb,
    -- The latest quotation revision made from it.
    quotation_id  uuid REFERENCES sales.quotations(id) ON DELETE SET NULL,
    created_by    uuid REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX configurations_company_idx ON configurator.configurations (company_id, updated_at DESC);
CREATE INDEX configurations_model_idx ON configurator.configurations (model_id);

-- The configurator module depends on products and sales (ADR 0034): where either is already
-- disabled, the configurator starts disabled too. Stored values only list what is DISABLED.
UPDATE core.companies
SET disabled_modules = array_append(disabled_modules, 'configurator')
WHERE ('products' = ANY (disabled_modules) OR 'sales' = ANY (disabled_modules))
  AND NOT ('configurator' = ANY (disabled_modules));

UPDATE core.user_companies
SET disabled_modules = array_append(disabled_modules, 'configurator')
WHERE ('products' = ANY (disabled_modules) OR 'sales' = ANY (disabled_modules))
  AND NOT ('configurator' = ANY (disabled_modules));
