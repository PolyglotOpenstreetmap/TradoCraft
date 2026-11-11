-- Applied by `envctl clone` to every clone, always (ADR 0048). Runs inside one transaction.
-- Clears everything that could let the clone act as, or log in as, someone in the live system, and
-- cancels mail that was still waiting to be sent. Mailbox (IMAP) and SMTP credentials are not in
-- the database at all: they are environment variables of the worker, which the outbound policy
-- overrides outside live.
BEGIN;

-- Customer portal: login links and sessions copied from live must not work in the clone.
DELETE FROM portal.login_links;
DELETE FROM portal.sessions;
-- Open satisfaction-survey links of live customers.
DELETE FROM helpdesk.csat_surveys;

-- Identity-provider bindings: the first OIDC sign-in in the clone binds again by e-mail address.
UPDATE core.users SET oidc_subject = NULL;

-- Mail that was still waiting to be sent in live must never be sent from the clone.
UPDATE core.mail_outbox
   SET delivery = 'failed', delivery_error = 'cancelled: environment clone'
 WHERE delivery = 'queued';
UPDATE helpdesk.articles
   SET delivery = 'failed', delivery_error = 'cancelled: environment clone'
 WHERE delivery = 'queued';

COMMIT;
