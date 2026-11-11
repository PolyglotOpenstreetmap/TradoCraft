#!/usr/bin/env bash
# Smoke test for scripts/envctl (ADR 0048): clone a database, boot the clone, check kind and name,
# scrubbed credentials, disabled outbound mail/IMAP, isolation from the source, anonymize, drop.
#
# Needs a built API and worker (cargo build -p tradocraft-api -p tradocraft-worker) and
# psql/pg_dump/pg_restore/curl/jq. PostgreSQL comes from one of:
#   SMOKE_DATABASE_URL  an existing server (a superuser-like URL, e.g. postgresql://u:p@host:5432/postgres)
#   Docker              otherwise a throwaway postgres:16 container on SMOKE_PG_PORT (default 55441)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENVCTL="$ROOT/scripts/envctl"
WORK="$(mktemp -d)"
CONTAINER="tradocraft-envctl-smoke"
FAILS=0

pass() { echo "PASS: $*"; }
fail() { echo "FAIL: $*" >&2; FAILS=$((FAILS + 1)); }
check() { # check "description" command...
  local d="$1"; shift
  if "$@" >/dev/null 2>&1; then pass "$d"; else fail "$d"; fi
}

cleanup() {
  if [[ -n "${STARTED_CONTAINER:-}" ]]; then docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; fi
  for n in smoke_clone smoke_anon smoke_src; do
    ENVCTL_DIR="$WORK/envs" "$ENVCTL" stop "$n" >/dev/null 2>&1 || true
  done
  rm -rf "$WORK"
}
trap cleanup EXIT

if [[ -n "${SMOKE_DATABASE_URL:-}" ]]; then
  export DATABASE_URL="$SMOKE_DATABASE_URL"
else
  PORT="${SMOKE_PG_PORT:-55441}"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  docker run -d --name "$CONTAINER" -e POSTGRES_PASSWORD=smoke -p "127.0.0.1:$PORT:5432" \
    postgres:16 -c max_connections=300 >/dev/null
  STARTED_CONTAINER=1
  export DATABASE_URL="postgresql://postgres:smoke@127.0.0.1:$PORT/postgres"
  for _ in $(seq 1 60); do
    PGPASSWORD=smoke psql -h 127.0.0.1 -p "$PORT" -U postgres -d postgres -c 'select 1' >/dev/null 2>&1 && break
    sleep 1
  done
fi

export ENVCTL_DIR="$WORK/envs"
export ENVCTL_CONFIG=/dev/null
export ENVCTL_PORT_MIN="${SMOKE_API_PORT_MIN:-18100}"
export ENVCTL_PORT_MAX="${SMOKE_API_PORT_MAX:-18150}"
# Credentials of a "real" mailbox and SMTP server in the shell: they must never be used.
export IMAP_HOST=imap.invalid IMAP_USER=real IMAP_PASSWORD=real SMTP_HOST=smtp.invalid MAIL_TRANSPORT=smtp
export AUTH_MODE=dev

# Parse the connection once for psql helpers (no password on the command line).
eval "$(DATABASE_URL="$DATABASE_URL" bash -c '
  rest="${DATABASE_URL#*://}"; ui="${rest%@*}"; hp="${rest##*@}"; hp="${hp%%/*}"
  echo "export PGUSER=${ui%%:*} PGPASSWORD=${ui#*:} PGHOST=${hp%%:*} PGPORT=${hp#*:}"')"
q() { psql -X -q -t -A -v ON_ERROR_STOP=1 -d "$1" -c "$2"; }

echo "== source: an empty environment with the development seed =="
"$ENVCTL" create-empty smoke_src --kind test --seed >/dev/null
q smoke_src "UPDATE core.users SET oidc_subject = 'live-subject-1' WHERE email = 'admin@example.com'"
q smoke_src "
  WITH c AS (INSERT INTO crm.customers (id, code, name, email, phone)
             VALUES (gen_random_uuid(), 'SMOKE1', 'Smoke Customer', 'real.person@customer.com', '+32 475 11 22 33')
             RETURNING id),
       k AS (INSERT INTO crm.contacts (id, customer_id, last_name, portal_access)
             SELECT gen_random_uuid(), id, 'Person', true FROM c RETURNING id, customer_id)
  INSERT INTO crm.contact_emails (id, contact_id, email, is_primary)
  SELECT gen_random_uuid(), id, 'real.person@customer.com', true FROM k"
q smoke_src "
  INSERT INTO portal.login_links (id, contact_id, company_id, token_hash, expires_at)
  SELECT gen_random_uuid(), k.id, (SELECT id FROM core.companies LIMIT 1), '\\xdeadbeef', now() + interval '1 day'
    FROM crm.contacts k LIMIT 1"
q smoke_src "
  INSERT INTO core.mail_outbox (id, company_id, to_address, subject, body)
  SELECT gen_random_uuid(), id, 'real.person@customer.com', 'queued in live', 'hello' FROM core.companies LIMIT 1"
SRC_CUSTOMERS="$(q smoke_src "SELECT count(*) FROM crm.customers")"

