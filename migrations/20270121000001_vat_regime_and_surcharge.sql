-- ADR 0038: VAT regime per customer, and negative general rebates (surcharges).
--   domestic     VAT is charged (end customer in the EU, the default: what existed before)
--   intra_eu     business customer in another EU country, VAT reverse-charged: no VAT on the quote
--   outside_eu   export: no VAT on the quote
ALTER TABLE crm.customers
    ADD COLUMN vat_regime text NOT NULL DEFAULT 'domestic'
        CHECK (vat_regime IN ('domestic', 'intra_eu', 'outside_eu'));

-- The regime is snapshotted on the quotation, like the VAT number.
ALTER TABLE sales.quotations
    ADD COLUMN vat_regime text NOT NULL DEFAULT 'domestic'
        CHECK (vat_regime IN ('domestic', 'intra_eu', 'outside_eu'));

-- A negative rebate raises the list price (-10 = 10 % more). Limited to -100..100.
ALTER TABLE crm.customer_terms DROP CONSTRAINT customer_terms_discount_percent_check;
ALTER TABLE crm.customer_terms
    ADD CONSTRAINT customer_terms_discount_percent_check
        CHECK (discount_percent >= -100 AND discount_percent <= 100);
ALTER TABLE crm.customer_rebates DROP CONSTRAINT customer_rebates_percent_check;
ALTER TABLE crm.customer_rebates
    ADD CONSTRAINT customer_rebates_percent_check CHECK (percent >= -100 AND percent <= 100);
