-- Purchasing depends on products and inventory (ADR 0031): where either is disabled,
-- purchasing is disabled too. Stored values only list what is DISABLED.

UPDATE core.companies
SET disabled_modules = array_append(disabled_modules, 'purchasing')
WHERE ('products' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules))
  AND NOT ('purchasing' = ANY (disabled_modules));

UPDATE core.user_companies
SET disabled_modules = array_append(disabled_modules, 'purchasing')
WHERE ('products' = ANY (disabled_modules) OR 'inventory' = ANY (disabled_modules))
  AND NOT ('purchasing' = ANY (disabled_modules));
