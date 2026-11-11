# 0013 — Helpdesk: e-mail tickets, prepared answers, orders and five languages

**Status:** Accepted

**Context.** Customers e-mail questions, new requests (leads), quote confirmations and change
requests. Production sometimes has questions that the salesperson must ask the customer, in the
customer's language. Communication happens in Dutch, French, English, German and Spanish.
The design takes its inspiration from Zammad.

**Decision.**
* **E-mail → ticket.** The worker polls a mailbox over IMAP (`IMAP_*`) and stores each message
  before marking it read; `POST /helpdesk/inbound` imports a raw e-mail the same way. A reply is
  attached to its ticket through In-Reply-To/References, else through the `[Ticket#T2026-00012]`
  subject tag; anything else opens a new ticket. Duplicate Message-IDs are ignored. Quoted history
  is stripped for display; the raw message is kept.
* **Matching.** Sender → customer by exact e-mail, else by an unambiguous company domain (never
  for public mail providers). Language: the customer's, else detected from the text, else the
  company default. A quotation number in the mail links the quotation and routes the ticket to the
  salesperson who made it. A category (order confirmation, quote change, lead, …) is *suggested*;
  the agent decides.
* **Outbox.** Replies are stored as queued articles in the same transaction as the ticket change,
  and the worker sends them (SMTP, or `.eml` files in development) with Message-ID, In-Reply-To and
  References, retrying up to five times. Delivery status is shown on the article.
* **Prepared answers** have a text per language with `{{placeholders}}` (customer, contact,
  ticket, agent, quotation, order). They are rendered for a ticket in its language, falling back
  to English (the agent is warned), and inserted into the composer to be edited before sending.
* **Sales orders** are a frozen copy of a sent or accepted quotation revision, with the
  customer's reference. Converting a sent revision accepts it (and wins its opportunity). If the
  customer wants changes, a new revision is made first; superseded revisions cannot be converted.
* **Production questions** are internal tickets on an order, assigned to its salesperson, in the
  customer's language. Production (role `production`) can write internal notes but cannot e-mail
  customers (`helpdesk.reply`); the salesperson asks the customer and reports back in a note.
  Translation is done by the salesperson; no external translation service is used.
* **Languages.** The app UI is translated into NL/FR/EN/DE/ES (Flutter gen-l10n, ARB files in
  `app/lib/l10n`), with a language picker; customer documents (quotation PDF, prepared answers)
  exist per language. API error messages and system notes are English for now.

**Later.** Microsoft 365 (Graph) mailbox, attachments, SLA timers and escalation, a customer
portal, and e-mailing quotation PDFs straight from the ticket.
