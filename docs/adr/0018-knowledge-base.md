# 0018 — Knowledge base

**Status:** Accepted

**Context.** Agents kept retyping the same explanations, customers opened tickets for questions
that were already answered, and prepared answers (ADR 0013) are replies, not documentation. The
helpdesk needs articles that staff, logged-in customers and, where wanted, the public can read, in
the five languages of the product.

**Decision.**
* **A crate of its own** (`crates/kb`, schema `kb`, migration `20261110000001_knowledge_base`). The
  table for articles is `kb.entries`: `helpdesk.articles` already means ticket messages.
* **Categories** (`kb.categories`, `kb.category_names`): a name per language (English, then any,
  as fallback), a sort order, an `active` flag and one level of nesting only (a sub-category cannot
  have children and a category with children cannot become one). A category is deleted only when
  unused; otherwise it is deactivated.
* **Entries** (`kb.entries`, `kb.entry_texts`): numbered `KB-00001` per company, a slug made from
  the English (else first) title, an *audience* (`visibility`: `internal` staff only, `portal`
  logged-in customers too, `public` everyone) and a *status* (`draft`, `published`, `archived`).
  The text exists per language (`nl fr en de es`) with title, summary and body in simple Markdown.
  Readers get the wanted language, else English, else the first one in the usual order, and are
  told when it is a fallback (same rule as prepared answers). Tags are lower-case, at most 40
  characters; entries can link to related entries, products (`products.products`) and ticket
  categories (entries linked to the category of a ticket rank higher in its suggestions).
* **Markdown** is deliberately small: headings, paragraphs, bold, italics, code, lists, quotes,
  links and pictures. The server stores it as typed; it never renders HTML. Clients show it
  without interpreting raw HTML, only open `http`, `https` and `mailto` links, and load only
  pictures that belong to the article, written `![description](image:<file id>)`.
* **Search** uses PostgreSQL full text, one configuration per language (`dutch`, `french`,
  `english`, `german`, `spanish`; `kb.config`). `kb.entry_texts.search` is a generated column
  (`kb.vector`: title weight A, summary B, body C) with a GIN index. For each entry the *effective*
  text is chosen first (wanted language, else English, else any) and the query is parsed with that
  text's configuration, so Dutch words are stemmed as Dutch and an entry without a Dutch text is
  still found, in English, and flagged as fallback. Results are ranked with `ts_rank_cd`; each hit
  has a plain-text snippet from `ts_headline` with the matched words wrapped in U+E000 and U+E001
  (private-use characters, so they cannot clash with text; clients split on them). The search
  syntax of users is `websearch_to_tsquery` (words, "phrases", `-excluded`). Suggestions
  (ticket subject and customer message, the portal subject field) instead match *any* of up to 14 words (8 in the
  portal), taken from letters and digits only, which cannot produce a query syntax error.
  Because the configuration depends on the row, the GIN index helps the planner only after the
  entry set has been narrowed; at the size of a knowledge base that is fine, and a per-language
  `OR` form can be added if a company grows past tens of thousands of entries.
* **Versions** (`kb.entry_versions`): an immutable snapshot (a trigger refuses `UPDATE`) of all
  texts, with editor and time. One is saved on publishing and whenever the texts of a *published*
  entry change (a change to tags, category or audience alone does not). Drafts are not versioned.
  *Restore* puts the texts of an old version back and saves them as a new version
  (`restored_from`); history is never rewritten.
* **Permissions.** `kb.read` (all entries incl. internal and drafts; support, product manager,
  sales, sales manager, production, viewer and admin), `kb.write` (create and edit, upload
  images, restore; support, product manager, admin) and `kb.admin` (categories, publish, unpublish,
  archive, and changing the audience of a *published* entry; admin only, as there is no support
  lead role yet). Publishing is separate because it decides what customers and the public read.
* **Staff API** (`/kb/...`): categories, entries (list/search with filters `q`, `category_id` incl.
  sub-categories, `tag`, `language`, `visibility`, `status`), create/update, publish/unpublish/
  archive, versions and restore, tags, images, and `GET /tickets/{id}/kb-suggestions` (published
  entries, internal ones included, matching the subject and the latest incoming message in the
  ticket's language) and `GET /tickets/{id}/kb-entries/{entry_id}/render` (title, plain text and,
  for entries customers can open, a portal link `<portal_url>/help/<id>`; drafts and archived
  entries cannot be inserted). Every query filters on `company_id`; an entry of another company is
  "not found".
* **Images** use `core.files`: JPEG, PNG and WebP only (checked by signature, at most 12 MB, 30 per
  entry, client-chosen ids make retries safe). They are downloaded through the entry, so the
  entry's rules apply: staff always, customers only for published portal/public entries, the
  public only for published public ones; everything else is "not found". Deleting an image removes
  the file.
* **Portal** (`/portal/kb/...`, portal session): categories (only those with readable entries),
  browse/search, one entry (counts one view per request), `PUT .../vote {helpful}`, images and
  `GET /portal/kb/suggestions?q=` (at most three entries, for the new-ticket form). The language is
  the `language` parameter, else the contact's, else the customer's. Internal, draft and archived
  entries are "not found". A vote is one row per contact and entry (`kb.votes`) that can be changed;
  the counters on the entry are recounted from the votes, so a changed vote never counts twice.
* **Public** (`/public/kb/{company_id}/categories|entries|entries/{id}|entries/{id}/images/{file}`,
  no login): the company is named in the path, as the portal login does with `company_id` in its
  body (the portal build knows it as `PORTAL_COMPANY_ID`). Only published public entries are
  visible; responses carry `Cache-Control: public, max-age=300`; the payload has no counters or
  status, and reading counts one view (cached responses count fewer). An unknown company gives an
  empty list, not an error.
* **Deflection.** The portal's new-ticket form asks for suggestions 400 ms after the contact stops
  typing the subject (not before three characters) and shows up to three entries. A failure is
  swallowed; creating the ticket is never blocked.
* **Flutter.** Staff: *Knowledge base* in the main navigation (`kb.read`) with search and filters
  (category, status, audience, language), the editor (tab per language, write/preview, category,
  audience, tags, related articles, image upload and "insert in text", publish/unpublish/archive,
  version history with restore), a categories page, and on the ticket a *Suggested articles* panel
  and *Insert article*, which put the title, text and link (or only a link) in the reply in the
  ticket's language. Portal: *Help* with search, categories, article page (own Markdown renderer,
  no new package), helpful yes/no and the suggestions under the subject.

**Consequences.**
* Counters are plain integers (views, votes); there is no per-day history.
* Articles are always public to anyone with the id when their audience is `public`; there is no
  link expiry or preview-token for drafts.
* `<portal_url>/help/<id>` needs a portal login; the public API is for a company website.
* Deleting an entry is not offered; archive it.
* The ticket suggestion is a plain full-text match, not a learned one.

**Not yet.** Attachments other than images, comments and free-text ratings, analytics (searches
without results, most read, deflection rate), semantic/AI search, a translation workflow
(missing-language reports, machine translation, review status), SEO export (sitemap, static
pages), expiry/review dates, per-customer or per-product audiences, and a public web page in the
Flutter portal build.
