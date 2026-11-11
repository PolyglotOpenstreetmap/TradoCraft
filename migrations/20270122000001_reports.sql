-- User-defined reports (ADR 0042). A report is a stored definition (object, columns, filters,
-- grouping, summaries, chart) that is turned into a whitelisted query when it is run.
CREATE SCHEMA IF NOT EXISTS reports;

CREATE TABLE reports.reports (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    owner_id    uuid NOT NULL REFERENCES core.users(id),
    name        text NOT NULL CHECK (btrim(name) <> ''),
    description text NOT NULL DEFAULT '',
    object      text NOT NULL CHECK (object IN ('customer', 'quotation', 'order')),
    definition  jsonb NOT NULL,
    -- false: only the owner sees it; true: everyone in the company who may run reports
    shared      boolean NOT NULL DEFAULT false,
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX reports_company_idx ON reports.reports (company_id, shared, owner_id);
