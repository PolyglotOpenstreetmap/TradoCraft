-- Engineering: bills of materials with revisions, and serial-number traceability.
CREATE SCHEMA IF NOT EXISTS engineering;

ALTER TABLE products.products
    ADD COLUMN tracking text NOT NULL DEFAULT 'none' CHECK (tracking IN ('none', 'serial'));

-- A BOM revision of a product. Drafts are editable; active and obsolete revisions are frozen.
-- Components are products, so a component with its own active BOM is a subassembly.
CREATE TABLE engineering.boms (
    id            uuid PRIMARY KEY,
    product_id    uuid NOT NULL REFERENCES products.products(id),
    revision      integer NOT NULL CHECK (revision > 0),
    status        text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'obsolete')),
    notes         text NOT NULL DEFAULT '',
    created_by    uuid NOT NULL REFERENCES core.users(id),
    created_at    timestamptz NOT NULL DEFAULT now(),
    activated_at  timestamptz,
    UNIQUE (product_id, revision)
);
CREATE UNIQUE INDEX boms_one_active_idx ON engineering.boms (product_id) WHERE status = 'active';
CREATE UNIQUE INDEX boms_one_draft_idx ON engineering.boms (product_id) WHERE status = 'draft';

CREATE TABLE engineering.bom_lines (
    id                    uuid PRIMARY KEY,
    bom_id                uuid NOT NULL REFERENCES engineering.boms(id) ON DELETE CASCADE,
    position              integer NOT NULL,
    component_product_id  uuid NOT NULL REFERENCES products.products(id),
    quantity              numeric(18,4) NOT NULL CHECK (quantity > 0),
    scrap_percent         numeric(5,2) NOT NULL DEFAULT 0 CHECK (scrap_percent >= 0 AND scrap_percent < 100),
    notes                 text NOT NULL DEFAULT '',
    UNIQUE (bom_id, position)
);
CREATE INDEX bom_lines_component_idx ON engineering.bom_lines (component_product_id);

-- Individual serialised units ("as built"). Later created by production and goods receipt;
-- the basis for installed assets, repairs and warranties.
CREATE TABLE engineering.serial_units (
    id             uuid PRIMARY KEY,
    company_id     uuid NOT NULL REFERENCES core.companies(id),
    product_id     uuid NOT NULL REFERENCES products.products(id),
    serial_number  text NOT NULL,
    status         text NOT NULL DEFAULT 'in_stock'
                   CHECK (status IN ('in_stock', 'built_in', 'delivered', 'installed', 'scrapped')),
    bom_id         uuid REFERENCES engineering.boms(id),
    customer_id    uuid REFERENCES crm.customers(id),
    notes          text NOT NULL DEFAULT '',
    created_by     uuid NOT NULL REFERENCES core.users(id),
    created_at     timestamptz NOT NULL DEFAULT now(),
    updated_at     timestamptz NOT NULL DEFAULT now(),
    UNIQUE (product_id, serial_number)
);

-- Genealogy: which serialised component units were built into which unit.
-- Links are never deleted: removing a component (e.g. during a repair) sets removed_at,
-- so the full as-built and as-maintained history is kept.
CREATE TABLE engineering.serial_unit_components (
    id          uuid PRIMARY KEY,
    parent_id   uuid NOT NULL REFERENCES engineering.serial_units(id),
    child_id    uuid NOT NULL REFERENCES engineering.serial_units(id),
    added_by    uuid NOT NULL REFERENCES core.users(id),
    added_at    timestamptz NOT NULL DEFAULT now(),
    removed_by  uuid REFERENCES core.users(id),
    removed_at  timestamptz,
    CHECK (parent_id <> child_id)
);
-- A unit can be built into only one other unit at a time.
CREATE UNIQUE INDEX serial_unit_components_current_idx
    ON engineering.serial_unit_components (child_id) WHERE removed_at IS NULL;
CREATE INDEX serial_unit_components_parent_idx ON engineering.serial_unit_components (parent_id);
