-- ADR 0030 phase 2: findings, the link to a serial unit, the automatic warranty lookup and the
-- workshop's override ("void, misuse"). Additive: existing repairs keep working.

ALTER TABLE repairs.repairs
    ADD COLUMN findings                 text NOT NULL DEFAULT '',
    ADD COLUMN diagnosis                text NOT NULL DEFAULT '',
    ADD COLUMN serial_unit_id           uuid REFERENCES engineering.serial_units(id),
    -- The result of the lookup, written by the system only; `warranty` stays the final status.
    ADD COLUMN warranty_auto            text NOT NULL DEFAULT 'unknown'
                                        CHECK (warranty_auto IN ('unknown', 'in_warranty', 'out_of_warranty')),
    ADD COLUMN warranty_auto_until      date,
    ADD COLUMN warranty_auto_at         timestamptz,
    ADD COLUMN warranty_overridden      boolean NOT NULL DEFAULT false,
    ADD COLUMN warranty_override_by     uuid REFERENCES core.users(id),
    ADD COLUMN warranty_override_at     timestamptz,
    ADD COLUMN warranty_override_reason text NOT NULL DEFAULT '';

-- History is not rewritten as "unknown": what staff typed so far is what the lookup "found".
UPDATE repairs.repairs SET warranty_auto = warranty;

ALTER TABLE repairs.repairs DROP CONSTRAINT repairs_warranty_check;
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_warranty_check
    CHECK (warranty IN ('unknown', 'in_warranty', 'out_of_warranty', 'void'));
-- A void warranty always says who, when and why.
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_override_complete
    CHECK (NOT warranty_overridden OR (warranty = 'void' AND warranty_override_by IS NOT NULL
           AND warranty_override_at IS NOT NULL AND btrim(warranty_override_reason) <> ''));
ALTER TABLE repairs.repairs ADD CONSTRAINT repairs_void_is_override
    CHECK (warranty <> 'void' OR warranty_overridden);
CREATE INDEX repairs_serial_unit_idx ON repairs.repairs (serial_unit_id);

-- All earlier kinds are kept (created .. photo, and the parcel links of phase 1).
ALTER TABLE repairs.events DROP CONSTRAINT events_kind_check;
ALTER TABLE repairs.events ADD CONSTRAINT events_kind_check
    CHECK (kind IN ('created', 'status', 'routed', 'note', 'photo', 'package_linked', 'package_unlinked',
                    'findings', 'serial_unit', 'warranty_auto', 'warranty_manual', 'warranty_void',
                    'warranty_void_removed'));
