-- Configured quotations (ADR 0033): structured quotation lines (a parent line with option lines
-- below it, sections, a comment per line, prices as a percentage of the parent line), the
-- traceability of quotations imported from an external configurator, and product names per
-- language.

-- A parent line must belong to the same quotation: the composite foreign key below needs a unique
-- key on (quotation_id, id).
ALTER TABLE sales.quotation_lines
    ADD CONSTRAINT quotation_lines_quotation_line_key UNIQUE (quotation_id, id);

ALTER TABLE sales.quotation_lines
    ADD COLUMN parent_line_id    uuid,
    ADD COLUMN section           text,
    ADD COLUMN comment           text,
    -- Priced as a percentage of the parent line's unit price; base is the parent's list or net price.
    ADD COLUMN percent_of_parent numeric(7,3),
    ADD COLUMN percent_base      text
        CHECK (percent_base IN ('parent_list_unit_price', 'parent_net_unit_price')),
    ADD CONSTRAINT quotation_lines_parent_fk FOREIGN KEY (quotation_id, parent_line_id)
        REFERENCES sales.quotation_lines (quotation_id, id) ON DELETE CASCADE,
    ADD CONSTRAINT quotation_lines_parent_not_self CHECK (parent_line_id IS DISTINCT FROM id),
    ADD CONSTRAINT quotation_lines_percent_complete
        CHECK ((percent_of_parent IS NULL) = (percent_base IS NULL)),
    ADD CONSTRAINT quotation_lines_percent_needs_parent
        CHECK (percent_of_parent IS NULL OR parent_line_id IS NOT NULL);
CREATE INDEX quotation_lines_parent_idx ON sales.quotation_lines (parent_line_id)
    WHERE parent_line_id IS NOT NULL;

-- Where an imported quotation came from. Every revision carries the same source and reference;
-- re-importing the same (company, source, external_ref) updates the latest draft or revises it.
ALTER TABLE sales.quotations
    ADD COLUMN import_source       text,
    ADD COLUMN import_external_ref text,
    -- The configurator's own payload, stored verbatim for traceability (never interpreted).
    ADD COLUMN configuration       jsonb,
    ADD CONSTRAINT quotations_import_ref_complete
        CHECK ((import_source IS NULL) = (import_external_ref IS NULL));
CREATE INDEX quotations_import_ref_idx ON sales.quotations (company_id, import_source, import_external_ref, revision DESC)
    WHERE import_source IS NOT NULL;

-- Product names and descriptions per language (products are shared master data, like the products
-- table itself). A quotation line takes the name in the customer's language when it is priced and
-- falls back to the base name.
CREATE TABLE products.product_translations (
    product_id   uuid NOT NULL REFERENCES products.products(id) ON DELETE CASCADE,
    language     char(2) NOT NULL CHECK (language ~ '^[a-z]{2}$'),
    name         text NOT NULL CHECK (btrim(name) <> ''),
    description  text NOT NULL DEFAULT '',
    updated_at   timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (product_id, language)
);
