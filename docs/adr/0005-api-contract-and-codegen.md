# 0005 — OpenAPI generated from Rust, Dart client generated from OpenAPI

**Status:** Accepted

**Context.** The biggest productivity risk of a custom ERP is hand-writing every API call, model
and screen. Open-source ERPs are productive because most screens come from metadata.

**Decision.**
* Handlers are annotated with `utoipa`; `tradocraft-api openapi` writes `api/openapi.json`.
* The Dart client (`app/lib/api/generated`) is generated with `swagger_parser` + `build_runner`
  (`scripts/generate-api-client.sh`). Generated files are committed; CI fails if they are stale.
* Lists use one generic widget (`EntityList`) configured per entity; forms will follow the same
  approach as patterns stabilise.

**Consequences.** API changes surface as Dart compile errors instead of runtime failures. JSON
shapes must stay generator-friendly: flat objects, no `serde(flatten)`, decimals as strings.
