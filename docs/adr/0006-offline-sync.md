# 0006 — Narrow, append-only offline sync for technicians

**Status:** Proposed (field service release)

**Decision.**
* Down-sync only what a technician needs: their assigned jobs, the related customers, sites,
  installed assets, documents and van stock, into a local SQLite database (Drift).
* Up-sync only append-only facts: time entries, parts consumed, checklist answers, photos,
  signatures, status changes. Each record gets a client-generated UUIDv7; server endpoints are
  idempotent, so retries are safe.
* No general bidirectional merge of editable records. Conflicts are resolved by the server
  (e.g. a job reassigned meanwhile) and reported back.
* Evaluate PowerSync (PostgreSQL ↔ SQLite, Flutter SDK) before building a custom sync.
