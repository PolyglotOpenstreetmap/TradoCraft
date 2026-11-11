-- Helpdesk: a business calendar per team (ADR 0021). A row with team_id NULL is the company
-- calendar; a team without a calendar of its own inherits it.

ALTER TABLE helpdesk.business_calendars
    ADD COLUMN team_id uuid REFERENCES helpdesk.teams(id) ON DELETE CASCADE;
ALTER TABLE helpdesk.business_calendars DROP CONSTRAINT business_calendars_pkey;
ALTER TABLE helpdesk.business_calendars
    ADD CONSTRAINT business_calendars_scope_key UNIQUE NULLS NOT DISTINCT (company_id, team_id);

ALTER TABLE helpdesk.business_hours
    ADD COLUMN team_id uuid REFERENCES helpdesk.teams(id) ON DELETE CASCADE;
ALTER TABLE helpdesk.business_hours DROP CONSTRAINT business_hours_pkey;
ALTER TABLE helpdesk.business_hours
    ADD CONSTRAINT business_hours_scope_key UNIQUE NULLS NOT DISTINCT (company_id, team_id, weekday);

ALTER TABLE helpdesk.holidays
    ADD COLUMN team_id uuid REFERENCES helpdesk.teams(id) ON DELETE CASCADE;
ALTER TABLE helpdesk.holidays DROP CONSTRAINT holidays_company_id_day_key;
ALTER TABLE helpdesk.holidays
    ADD CONSTRAINT holidays_scope_key UNIQUE NULLS NOT DISTINCT (company_id, team_id, day);
