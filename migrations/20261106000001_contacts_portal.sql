-- Contacts (people at customers), phone calls on tickets, and the customer portal.

-- ---------------------------------------------------------------------------------------------
-- Contacts: a person at a customer. Customers are shared across companies, so are contacts.
--
-- Language: crm.customers.language is the language of the company (customer); a contact may
-- speak another one. NULL means "same as the customer". How to address the person is stored as
-- title + address form (formal "u / vous / Sie" or informal "je / tu / du").
-- ---------------------------------------------------------------------------------------------
CREATE TABLE crm.contacts (
    id            uuid PRIMARY KEY,
    customer_id   uuid NOT NULL REFERENCES crm.customers(id),
    first_name    text NOT NULL DEFAULT '',
    last_name     text NOT NULL CHECK (btrim(last_name) <> ''),
    job_title     text NOT NULL DEFAULT '',
    title         text NOT NULL DEFAULT 'none' CHECK (title IN ('mr', 'ms', 'none')),
    address_form  text NOT NULL DEFAULT 'formal' CHECK (address_form IN ('formal', 'informal')),
    language      char(2) CHECK (language IN ('nl', 'fr', 'en', 'de', 'es')),
    -- what this person is for the customer; a person can have several roles
    roles         text[] NOT NULL DEFAULT '{}'
                  CHECK (roles <@ ARRAY['sales_contact', 'technical', 'purchasing', 'invoicing',
                                        'management', 'quality', 'other']::text[]),
    notes         text NOT NULL DEFAULT '',
    active        boolean NOT NULL DEFAULT true,
    -- customer portal
    portal_access boolean NOT NULL DEFAULT false,
    -- own: only the tickets this person raised; customer: all tickets of the customer
    portal_scope  text NOT NULL DEFAULT 'own' CHECK (portal_scope IN ('own', 'customer')),
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX contacts_customer_idx ON crm.contacts (customer_id);
CREATE INDEX contacts_name_idx ON crm.contacts (lower(last_name), lower(first_name));

-- An address identifies exactly one person (stored lower-case), so incoming mail can be matched.
CREATE TABLE crm.contact_emails (
    id          uuid PRIMARY KEY,
    contact_id  uuid NOT NULL REFERENCES crm.contacts(id) ON DELETE CASCADE,
    email       text NOT NULL CHECK (email = lower(email) AND email LIKE '%_@_%'),
    is_primary  boolean NOT NULL DEFAULT false
);
CREATE UNIQUE INDEX contact_emails_email_idx ON crm.contact_emails (email);
CREATE UNIQUE INDEX contact_emails_primary_idx ON crm.contact_emails (contact_id) WHERE is_primary;

-- `number` is what was typed; `normalized` is digits with country code (e.g. 32475123456) so a
-- caller id like +32 475 12 34 56 finds "0475/12.34.56".
CREATE TABLE crm.contact_phones (
    id          uuid PRIMARY KEY,
    contact_id  uuid NOT NULL REFERENCES crm.contacts(id) ON DELETE CASCADE,
    label       text NOT NULL DEFAULT 'work' CHECK (label IN ('work', 'mobile', 'direct', 'other')),
    number      text NOT NULL,
    normalized  text NOT NULL CHECK (normalized ~ '^[0-9]{6,15}$'),
    UNIQUE (contact_id, normalized)
);
CREATE INDEX contact_phones_normalized_idx ON crm.contact_phones (normalized);

-- ---------------------------------------------------------------------------------------------
-- Tickets know the person, and phone calls are logged as articles.
-- ---------------------------------------------------------------------------------------------
ALTER TABLE helpdesk.tickets ADD COLUMN contact_id uuid REFERENCES crm.contacts(id);
CREATE INDEX tickets_contact_idx ON helpdesk.tickets (contact_id);

ALTER TABLE helpdesk.articles DROP CONSTRAINT articles_kind_check;
ALTER TABLE helpdesk.articles ADD CONSTRAINT articles_kind_check
    CHECK (kind IN ('email_in', 'email_out', 'note', 'call_in', 'call_out', 'portal_in'));
ALTER TABLE helpdesk.articles
    ADD COLUMN contact_id       uuid REFERENCES crm.contacts(id),
    ADD COLUMN phone_number     text,
    ADD COLUMN duration_seconds integer CHECK (duration_seconds IS NULL OR duration_seconds >= 0);

-- ---------------------------------------------------------------------------------------------
-- System e-mail (portal login links): not tied to a ticket, sent by the worker like the outbox.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE core.mail_outbox (
    id            uuid PRIMARY KEY,
    company_id    uuid NOT NULL REFERENCES core.companies(id),
    to_address    text NOT NULL,
    subject       text NOT NULL,
    body          text NOT NULL,
    delivery      text NOT NULL DEFAULT 'queued' CHECK (delivery IN ('queued', 'sent', 'failed')),
    delivery_error text,
    attempts      integer NOT NULL DEFAULT 0,
    created_at    timestamptz NOT NULL DEFAULT now(),
    sent_at       timestamptz
);
CREATE INDEX mail_outbox_queue_idx ON core.mail_outbox (created_at) WHERE delivery = 'queued';

-- ---------------------------------------------------------------------------------------------
-- Customer portal: passwordless login. A single-use link is e-mailed to the contact; exchanging
-- it creates a session. Only SHA-256 hashes of the tokens are stored.
-- ---------------------------------------------------------------------------------------------
-- Where the portal lives for this company; used in the login e-mail.
ALTER TABLE core.companies ADD COLUMN portal_url text;

CREATE SCHEMA IF NOT EXISTS portal;

CREATE TABLE portal.login_links (
    id          uuid PRIMARY KEY,
    contact_id  uuid NOT NULL REFERENCES crm.contacts(id) ON DELETE CASCADE,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    token_hash  bytea NOT NULL UNIQUE,
    expires_at  timestamptz NOT NULL,
    used_at     timestamptz,
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX login_links_contact_idx ON portal.login_links (contact_id, created_at DESC);

CREATE TABLE portal.sessions (
    id          uuid PRIMARY KEY,
    contact_id  uuid NOT NULL REFERENCES crm.contacts(id) ON DELETE CASCADE,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    token_hash  bytea NOT NULL UNIQUE,
    expires_at  timestamptz NOT NULL,
    revoked_at  timestamptz,
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX portal_sessions_contact_idx ON portal.sessions (contact_id);
