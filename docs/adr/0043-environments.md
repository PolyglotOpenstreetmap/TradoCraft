# 0043 — Environments (live, test, tutorial)

**Status:** Accepted

**Context.** tradocraft has one database and one API instance, so everything a user does is live
data. A guided tutorial, training sessions and trying out a change all need a place where mistakes
cost nothing, and the user must never be unsure which data they are looking at. Customers and
products are global tables (no `company_id`, ADR 0003), so a second company in the same database
would not isolate practice data; a company-level switch would also leave e-mail polling and
sending running against real mailboxes.

**Decision.** An environment is one database served by one API instance. Nothing in the code
branches on "which environment" inside a request; the app talks to a different server instead.

*Instance.* Two settings are read at start-up: `APP_ENV_KIND` (`live`, `test` or `tutorial`;
default `live`) and `APP_ENV_NAME` (default: the name of the database). An unknown kind stops the
API with a clear error, so a typo cannot turn a test instance into a live-looking one or the
reverse. `GET /api/v1/meta/environment` returns `{name, kind, db_name, version}`. The database
name comes from `current_database()`; host, user and password are never exposed. The route needs
neither a sign-in nor a company: the login screen shows the environment too, and the document is
small and not sensitive.

*App.* The app keeps a list of environments (id, name, kind, server URL, and per environment the
dev user, token and company the `Session` already models) in the device storage. Switching
replaces the session, which rebuilds the HTTP client and the per-user storage (so the open
workspace tabs of one environment never show up in another). A session of one environment is
never reused for another: the stored session is per environment, and a new environment starts
signed out. The active environment is always visible in a banner: red for live, amber for test,
blue for tutorial, always with the kind written out (colour is not the only signal). When an
environment is added, its kind is taken from the server's answer, not typed by the user. The
single session that existing installations hold becomes the first environment, with the kind read
from the server when it is reachable and `live` otherwise. `--profile` keeps working: a profile
has its own storage and therefore its own list.

*Later phases (recorded here, not built yet).*

* A test environment is a clone of the live database, created by a CLI/ops script (`pg_dump |
  pg_restore` into a new database and a second API instance with its own `DATABASE_URL`, port
  and `APP_ENV_KIND=test`), not from the app. A runbook describes it.
* A new-company bootstrap wizard creates a company with its basics (units, VAT, warehouse,
  numbering, price zones) in an empty database, so a tutorial environment can start clean.
* For `kind != live` every outbound side effect must be disabled: the helpdesk IMAP poller, SMTP
  and any webhook, and stored mail credentials are cleared in a clone.

**Consequences.**

* Safety rests on the banner and on the ops script, not on permissions: nothing stops a user from
  adding the live server to the list on purpose.
* Until the ops script disables outbound mail, a clone made by hand must have its `IMAP_*` and
  `SMTP_*` settings removed from the worker by whoever makes it.
* Each instance runs its own migrations at start-up, so environments can be on different versions;
  `version` in the document makes that visible.
* With OIDC, the identity provider must accept the same audience for every environment, or the
  token is stored per environment (it is: sessions are per environment).
* The kind comes from the server and is trusted by the app; a wrong `APP_ENV_KIND` on the server
  is a wrong banner.
