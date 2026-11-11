# 0023 — Helpdesk collaboration: escalation, hand-over, watchers, unread, timeline and presence

**Status:** Accepted

**Context.** ADR 0017 gave the helpdesk SLAs and teams, but colleagues still worked on a ticket
blind: there was no way to say "this needs more attention", to hand a ticket over with an
explanation, to follow a ticket one does not own, to see what has changed since one last looked,
or to notice that a colleague is answering the same customer at the same moment.

**Decision.** Migration `20261113000001_helpdesk_collaboration.sql`. All new tables carry
`company_id`; every query filters on the caller's company, so another company gets "not found".
Permissions are the existing ones: `helpdesk.read` to read the timeline, watch, mark read and
send presence; `helpdesk.write` to escalate, hand over and add other people as watchers.

* **Activity timeline** (`helpdesk.ticket_events`): kinds `created`, `assigned` (owner or team),
  `delegated`, `escalated`, `status`, `priority`, `merged`, `tag_added`, `tag_removed`, with actor
  (empty for the system, e.g. a mail opened the ticket), `from_value`/`to_value`, previous and
  target user, target team and an internal comment. The existing handlers write events
  (create, assignment, update, reply/note status changes, tags, merge, portal and mail intake).
  `GET /tickets/{id}/events` merges these with the ticket's messages (customer mails and portal
  messages, replies, internal notes, calls; the first 300 characters of the text), **oldest
  first**. It is internal: nothing of it reaches the customer or the portal.
* **Manual escalation** `POST /tickets/{id}/escalate {reason, to_user_id?|to_team_id?,
  raise_priority}`. The reason is required; a person and a team are mutually exclusive; a closed
  ticket cannot be escalated. It sets `tickets.escalated_at/escalated_by` (list filter
  `escalated=true`, badge in list and ticket), optionally raises the priority one step
  (low < normal < high < urgent, stops at urgent; the SLA is recalculated and a `priority` event
  is written) and writes an `escalated` event. The target is told by mail: the person, the leads
  of the chosen team (all members if it has no leads) or, without a target, the leads of the
  ticket's team, else its members, else the company's administrators. The caller is never mailed
  and becomes a watcher; an explicit target (person, or team leads) becomes a watcher too so the
  ticket shows up as unread for them. Escalating does **not** reassign the ticket or move it to
  another team.
* **Hand-over** `POST /tickets/{id}/delegate {to_user_id, comment?}` reuses the checks of
  `assign` (active member of the company) and sets the owner; the team is unchanged. The event
  keeps previous owner, new owner and the comment; the new owner gets a mail that includes the
  comment, except when taking the ticket over oneself. The caller and the previous owner become
  watchers. The comment is internal.
* **Watchers** (`helpdesk.ticket_watchers`): `PUT/DELETE /tickets/{id}/watch` (yourself),
  `POST /tickets/{id}/watchers {user_id}` (a colleague). Automatic watchers: the creator, the
  owner (also after an owner change), whoever replies or writes a note, delegators, escalators
  and their targets. Unwatching is honoured until the next time one replies. The ticket detail
  lists watchers (`watchers`). Existing owners were backfilled.
* **Watcher mail**: when a customer replies (mail or portal) or the owner changes, watchers get
  a mail in their language. At most **one mail per watcher and ticket per 10 minutes**
  (`ticket_watchers.last_notified_at`, claimed with one `UPDATE … RETURNING` in the same
  transaction as the mail), never to the author of the change. The new owner after a hand-over
  gets the hand-over mail instead. Mail language is the user's own (`core.users.language`, new,
  nullable) else the company's default language; there is no screen to set it yet.
* **Unread** (`helpdesk.ticket_reads`, `POST /tickets/{id}/read`; the app calls it after loading
  the ticket). The rule, in one place (`collab::unread_sql`): *a ticket is unread for a user if
  the user is involved and, after the user's `last_read_at` (never read = from the beginning),
  somebody else did something.* **Involved** means owner, member of the ticket's team or
  watcher. **Something** means a message (customer or colleague, note, reply, call) or a
  timeline event (assignment, status, priority, escalation, tag, merge) whose author is not the
  user. So a new ticket that nobody opened is unread for its owner, team and watchers only,
  never for everyone; own actions never make a ticket unread for oneself; users who are not
  involved never see the marker. Customer replies count (no author). The list returns `unread`
  and `watching` per ticket for the caller (also `GET /tickets/{id}`; other endpoints that
  return a ticket detail leave both false) and filters `unread=true`, `watching=true`,
  `escalated=true`. The migration marks existing tickets as read for their owner and team
  members.
* **Seen by**: the ticket detail returns `readers` (user, `last_read_at`, most recent first).
* **Soft presence** (`helpdesk.ticket_presence`, one row per ticket and user): the open ticket
  screen sends `POST /tickets/{id}/presence {mode: viewing|replying}` every ~20 s and at once
  when the reply box gets (or loses) text; the answer is the list of the others, so one call per
  beat is enough. `GET /tickets/{id}/presence` returns the same without a beat. Others means
  rows from other users seen within the last 60 s; older rows are ignored by the queries and
  deleted by the worker loop (`collab::cleanup_presence`, rows older than 5 minutes). The
  screen shows "Jo is replying — coordinate before sending" and, when sending a customer reply
  while somebody else is `replying`, asks for confirmation (it asks the server again at that
  moment).

**Consequences.**
* **Presence is advisory, polling-based and per company.** Nothing is locked and nothing
  prevents two replies; there is no push (no WebSocket), so a banner can be up to ~20 s late and
  a crashed client lingers for up to 60 s. Users only see colleagues of their own company.
* Timeline events exist from this migration on; older tickets show only their messages. The
  "assigned" event records an owner or team change, not a change of customer, links or
  language.
* Status events are written for every real change, including the automatic "waiting for
  customer" after a reply; actions of the system (SLA sync, customer reopening a ticket) write
  no status event, the customer message itself is the activity.
* Unread is computed with sub-selects per ticket; fine for a list page of 50, to be revisited
  (denormalised last-activity columns) if lists get large.
* Watcher mail and escalation mail are plain system mail like the SLA escalation: nothing
  threads back to the ticket.

**Not yet.** A per-user language setting, notification preferences (opt out per event kind),
in-app notifications, mentions in notes, escalation levels/rules, unwatching a colleague,
presence over a push channel.
