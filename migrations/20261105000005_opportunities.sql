-- CRM opportunities (sales pipeline). Quotations can be linked to an opportunity.
CREATE TABLE crm.opportunities (
    id               uuid PRIMARY KEY,
    company_id       uuid NOT NULL REFERENCES core.companies(id),
    number           text NOT NULL,
    title            text NOT NULL,
    customer_id      uuid NOT NULL REFERENCES crm.customers(id),
    owner_user_id    uuid NOT NULL REFERENCES core.users(id),
    stage            text NOT NULL DEFAULT 'lead'
                     CHECK (stage IN ('lead', 'qualified', 'proposal', 'negotiation', 'won', 'lost')),
    expected_value   numeric(18,2) NOT NULL DEFAULT 0 CHECK (expected_value >= 0),
    probability      smallint NOT NULL DEFAULT 10 CHECK (probability BETWEEN 0 AND 100),
    expected_close   date,
    lost_reason      text,
    notes            text NOT NULL DEFAULT '',
    created_at       timestamptz NOT NULL DEFAULT now(),
    updated_at       timestamptz NOT NULL DEFAULT now(),
    closed_at        timestamptz,
    UNIQUE (company_id, number),
    CHECK (stage <> 'lost' OR coalesce(lost_reason, '') <> '')
);
CREATE INDEX opportunities_company_stage_idx ON crm.opportunities (company_id, stage);
CREATE INDEX opportunities_customer_idx ON crm.opportunities (customer_id);

ALTER TABLE sales.quotations ADD COLUMN opportunity_id uuid REFERENCES crm.opportunities(id);
CREATE INDEX quotations_opportunity_idx ON sales.quotations (opportunity_id);
