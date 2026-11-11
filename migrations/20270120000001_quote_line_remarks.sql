-- Remarks on quotation and order lines (ADR 0037). The customer remark is printed on the document;
-- the production remark is internal and never shown to the customer. A product can carry a default
-- production remark that new quotation lines start with.
ALTER TABLE sales.quotation_lines
    ADD COLUMN production_remark text NOT NULL DEFAULT '',
    ADD COLUMN customer_remark   text NOT NULL DEFAULT '';

ALTER TABLE sales.order_lines
    ADD COLUMN production_remark text NOT NULL DEFAULT '',
    ADD COLUMN customer_remark   text NOT NULL DEFAULT '';

ALTER TABLE products.products
    ADD COLUMN default_production_remark text NOT NULL DEFAULT '';

-- The remark history looks up the texts used earlier for one product.
CREATE INDEX quotation_lines_product_idx ON sales.quotation_lines (product_id);