echo "== clone (the source API keeps running and stays connected) =="
"$ENVCTL" clone smoke_src smoke_clone --kind test --worker
URL="$(grep '^url=' "$ENVCTL_DIR/registry/smoke_clone.env" | cut -d= -f2-)"
DOC="$(curl -fsS "$URL/api/v1/meta/environment")"
[[ "$(jq -r .kind <<<"$DOC")" == test ]] && pass "meta/environment kind is test" || fail "kind: $DOC"
[[ "$(jq -r .name <<<"$DOC")" == smoke_clone ]] && pass "meta/environment name is the clone" || fail "name: $DOC"
[[ "$(jq -r .db_name <<<"$DOC")" == smoke_clone ]] && pass "clone runs on its own database" || fail "db: $DOC"

echo "== credentials scrubbed =="
[[ "$(q smoke_clone "SELECT count(*) FROM portal.login_links")" == 0 ]] && pass "portal login links removed" || fail "login links"
[[ "$(q smoke_clone "SELECT count(*) FROM core.users WHERE oidc_subject IS NOT NULL")" == 0 ]] && pass "oidc subjects cleared" || fail "oidc"
[[ "$(q smoke_clone "SELECT count(*) FROM core.mail_outbox WHERE delivery = 'queued'")" == 0 ]] && pass "queued system mail cancelled" || fail "outbox"
[[ "$(q smoke_src "SELECT count(*) FROM portal.login_links")" == 1 ]] && pass "source keeps its data" || fail "source login links changed"
[[ "$(q smoke_clone "SELECT count(*) FROM crm.customers WHERE email = 'real.person@customer.com'")" == 1 ]] && pass "customer data copied (no anonymize)" || fail "customer copy"

echo "== outbound disabled =="
API_LOG="$ENVCTL_DIR/envs/smoke_clone/api.log"
WORKER_LOG="$ENVCTL_DIR/envs/smoke_clone/worker.log"
sleep 3
check "API log shows the outbound safety banner" grep -q "OUTBOUND SAFETY" "$API_LOG"
check "worker log shows the outbound safety banner" grep -q "OUTBOUND SAFETY" "$WORKER_LOG"
check "worker ignored IMAP_HOST" grep -q "IMAP_HOST ignored" "$WORKER_LOG"
check "worker did not poll IMAP" bash -c "! grep -qi 'IMAP poll failed' '$WORKER_LOG'"
check "worker writes mail to files" grep -q "MAIL_TRANSPORT=files" "$WORKER_LOG"

echo "== isolation =="
q smoke_clone "INSERT INTO crm.customers (id, code, name) VALUES (gen_random_uuid(), 'ONLYCLONE', 'Only in clone')"
[[ "$(q smoke_src "SELECT count(*) FROM crm.customers WHERE code = 'ONLYCLONE'")" == 0 ]] && pass "write in the clone is absent from the source" || fail "leak into source"
[[ "$(q smoke_src "SELECT count(*) FROM crm.customers")" == "$SRC_CUSTOMERS" ]] && pass "source row count unchanged" || fail "source changed"

echo "== safety refusals =="
check "refuses an existing target" bash -c "! '$ENVCTL' clone smoke_src smoke_clone --no-start"
check "refuses an unsafe name" bash -c "! '$ENVCTL' clone smoke_src 'x; drop database postgres' --no-start"
check "refuses cloning onto the source" bash -c "! '$ENVCTL' clone smoke_src smoke_src --no-start"
check "refuses kind live" bash -c "! '$ENVCTL' create-empty smoke_live --kind live --no-start"
"$ENVCTL" register smoke_livereg --database smoke_src --kind live >/dev/null
check "refuses to drop a live-registered environment" bash -c "! '$ENVCTL' drop smoke_livereg --yes"
rm -f "$ENVCTL_DIR/registry/smoke_livereg.env"
check "refuses to drop without confirmation" bash -c "! '$ENVCTL' drop smoke_clone </dev/null"

echo "== anonymize =="
"$ENVCTL" clone smoke_src smoke_anon --kind tutorial --anonymize --no-start
[[ "$(q smoke_anon "SELECT count(*) FROM crm.customers WHERE email LIKE '%@customer.com'")" == 0 ]] && pass "customer e-mail anonymized" || fail "anon customers"
[[ "$(q smoke_anon "SELECT count(*) FROM crm.contact_emails WHERE email NOT LIKE '%@example.test'")" == 0 ]] && pass "contact e-mail anonymized" || fail "anon contacts"
[[ "$(q smoke_anon "SELECT count(*) FROM core.users WHERE email = 'admin@example.com'")" == 1 ]] && pass "admin user kept for sign-in" || fail "admin kept"
[[ "$(q smoke_src "SELECT count(*) FROM crm.customers WHERE email = 'real.person@customer.com'")" == 1 ]] && pass "source not anonymized" || fail "source anon"

echo "== versions, status, drop =="
"$ENVCTL" versions
"$ENVCTL" list
"$ENVCTL" drop smoke_clone --yes
"$ENVCTL" drop smoke_anon --yes
"$ENVCTL" drop smoke_src --yes
[[ "$(q postgres "SELECT count(*) FROM pg_database WHERE datname LIKE 'smoke\_%'")" == 0 ]] && pass "databases dropped" || fail "databases remain"
check "clone instance stopped" bash -c "! curl -fsS -m 2 '$URL/api/v1/meta/environment'"

if ((FAILS > 0)); then echo "$FAILS check(s) failed" >&2; exit 1; fi
echo "envctl smoke test passed"
