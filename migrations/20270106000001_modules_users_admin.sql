-- Module visibility and user administration (ADR 0027).
-- Only what is DISABLED is stored, so existing installs keep every module on without a backfill.

ALTER TABLE core.companies
    ADD COLUMN disabled_modules text[] NOT NULL DEFAULT '{}';

ALTER TABLE core.user_companies
    ADD COLUMN disabled_modules text[] NOT NULL DEFAULT '{}',
    ADD COLUMN active boolean NOT NULL DEFAULT true;
