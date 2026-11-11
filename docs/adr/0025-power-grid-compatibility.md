# 0025 — Power grids and product supply compatibility

**Status:** Accepted (the grid data must be reviewed before anyone relies on it)

**Context.** Products are electrical (24 V DC, 230 V, 3x400 V ...) and countries differ in what the
wall socket or the switchboard delivers. A quotation for a 110 V machine going to a 230 V country
should not go out unnoticed. Nothing in the data model described either side.

**Decision.** Migration `20270104000001_power_grids.sql`.

The header comment of `20270104000001_power_grids.sql` cites ADR 0024, the number this ADR had when the file was first committed. It is frozen on purpose (sqlx checksums); see the [runbook](../runbooks/sqlx-migration-checksums.md).

* **Grids** (`products.power_grids`, reference data without `company_id`): `country_code`, `name`,
  `voltage_v`, `phases` (1 or 3), `frequency_hz` (50 or 60), `active`; unique on
  (country, voltage, phases, frequency). A country can have several grids. Managed under
  `/power-grids` with `products.read` / `products.write`, audited. Voltage is the nominal figure as
  quoted: phase-to-neutral for 1 phase, line-to-line for 3 phases.
* **Supplies** (`products.product_power_supplies`): `current_type` (`ac`|`dc`), `voltage_v`,
  `phases` (required for AC, NULL for DC), `frequency_hz` (NULL = works on 50 and 60 Hz, always NULL
  for DC), `tolerance_pct` (default 10, 0 to 50). `PUT /products/{id}/power-supplies` replaces the
  whole list in one audited transaction. No rows = not an electrical product, never checked.
* **Check** (`products::power::check`, pure and unit-tested): `NotApplicable` (no supplies),
  `NeedsExternalSupply` (only DC), `UnknownGrid` (no grid data), `Compatible`, or
  `Incompatible { supported, available }`. An AC supply fits a grid when the phases are equal, the
  frequency is equal (or the supply has none) and `|grid - supply| <= supply * tolerance / 100`; so
  220 V fits a 230 V grid and 3x380 V fits 3x400 V. Against a chosen grid only that grid counts,
  otherwise one match with any active grid of the country is enough. With AC and DC supplies on one
  product the AC ones decide. `GET /products/{id}/power-compatibility?country=&grid_id=` exposes it
  (a flat `status` plus the profiles); sales uses `power::check_products` and `grid_in_country`
  instead of reading product tables.
* **Sales.** `sales.quotations` and `sales.orders` get a nullable `power_grid_id` (needed when a
  country has several grids). It must belong to the delivery country (422 otherwise); on a quotation
  update it is kept only while the delivery country is unchanged; on an order edit it is cleared if
  the new country does not have it. `PUT /orders/{id}` takes an optional `power_grid_id` too (draft
  only, validated against the new or stored delivery country, 422 otherwise). To go back to "any
  grid of the country" a quotation update or order edit sends `clear_power_grid: true`: it wins
  over keeping the stored grid, stores NULL, and is a 422 together with a `power_grid_id`. It is an
  additive flag rather than a nullable `power_grid_id`, so generated clients stay simple. The grid
  is copied on conversion and on both kinds of revision.
  Quotation create/update, `POST /pricing/preview` (which gained optional `purpose`,
  `delivery_country_code`, `power_grid_id`) and `POST /orders/{id}/confirm` return a `warnings` array
  (`kind: "power_grid"`, `product_code`, `message`, `details.status` of `incompatible`,
  `needs_external_supply` or `unknown_grid`). The country is the delivery country, falling back to
  the customer's (an invalid customer country gives no warnings and is not stored). A stored grid
  that no longer belongs to that country is ignored and the country's active grids are used.
  Purpose `stock` produces none. Warnings never block anything. `GET` of a
  quotation or order returns an empty `warnings`.

**Data caveats.** `seed/power_grids.csv` (496 rows, 228 countries) is the documented source of the
seeded rows and was converted from a table that was not built for machines to read:

* Cells with several values were expanded to every voltage x frequency combination, e.g. Japan
  100/200/210 V is stored at both 50 and 60 Hz, although in reality the frequency depends on the
  region. Such rows over-state what a country offers and make mismatches less likely to be flagged.
* The source token for Colombia, "240/208/120", was dropped.
* Russia's glued value was split into 230/220 V single phase and 400/380 V three phase.
* Guyana and the US Virgin Islands list 190 V under single phase in the source (probably a 3-phase
  value); kept as given.
* JP, GY, SC, MR, SX and VI have no 3-phase entries, so every 3-phase product there shows as
  incompatible although a 3-phase supply may exist.

**The data should be reviewed by the user before it is relied on.** The grids are editable
reference data, so corrections need no migration.

**Consequences.** Warnings can be wrong where the data is, and are advice only. Frequency is not
checked for DC and a grid without data gives `unknown_grid`, which is shown for every electrical
product. Future work: app screens for grids and supplies, rules for combining products (step 4).
