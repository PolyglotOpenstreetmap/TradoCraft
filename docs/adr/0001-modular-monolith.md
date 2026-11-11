# 0001 — Modular monolith in Rust, one PostgreSQL database

**Status:** Accepted

**Context.** 20–50 users, one team, strongly connected processes (a repair consumes stock, which
affects purchasing, which affects invoicing). Microservices would add network failure modes and
distributed transactions without solving a real problem.

**Decision.**
* One API binary (`apps/api`), business modules as crates (`crates/*`).
* One database, one schema per module. A module's tables are private: other modules call its
  public Rust functions (e.g. `crm::get_customer_for_quoting`).
* A business operation is one database transaction.
* Side effects outside the database (e-mail, Peppol, accounting sync, heavy PDFs) go through a
  transactional outbox processed by a worker binary, added when the first such effect is needed.
  Internal module-to-module calls stay synchronous; no internal event bus.

**Consequences.** Simple deployment and consistency. Module boundaries are enforced by review and
crate dependencies, not by the network; keep the dependency graph acyclic (sales → crm, products).
