-- Purpose of a quotation / order (installation at a site, or goods for the customer's stock) and
-- the country the goods are delivered to. Orders inherit both from the quotation on conversion.

ALTER TABLE sales.quotations
    ADD COLUMN purpose text NOT NULL DEFAULT 'installation'
        CHECK (purpose IN ('installation', 'stock')),
    ADD COLUMN delivery_country_code char(2);

ALTER TABLE sales.orders
    ADD COLUMN purpose text NOT NULL DEFAULT 'installation'
        CHECK (purpose IN ('installation', 'stock')),
    ADD COLUMN delivery_country_code char(2);
