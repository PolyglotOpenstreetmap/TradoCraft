-- Helpdesk follow-up (ADR 0021): business hours for the SLA, automation rules, ticket split.

-- ---------------------------------------------------------------------------------------------
-- Business calendar: one per company. Absent or disabled: plain elapsed time, as before.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.business_calendars (
    company_id  uuid PRIMARY KEY REFERENCES core.companies(id),
    enabled     boolean NOT NULL DEFAULT false,
    time_zone   text NOT NULL DEFAULT 'Europe/Brussels',
    updated_at  timestamptz NOT NULL DEFAULT now()
);

-- One opening interval per ISO weekday (1 = Monday); a weekday without a row is closed.
CREATE TABLE helpdesk.business_hours (
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    weekday     smallint NOT NULL CHECK (weekday BETWEEN 1 AND 7),
    start_time  time NOT NULL,
    end_time    time NOT NULL,
    PRIMARY KEY (company_id, weekday),
    CHECK (end_time > start_time)
);

CREATE TABLE helpdesk.holidays (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    day         date NOT NULL,
    name        text NOT NULL CHECK (btrim(name) <> ''),
    UNIQUE (company_id, day)
);

-- Every time the SLA clock was stopped, so due times can be recomputed under any calendar.
-- Tickets that were paused before this migration only have the total in sla_paused_seconds.
CREATE TABLE helpdesk.sla_pauses (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    ticket_id   uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    started_at  timestamptz NOT NULL,
    ended_at    timestamptz,
    CHECK (ended_at IS NULL OR ended_at >= started_at)
);
CREATE INDEX sla_pauses_ticket_idx ON helpdesk.sla_pauses (ticket_id);
CREATE UNIQUE INDEX sla_pauses_open_idx ON helpdesk.sla_pauses (ticket_id) WHERE ended_at IS NULL;
INSERT INTO helpdesk.sla_pauses (id, company_id, ticket_id, started_at)
SELECT gen_random_uuid(), company_id, id, sla_paused_at FROM helpdesk.tickets WHERE sla_paused_at IS NOT NULL;

-- The moment a window is 75 % used (25 % left): "at risk" starts after it. Stored because with
-- business hours it cannot be derived in SQL.
ALTER TABLE helpdesk.tickets
    ADD COLUMN first_response_risk_at timestamptz,
    ADD COLUMN resolution_risk_at     timestamptz,
    ADD COLUMN channel text NOT NULL DEFAULT 'manual'
        CHECK (channel IN ('email', 'portal', 'phone', 'manual', 'internal')),
    ADD COLUMN split_from_id uuid REFERENCES helpdesk.tickets(id);
UPDATE helpdesk.tickets
SET first_response_risk_at = first_response_due_at - sla_first_response_minutes * interval '15 seconds',
    resolution_risk_at     = resolution_due_at - sla_resolution_minutes * interval '15 seconds'
WHERE first_response_due_at IS NOT NULL;
UPDATE helpdesk.tickets t SET channel = CASE
    WHEN t.internal THEN 'internal'
    ELSE coalesce((SELECT CASE a.kind WHEN 'email_in' THEN 'email' WHEN 'portal_in' THEN 'portal'
                                       WHEN 'call_in' THEN 'phone' WHEN 'call_out' THEN 'phone' ELSE 'manual' END
                   FROM helpdesk.articles a WHERE a.ticket_id = t.id ORDER BY a.created_at LIMIT 1), 'manual')
    END;

-- ---------------------------------------------------------------------------------------------
-- Automation rules
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.automation_rules (
    id             uuid PRIMARY KEY,
    company_id     uuid NOT NULL REFERENCES core.companies(id),
    name           text NOT NULL CHECK (btrim(name) <> ''),
    active         boolean NOT NULL DEFAULT true,
    position       integer NOT NULL,
    trigger        text NOT NULL CHECK (trigger IN ('created', 'updated', 'customer_replied')),
    conditions     jsonb NOT NULL DEFAULT '{}',
    actions        jsonb NOT NULL DEFAULT '[]',
    stop_on_match  boolean NOT NULL DEFAULT false,
    created_by     uuid REFERENCES core.users(id),
    created_at     timestamptz NOT NULL DEFAULT now(),
    updated_at     timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX automation_rules_company_idx ON helpdesk.automation_rules (company_id, position);
