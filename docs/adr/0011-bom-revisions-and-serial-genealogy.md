# 0011 — BOM revisions and serial-number genealogy

**Status:** Accepted

**Context.** Products are bought, made from components, or configured. Repairs and warranties need
to know exactly what is inside an individual unit, even years later and after parts were replaced.

**Decision.**
* **BOMs are revisions of a product** (`engineering.boms`): `draft → active → obsolete`. Only a
  draft is editable; at most one draft and one active revision per product. Activating a draft
  obsoletes the previous revision. Revisions are never deleted.
* **Multi-level by composition.** A BOM line points to a product. If that component has an active
  BOM itself it is a subassembly; otherwise it is a leaf (bought, or made without a BOM). Lines
  carry quantity and scrap %.
* **No cycles.** Activation checks the whole active structure (serialised with an advisory lock)
  and reports the loop, e.g. `FRAME-A → UNIT-100 → FRAME-A`.
* **Explosion** is a pure function (`engineering::explode`) over the active structure: required
  quantity = parent quantity × line quantity × (1 + scrap %), exact decimals, rounded only for
  output; leaf totals are what must be bought or made.
* **Serial tracking is a product property** (`tracking = none | serial`). Each unit
  (`engineering.serial_units`) records the BOM revision it was built to, its status
  (`in_stock`, `built_in`, `delivered`, `installed`, `scrapped`) and customer.
* **Genealogy.** `serial_unit_components` links serialised components to the unit they are built
  into. A component must be in the unit's BOM revision, within its quantity, and cannot create a
  loop. Links are never deleted: removing a component sets `removed_at`, so the as-built and
  as-maintained history stays complete.

**Next.** Production orders create serial units and links automatically ([ADR 0036](0036-production.md)); goods receipt will
register purchased serialised parts; the service module will treat installed units as assets.
Phantom assemblies, alternative components and configurable options are later extensions of the
BOM line.
