-- Product relation rules and computed co-occurrence (ADR 0026).

-- Manual rules. Each side is EITHER one product OR a whole family. Master data, shared by all
-- companies, like the products themselves.
CREATE TABLE products.product_relations (
    id            uuid PRIMARY KEY,
    kind          text NOT NULL CHECK (kind IN ('compatible', 'incompatible', 'suggested')),
    a_product_id  uuid REFERENCES products.products(id) ON DELETE CASCADE,
    a_family_id   uuid REFERENCES products.product_families(id) ON DELETE CASCADE,
    b_product_id  uuid REFERENCES products.products(id) ON DELETE CASCADE,
    b_family_id   uuid REFERENCES products.product_families(id) ON DELETE CASCADE,
    -- true: the rule also applies with the sides swapped
    bidirectional boolean NOT NULL DEFAULT true,
    -- weight of a `suggested` rule (1 weak .. 100 strong)
    strength      integer NOT NULL DEFAULT 50 CHECK (strength BETWEEN 1 AND 100),
    note          text NOT NULL DEFAULT '',
    active        boolean NOT NULL DEFAULT true,
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now(),
    CHECK ((a_product_id IS NOT NULL) <> (a_family_id IS NOT NULL)),
    CHECK ((b_product_id IS NOT NULL) <> (b_family_id IS NOT NULL)),
    -- a product is not related to itself
    CHECK (NOT (a_product_id = b_product_id))
);
-- The same rule (kind and both sides in this orientation) exists once. The mirrored duplicate is
-- refused by the API, which also looks at the reverse orientation.
CREATE UNIQUE INDEX product_relations_unique_idx ON products.product_relations (
    kind,
    COALESCE(a_product_id, '00000000-0000-0000-0000-000000000000'),
    COALESCE(a_family_id,  '00000000-0000-0000-0000-000000000000'),
    COALESCE(b_product_id, '00000000-0000-0000-0000-000000000000'),
    COALESCE(b_family_id,  '00000000-0000-0000-0000-000000000000'));
CREATE INDEX product_relations_a_product_idx ON products.product_relations (a_product_id) WHERE a_product_id IS NOT NULL;
CREATE INDEX product_relations_b_product_idx ON products.product_relations (b_product_id) WHERE b_product_id IS NOT NULL;
CREATE INDEX product_relations_a_family_idx ON products.product_relations (a_family_id) WHERE a_family_id IS NOT NULL;
CREATE INDEX product_relations_b_family_idx ON products.product_relations (b_family_id) WHERE b_family_id IS NOT NULL;

-- Computed "often bought together", filled by the worker from confirmed installation orders of
-- all companies. One row per unordered pair, stored with product_a_id < product_b_id.
CREATE TABLE products.product_cooccurrence (
    product_a_id    uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    product_b_id    uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    orders_together integer NOT NULL CHECK (orders_together >= 1),
    orders_with_a   integer NOT NULL,
    orders_with_b   integer NOT NULL,
    computed_at     timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (product_a_id, product_b_id),
    CHECK (product_a_id < product_b_id),
    CHECK (orders_together <= orders_with_a AND orders_together <= orders_with_b)
);
CREATE INDEX product_cooccurrence_b_idx ON products.product_cooccurrence (product_b_id);
