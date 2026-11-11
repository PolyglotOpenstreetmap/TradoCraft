-- Languages used for customer communication: Dutch, French, English, German, Spanish.
UPDATE crm.customers SET language = 'en' WHERE language NOT IN ('nl', 'fr', 'en', 'de', 'es');
ALTER TABLE crm.customers ADD CONSTRAINT customers_language_check CHECK (language IN ('nl', 'fr', 'en', 'de', 'es'));
ALTER TABLE core.companies ADD COLUMN default_language char(2) NOT NULL DEFAULT 'nl'
    CHECK (default_language IN ('nl', 'fr', 'en', 'de', 'es'));

-- ---------------------------------------------------------------------------------------------
-- Sales orders: a frozen copy of an accepted quotation revision.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE sales.orders (
    id                  uuid PRIMARY KEY,
    company_id          uuid NOT NULL REFERENCES core.companies(id),
    number              text NOT NULL,
    status              text NOT NULL DEFAULT 'confirmed' CHECK (status IN ('confirmed', 'cancelled')),
    quotation_id        uuid NOT NULL UNIQUE REFERENCES sales.quotations(id),
    customer_id         uuid NOT NULL REFERENCES crm.customers(id),
    customer_name       text NOT NULL,
    customer_reference  text NOT NULL DEFAULT '',
    order_date          date NOT NULL,
    currency            char(3) NOT NULL,
    subtotal            numeric(18,2) NOT NULL,
    vat_total           numeric(18,2) NOT NULL,
    total               numeric(18,2) NOT NULL,
    salesperson_id      uuid NOT NULL REFERENCES core.users(id),
    notes               text NOT NULL DEFAULT '',
    created_by          uuid NOT NULL REFERENCES core.users(id),
    created_at          timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, number)
);
CREATE INDEX orders_customer_idx ON sales.orders (customer_id);

CREATE TABLE sales.order_lines (
    id                     uuid PRIMARY KEY,
    order_id               uuid NOT NULL REFERENCES sales.orders(id) ON DELETE CASCADE,
    position               integer NOT NULL,
    product_id             uuid REFERENCES products.products(id),
    product_code           text NOT NULL,
    description            text NOT NULL,
    unit                   text NOT NULL,
    quantity               numeric(18,4) NOT NULL,
    list_unit_price        numeric(18,4) NOT NULL,
    rebate_percent         numeric(5,2) NOT NULL,
    line_discount_percent  numeric(5,2) NOT NULL,
    net_unit_price         numeric(18,4) NOT NULL,
    line_total             numeric(18,2) NOT NULL,
    vat_rate               numeric(5,2) NOT NULL,
    UNIQUE (order_id, position)
);

-- ---------------------------------------------------------------------------------------------
-- Helpdesk
-- ---------------------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS helpdesk;

CREATE TABLE helpdesk.tickets (
    id                uuid PRIMARY KEY,
    company_id        uuid NOT NULL REFERENCES core.companies(id),
    number            text NOT NULL,
    subject           text NOT NULL,
    -- new: not yet handled; open: being handled; pending_customer: waiting for the customer;
    -- pending_internal: waiting for a colleague (e.g. production); closed
    status            text NOT NULL DEFAULT 'new'
                      CHECK (status IN ('new', 'open', 'pending_customer', 'pending_internal', 'closed')),
    priority          text NOT NULL DEFAULT 'normal' CHECK (priority IN ('low', 'normal', 'high', 'urgent')),
    category          text NOT NULL DEFAULT 'question'
                      CHECK (category IN ('question', 'lead', 'quote_change', 'order_confirmation',
                                          'production_question', 'service', 'complaint', 'other')),
    -- what the system guessed from an incoming e-mail; the agent decides
    suggested_category text,
    language          char(2) NOT NULL CHECK (language IN ('nl', 'fr', 'en', 'de', 'es')),
    customer_id       uuid REFERENCES crm.customers(id),
    contact_name      text NOT NULL DEFAULT '',
    contact_email     text NOT NULL DEFAULT '',
    owner_user_id     uuid REFERENCES core.users(id),
    quotation_id      uuid REFERENCES sales.quotations(id),
    opportunity_id    uuid REFERENCES crm.opportunities(id),
    order_id          uuid REFERENCES sales.orders(id),
    -- internal tickets (e.g. a production question) start without customer contact
    internal          boolean NOT NULL DEFAULT false,
    created_by        uuid REFERENCES core.users(id),
    created_at        timestamptz NOT NULL DEFAULT now(),
    updated_at        timestamptz NOT NULL DEFAULT now(),
    first_response_at timestamptz,
    closed_at         timestamptz,
    UNIQUE (company_id, number)
);
CREATE INDEX tickets_company_status_idx ON helpdesk.tickets (company_id, status, updated_at DESC);
CREATE INDEX tickets_owner_idx ON helpdesk.tickets (owner_user_id);
CREATE INDEX tickets_customer_idx ON helpdesk.tickets (customer_id);

-- Messages of a ticket: incoming and outgoing e-mails and internal notes.
-- Outgoing e-mails are queued here and sent by the worker (transactional outbox).
CREATE TABLE helpdesk.articles (
    id             uuid PRIMARY KEY,
    ticket_id      uuid NOT NULL REFERENCES helpdesk.tickets(id) ON DELETE CASCADE,
    kind           text NOT NULL CHECK (kind IN ('email_in', 'email_out', 'note')),
    from_address   text NOT NULL DEFAULT '',
    from_name      text NOT NULL DEFAULT '',
    to_addresses   text[] NOT NULL DEFAULT '{}',
    cc_addresses   text[] NOT NULL DEFAULT '{}',
    subject        text NOT NULL DEFAULT '',
    -- only what is new in the message (quoted history removed); the full original is in raw_message
    body           text NOT NULL,
    raw_message    bytea,
    language       char(2),
    message_id     text UNIQUE,
    in_reply_to    text,
    author_user_id uuid REFERENCES core.users(id),
    -- outgoing e-mail delivery: queued -> sent | failed
    delivery       text CHECK (delivery IN ('queued', 'sent', 'failed')),
    delivery_error text,
    attempts       integer NOT NULL DEFAULT 0,
    created_at     timestamptz NOT NULL DEFAULT now(),
    sent_at        timestamptz,
    CHECK ((kind = 'email_out') = (delivery IS NOT NULL))
);
CREATE INDEX articles_ticket_idx ON helpdesk.articles (ticket_id, created_at);
CREATE INDEX articles_outbox_idx ON helpdesk.articles (created_at) WHERE delivery = 'queued';

-- Prepared answers ("text modules") with one text per language.
CREATE TABLE helpdesk.prepared_answers (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    code        text NOT NULL,
    name        text NOT NULL,
    category    text,
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz NOT NULL DEFAULT now(),
    UNIQUE (company_id, code)
);

CREATE TABLE helpdesk.prepared_answer_texts (
    answer_id  uuid NOT NULL REFERENCES helpdesk.prepared_answers(id) ON DELETE CASCADE,
    language   char(2) NOT NULL CHECK (language IN ('nl', 'fr', 'en', 'de', 'es')),
    subject    text NOT NULL DEFAULT '',
    body       text NOT NULL,
    PRIMARY KEY (answer_id, language)
);
