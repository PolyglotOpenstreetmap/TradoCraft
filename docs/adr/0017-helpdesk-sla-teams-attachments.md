# 0017 — Helpdesk: SLA and escalation, teams, attachments, tags, merge, time and satisfaction

**Status:** Accepted

**Context.** The helpdesk (ADR 0013, 0014) handled e-mail, calls and the portal, but it had no
notion of "too late", no queues for groups of agents, no files on tickets, and no way to tidy up
duplicates. These are the gaps agents hit first.

**Decision.**
* **SLA policies** (`helpdesk.sla_policies`, one row per company and priority): minutes for the
  first response and for the resolution. Defaults: urgent 1 h / 8 h, high 4 h / 24 h, normal
  8 h / 72 h, low 24 h / 120 h. They are seeded for existing companies by the migration and
  created on first use for new ones. `GET/PUT /helpdesk/sla-policies` (permission
  `helpdesk.admin`); a change applies to open tickets.
* **Plain elapsed time.** Business hours and holidays are not modelled; the windows run around the
  clock. This is a known simplification (see "Not yet").
* **Due times** are stored on the ticket (`first_response_due_at`, `resolution_due_at`, plus the
  windows they were computed with) and always equal *creation + window + time spent waiting for
  the customer*. `sla::sync` recomputes them after any change that can matter (creation, priority,
  status); a priority change therefore recalculates from the creation time, so raising the
  priority of an old ticket can breach it at once. Internal tickets (production questions) have no
  SLA.
* **The clock stops** while the status is `pending_customer`: `sla_paused_at` is set, and on the
  next status change the waiting time is added to `sla_paused_seconds` and so to both due
  times. While stopped, a ticket is judged at the moment it stopped (it keeps the time it had
  left, or stays breached if it had already run out).
* **First response** is the first outgoing e-mail reply or outgoing call (`first_response_at`,
  already recorded that way); a ticket closed without any answer counts as answered when closed.
  The resolution is met when the ticket is closed. A target is *breached* when its due time is
  earlier than the moment it was met, or than now while still open; a late answer does not undo
  the breach. A target is *at risk* when it is open and less than 25 % of its window is left.
  The rules exist twice and are kept in step: the pure functions in `helpdesk::sla` (unit
  tested; used for the API response, `minutes_remaining` and the flags) and the same rules in SQL
  (`sla::sql`, used by the list filter `sla=breached|at_risk` and by the worker).
* **Escalation** runs in the worker loop (`sla::escalate`, each poll). An open, running ticket
  whose first response or resolution time has run out is claimed by one `UPDATE … RETURNING` that
  sets `first_response_breach_notified_at` / `resolution_breach_notified_at`, so it is escalated
  once per kind of breach, also with several workers. The claim, an internal e-mail per recipient
  in `core.mail_outbox` (sent by the existing system-mail loop, in the company's default language)
  and an audit event `sla_breached` are one transaction. Recipients: the owner and the team's
  leads; without an owner all members of the team; without either the company's administrators.
* **Teams** (`helpdesk.teams`, `team_members` with `is_lead`): a ticket has at most one team (the
  queue) and one owner (the person). `PUT /tickets/{id}/assignment` sets both; the ticket list
  filters on `team_id`, `mine` (mine, or unowned in one of my teams) and `unassigned` (no owner and
  no team). `helpdesk.settings` holds an optional default team: tickets from e-mail, the portal,
  the phone and manual entry start in it, otherwise they stay unassigned. Teams are retired
  (`active = false`), not deleted. Managing teams and settings needs the new permission
  `helpdesk.admin`, granted to the admin role only (there is no support lead role yet).
* **Attachments.** A file is uploaded to the ticket (`POST /tickets/{id}/attachments`, body = the
  file) and linked to the note, reply or portal message by passing its id in `attachment_ids`.
  Files are stored with `core.files` (ADR 0015): the type comes from the signature and only JPEG,
  PNG, WebP and **PDF** are accepted (the `core.files` check was widened), at most 12 MB, a cleaned
  file name, and a client-chosen id makes retries idempotent. Downloads are sent as attachments
  with `nosniff`. A file can be used once, only on the ticket it was uploaded to and, for the
  portal, only by the contact who uploaded it. Outgoing replies attach the files when the worker
  builds the mail (they are read from the file store at that moment; the single-attachment columns
  of `core.mail_outbox` are for system mail and are not used for tickets). Incoming mail
  attachments are stored the same way by the IMAP/inbound ingest; ones that are not acceptable are
  skipped (the full original stays in `raw_message`). **Portal visibility** follows the article:
  customers see files of `email_in`, `email_out` and `portal_in` articles only, so files on
  internal notes (and on calls) are "not found" for them; they may also attach files to their own
  messages.
* **Tags**: free-form, lower-cased, at most 40 characters, `POST/DELETE /tickets/{id}/tags[/{tag}]`,
  `GET /helpdesk/tags` for suggestions, list filter `tag`.
* **Merge** (`POST /tickets/{id}/merge`): the source is merged into a target of the same customer.
  Articles (with their files), tags and time entries move to the target, the earlier first
  response counts, the source is closed with a note, points at the target (`merged_into_id`), and
  both get a note and an audit event. It cannot be undone; a mail that still carries the old ticket
  number is threaded into the target, and the portal no longer lists the source.
* **Time entries** (`helpdesk.time_entries`: user, minutes 1–1440, note, billable): add, list and
  delete (own entries; an admin may delete any); the ticket shows the total.
* **Customer satisfaction (CSAT)**, off by default (`helpdesk.settings.csat_enabled`). When a
  ticket is closed, one survey per ticket is queued as an e-mail in the ticket's language (five
  languages) with a "good" and a "bad" link to `<portal_url>/csat#token=…&rating=…`. The token is
  256 random bits, stored as a SHA-256 hash, valid for 30 days and usable once. The page needs no
  login: `GET /csat/{token}` only reads (mail scanners that open links do not use it up) and
  `POST /csat/{token}` records the rating and an optional comment atomically (a second try is a
  409). The latest rating shows on the ticket.

**Consequences.**
* Tickets that existed before the migration have no due times until they are next changed; then
  they get them computed from their creation time and may be breached at once.
* Uploaded files that are never attached stay in the file store (no clean-up yet).
* The satisfaction page is part of the portal build and needs `portal_url` on the company.
* Escalation mails are plain system mail, not tickets; nothing threads back.

**Not yet.** Automation rules (triggers and macros), splitting a ticket, reporting (SLA
compliance, workload, CSAT trends), business hours and holidays in the SLA, per-customer SLAs,
and notification of the customer about the SLA.
