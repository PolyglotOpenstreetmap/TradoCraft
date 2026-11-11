-- Options of a product (ADR 0037): products offered with it on a quotation. Separate from
-- products.product_relations, which holds compatibility and suggestion rules.
CREATE TABLE products.product_options (
    id                uuid PRIMARY KEY,
    parent_product_id uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    option_product_id uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    -- Quantity of the option per unit of the parent line.
    default_quantity  numeric(14,4) NOT NULL DEFAULT 1 CHECK (default_quantity > 0),
    -- A required option is added to the quotation automatically with its parent.
    required          boolean NOT NULL DEFAULT false,
    sort_order        integer NOT NULL DEFAULT 0,
    active            boolean NOT NULL DEFAULT true,
    created_at        timestamptz NOT NULL DEFAULT now(),
    updated_at        timestamptz NOT NULL DEFAULT now(),
    UNIQUE (parent_product_id, option_product_id),
    CHECK (parent_product_id <> option_product_id)
);
CREATE INDEX product_options_option_idx ON products.product_options (option_product_id);
