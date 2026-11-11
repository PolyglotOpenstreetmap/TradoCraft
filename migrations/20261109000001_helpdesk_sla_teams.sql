-- Helpdesk maturity: SLA, teams, attachments, tags, time entries, merge and satisfaction ratings.

-- ---------------------------------------------------------------------------------------------
-- Files may now also be PDF documents (ticket attachments).
-- ---------------------------------------------------------------------------------------------
ALTER TABLE core.files DROP CONSTRAINT files_content_type_check;
ALTER TABLE core.files ADD CONSTRAINT files_content_type_check
    CHECK (content_type IN ('image/jpeg', 'image/png', 'image/webp', 'application/pdf'));

-- ---------------------------------------------------------------------------------------------
-- Teams (queues) and per-company helpdesk settings.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.teams (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    name        text NOT NULL CHECK (btrim(name) <> ''),
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, name)
);
CREATE TABLE helpdesk.team_members (
    team_id  uuid NOT NULL REFERENCES helpdesk.teams(id) ON DELETE CASCADE,
    user_id  uuid NOT NULL REFERENCES core.users(id),
    is_lead  boolean NOT NULL DEFAULT false,
    PRIMARY KEY (team_id, user_id)
);
CREATE INDEX team_members_user_idx ON helpdesk.team_members (user_id);

-- One optional row per company; no row means the defaults (no default team, no CSAT).
CREATE TABLE helpdesk.settings (
    company_id       uuid PRIMARY KEY REFERENCES core.companies(id),
    default_team_id  uuid REFERENCES helpdesk.teams(id),
    csat_enabled     boolean NOT NULL DEFAULT false,
    updated_at       timestamptz NOT NULL DEFAULT now()
);

-- ---------------------------------------------------------------------------------------------
-- SLA policies per company and priority. Plain elapsed minutes (business hours are not modelled).
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.sla_policies (
    company_id             uuid NOT NULL REFERENCES core.companies(id),
    priority               text NOT NULL CHECK (priority IN ('low', 'normal', 'high', 'urgent')),
    first_response_minutes integer NOT NULL CHECK (first_response_minutes > 0),
    resolution_minutes     integer NOT NULL CHECK (resolution_minutes >= first_response_minutes),
    updated_at             timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (company_id, priority)
);
INSERT INTO helpdesk.sla_policies (company_id, priority, first_response_minutes, resolution_minutes)
SELECT c.id, p.priority, p.fr, p.res
FROM core.companies c
CROSS JOIN (VALUES ('urgent', 60, 480), ('high', 240, 1440), ('normal', 480, 4320), ('low', 1440, 7200))
     AS p(priority, fr, res);

ALTER TABLE helpdesk.tickets
    ADD COLUMN team_id                          uuid REFERENCES helpdesk.teams(id),
    -- The windows in force when the due times were last computed (for "at risk").
    ADD COLUMN sla_first_response_minutes       integer,
    ADD COLUMN sla_resolution_minutes           integer,
    ADD COLUMN first_response_due_at            timestamptz,
    ADD COLUMN resolution_due_at                timestamptz,
    -- The clock stops while the ticket waits for the customer.
    ADD COLUMN sla_paused_at                    timestamptz,
    ADD COLUMN sla_paused_seconds               bigint NOT NULL DEFAULT 0 CHECK (sla_paused_seconds >= 0),
    -- Escalation e-mails are sent once per kind of breach.
    ADD COLUMN first_response_breach_notified_at timestamptz,
    ADD COLUMN resolution_breach_notified_at     timestamptz,
    -- Set on the source of a merge; the ticket is closed and its content lives in the target.
    ADD COLUMN merged_into_id                   uuid REFERENCES helpdesk.tickets(id);
CREATE INDEX tickets_team_idx ON helpdesk.tickets (team_id);
CREATE INDEX tickets_sla_open_idx ON helpdesk.tickets (company_id)
    WHERE closed_at IS NULL AND NOT internal;

-- ---------------------------------------------------------------------------------------------
-- Tags
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.ticket_tags (
    ticket_id  uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    tag        text NOT NULL CHECK (btrim(tag) <> '' AND tag = lower(tag)),
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (ticket_id, tag)
);
CREATE INDEX ticket_tags_tag_idx ON helpdesk.ticket_tags (tag);

-- ---------------------------------------------------------------------------------------------
-- Time entries
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.time_entries (
    id         uuid PRIMARY KEY,
    ticket_id  uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    user_id    uuid NOT NULL REFERENCES core.users(id),
    minutes    integer NOT NULL CHECK (minutes > 0 AND minutes <= 1440),
    note       text NOT NULL DEFAULT '',
    billable   boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX time_entries_ticket_idx ON helpdesk.time_entries (ticket_id);

-- ---------------------------------------------------------------------------------------------
-- Attachments: a file is uploaded to the ticket first (article_id empty) and linked to the
-- article when the note, reply or portal message is written.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.ticket_attachments (
    file_id     uuid PRIMARY KEY REFERENCES core.files(id),
    ticket_id   uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    article_id  uuid REFERENCES helpdesk.articles(id) ON DELETE CASCADE,
    -- set when a customer (portal) uploaded it
    contact_id  uuid REFERENCES crm.contacts(id),
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX ticket_attachments_ticket_idx ON helpdesk.ticket_attachments (ticket_id);
CREATE INDEX ticket_attachments_article_idx ON helpdesk.ticket_attachments (article_id);

-- ---------------------------------------------------------------------------------------------
-- Customer satisfaction: one single-use token per survey, stored hashed.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.csat_surveys (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    ticket_id   uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    token_hash  bytea NOT NULL UNIQUE,
    created_at  timestamptz NOT NULL DEFAULT now(),
    expires_at  timestamptz NOT NULL,
    rating      text CHECK (rating IN ('good', 'bad')),
    comment     text NOT NULL DEFAULT '',
    rated_at    timestamptz,
    CHECK ((rating IS NULL) = (rated_at IS NULL))
);
CREATE INDEX csat_surveys_ticket_idx ON helpdesk.csat_surveys (ticket_id);
