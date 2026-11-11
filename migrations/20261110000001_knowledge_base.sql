-- Knowledge base: categories, entries with one text per language, full-text search, versions,
-- tags, related entries, links to products and ticket categories, images and helpful votes.
CREATE SCHEMA IF NOT EXISTS kb;

-- ---------------------------------------------------------------------------------------------
-- Full-text search helpers. One Postgres text search configuration per language.
-- ---------------------------------------------------------------------------------------------
CREATE FUNCTION kb.config(lang text) RETURNS regconfig
LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$
    SELECT CASE lang
        WHEN 'nl' THEN 'dutch'::regconfig
        WHEN 'fr' THEN 'french'::regconfig
        WHEN 'de' THEN 'german'::regconfig
        WHEN 'es' THEN 'spanish'::regconfig
        ELSE 'english'::regconfig
    END
$$;

-- Title counts most, then the summary, then the body.
CREATE FUNCTION kb.vector(lang text, title text, summary text, body text) RETURNS tsvector
LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$
    SELECT setweight(to_tsvector(kb.config(lang), coalesce(title, '')), 'A')
        || setweight(to_tsvector(kb.config(lang), coalesce(summary, '')), 'B')
        || setweight(to_tsvector(kb.config(lang), coalesce(body, '')), 'C')
$$;

-- `any_word` = false: web-search syntax (all words); true: `q` is a ready-made `a | b | c`.
CREATE FUNCTION kb.query(lang text, q text, any_word boolean) RETURNS tsquery
LANGUAGE sql STABLE PARALLEL SAFE AS $$
    SELECT CASE WHEN any_word THEN to_tsquery(kb.config(lang), q)
                ELSE websearch_to_tsquery(kb.config(lang), q) END
$$;

-- ---------------------------------------------------------------------------------------------
-- Categories: one level of nesting, a name per language.
-- ---------------------------------------------------------------------------------------------
CREATE TABLE kb.categories (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    parent_id   uuid REFERENCES kb.categories(id),
    sort_order  integer NOT NULL DEFAULT 0,
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now(),
    CHECK (parent_id IS DISTINCT FROM id)
);
CREATE INDEX kb_categories_company_idx ON kb.categories (company_id, parent_id);

CREATE TABLE kb.category_names (
    category_id  uuid NOT NULL REFERENCES kb.categories(id) ON DELETE CASCADE,
    language     char(2) NOT NULL CHECK (language IN ('nl', 'fr', 'en', 'de', 'es')),
    name         text NOT NULL CHECK (btrim(name) <> ''),
    PRIMARY KEY (category_id, language)
);

-- ---------------------------------------------------------------------------------------------
-- Entries. (Not "articles": that is the name of the ticket message table.)
-- ---------------------------------------------------------------------------------------------
CREATE TABLE kb.entries (
    id           uuid PRIMARY KEY,
    company_id   uuid NOT NULL REFERENCES core.companies(id),
    number       text NOT NULL,
    slug         text NOT NULL,
    category_id  uuid REFERENCES kb.categories(id),
    -- internal: staff only; portal: logged-in customers too; public: anyone
    visibility   text NOT NULL DEFAULT 'internal' CHECK (visibility IN ('internal', 'portal', 'public')),
    status       text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    author_id    uuid REFERENCES core.users(id),
    published_at timestamptz,
    created_at   timestamptz NOT NULL DEFAULT now(),
    updated_at   timestamptz NOT NULL DEFAULT now(),
    view_count   bigint NOT NULL DEFAULT 0,
    helpful_yes  integer NOT NULL DEFAULT 0,
    helpful_no   integer NOT NULL DEFAULT 0,
    UNIQUE (company_id, number),
    UNIQUE (company_id, slug)
);
CREATE INDEX kb_entries_category_idx ON kb.entries (category_id);
CREATE INDEX kb_entries_listing_idx ON kb.entries (company_id, status, visibility);

CREATE TABLE kb.entry_texts (
    entry_id  uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    language  char(2) NOT NULL CHECK (language IN ('nl', 'fr', 'en', 'de', 'es')),
    title     text NOT NULL CHECK (btrim(title) <> ''),
    summary   text NOT NULL DEFAULT '',
    -- simple Markdown
    body      text NOT NULL DEFAULT '',
    search    tsvector GENERATED ALWAYS AS (kb.vector(language::text, title, summary, body)) STORED,
    PRIMARY KEY (entry_id, language)
);
CREATE INDEX kb_entry_texts_search_idx ON kb.entry_texts USING gin (search);

-- Immutable snapshots of the texts, one per published change; restoring writes a new one.
CREATE TABLE kb.entry_versions (
    id             uuid PRIMARY KEY,
    entry_id       uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    version        integer NOT NULL,
    editor_id      uuid REFERENCES core.users(id),
    edited_at      timestamptz NOT NULL DEFAULT now(),
    restored_from  integer,
    texts          jsonb NOT NULL,
    UNIQUE (entry_id, version)
);
CREATE FUNCTION kb.versions_are_immutable() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'knowledge base versions cannot be changed';
END
$$;
CREATE TRIGGER entry_versions_immutable BEFORE UPDATE ON kb.entry_versions
    FOR EACH ROW EXECUTE FUNCTION kb.versions_are_immutable();

CREATE TABLE kb.entry_tags (
    entry_id  uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    tag       text NOT NULL CHECK (tag = lower(tag) AND btrim(tag) <> '' AND length(tag) <= 40),
    PRIMARY KEY (entry_id, tag)
);
CREATE INDEX kb_entry_tags_tag_idx ON kb.entry_tags (tag);

CREATE TABLE kb.entry_links (
    entry_id    uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    related_id  uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    PRIMARY KEY (entry_id, related_id),
    CHECK (entry_id <> related_id)
);

CREATE TABLE kb.entry_products (
    entry_id    uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    product_id  uuid NOT NULL REFERENCES products.products(id),
    PRIMARY KEY (entry_id, product_id)
);

CREATE TABLE kb.entry_ticket_categories (
    entry_id         uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    ticket_category  text NOT NULL CHECK (ticket_category IN
        ('question', 'lead', 'quote_change', 'order_confirmation', 'production_question',
         'service', 'complaint', 'other')),
    PRIMARY KEY (entry_id, ticket_category)
);

-- Images used in the text (`![alt](image:<file id>)`), stored with core.files.
CREATE TABLE kb.entry_images (
    file_id     uuid PRIMARY KEY REFERENCES core.files(id),
    entry_id    uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX kb_entry_images_entry_idx ON kb.entry_images (entry_id);

-- One vote per customer contact and entry; it can be changed.
CREATE TABLE kb.votes (
    entry_id    uuid NOT NULL REFERENCES kb.entries(id) ON DELETE CASCADE,
    contact_id  uuid NOT NULL REFERENCES crm.contacts(id),
    helpful     boolean NOT NULL,
    updated_at  timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (entry_id, contact_id)
);
