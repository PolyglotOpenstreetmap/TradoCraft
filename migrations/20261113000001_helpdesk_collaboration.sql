-- Helpdesk collaboration: escalation and hand-over, watchers, per-user unread, activity timeline
-- and soft presence.

-- A user's own language for system mail addressed to them; empty means the company's default.
ALTER TABLE core.users
    ADD COLUMN language char(2) CHECK (language IN ('nl', 'fr', 'en', 'de', 'es'));

-- Manual escalation marker (who and why live in the timeline event).
ALTER TABLE helpdesk.tickets
    ADD COLUMN escalated_at timestamptz,
    ADD COLUMN escalated_by uuid REFERENCES core.users(id);
CREATE INDEX tickets_escalated_idx ON helpdesk.tickets (company_id) WHERE escalated_at IS NOT NULL;

-- ---------------------------------------------------------------------------------------------
-- Activity timeline: what colleagues did to a ticket (messages are read from helpdesk.articles).
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.ticket_events (
    id               uuid PRIMARY KEY,
    company_id       uuid NOT NULL REFERENCES core.companies(id),
    ticket_id        uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    kind             text NOT NULL CHECK (kind IN
        ('created', 'assigned', 'delegated', 'escalated', 'status', 'priority', 'merged',
         'tag_added', 'tag_removed')),
    -- empty for the system (a mail or the customer portal created the ticket)
    actor_user_id    uuid REFERENCES core.users(id),
    from_value       text,
    to_value         text,
    previous_user_id uuid REFERENCES core.users(id),
    target_user_id   uuid REFERENCES core.users(id),
    target_team_id   uuid REFERENCES helpdesk.teams(id),
    -- internal: never shown to the customer
    comment          text NOT NULL DEFAULT '',
    created_at       timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX ticket_events_ticket_idx ON helpdesk.ticket_events (ticket_id, created_at);
CREATE INDEX ticket_events_company_idx ON helpdesk.ticket_events (company_id);

-- ---------------------------------------------------------------------------------------------
-- Watchers (followers) and per-user "last read".
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.ticket_watchers (
    ticket_id        uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    user_id          uuid NOT NULL REFERENCES core.users(id),
    company_id       uuid NOT NULL REFERENCES core.companies(id),
    created_at       timestamptz NOT NULL DEFAULT now(),
    -- throttle: at most one watcher mail per ticket and user per 10 minutes
    last_notified_at timestamptz,
    PRIMARY KEY (ticket_id, user_id)
);
CREATE INDEX ticket_watchers_user_idx ON helpdesk.ticket_watchers (user_id);

CREATE TABLE helpdesk.ticket_reads (
    ticket_id    uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    user_id      uuid NOT NULL REFERENCES core.users(id),
    company_id   uuid NOT NULL REFERENCES core.companies(id),
    last_read_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (ticket_id, user_id)
);
CREATE INDEX ticket_reads_user_idx ON helpdesk.ticket_reads (user_id);

-- ---------------------------------------------------------------------------------------------
-- Soft presence: advisory, refreshed by a heartbeat, cleaned by the worker.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE helpdesk.ticket_presence (
    ticket_id  uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    user_id    uuid NOT NULL REFERENCES core.users(id),
    company_id uuid NOT NULL REFERENCES core.companies(id),
    mode       text NOT NULL CHECK (mode IN ('viewing', 'replying')),
    seen_at    timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (ticket_id, user_id)
);
CREATE INDEX ticket_presence_seen_idx ON helpdesk.ticket_presence (seen_at);

-- ---------------------------------------------------------------------------------------------
-- Existing tickets: owners follow their tickets, and what exists today counts as read for the
-- people involved, so nobody starts with a wall of unread tickets.
-- ---------------------------------------------------------------------------------------------
INSERT INTO helpdesk.ticket_watchers (ticket_id, user_id, company_id)
SELECT t.id, t.owner_user_id, t.company_id FROM helpdesk.tickets t WHERE t.owner_user_id IS NOT NULL
ON CONFLICT DO NOTHING;
INSERT INTO helpdesk.ticket_reads (ticket_id, user_id, company_id)
SELECT t.id, t.owner_user_id, t.company_id FROM helpdesk.tickets t WHERE t.owner_user_id IS NOT NULL
ON CONFLICT DO NOTHING;
INSERT INTO helpdesk.ticket_reads (ticket_id, user_id, company_id)
SELECT t.id, m.user_id, t.company_id
FROM helpdesk.tickets t JOIN helpdesk.team_members m ON m.team_id = t.team_id
ON CONFLICT DO NOTHING;
