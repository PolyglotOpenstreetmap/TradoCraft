-- Planning board (field-jobs half): resource calendars (working hours, absences), skills and
-- per-company planning settings. The calendar tables are keyed by user today; work centres reuse
-- them later by adding a sibling resource column (ADR 0024).

CREATE SCHEMA IF NOT EXISTS planning;

-- One row per company (defaults apply while there is none).
CREATE TABLE planning.settings (
    company_id            uuid PRIMARY KEY REFERENCES core.companies(id),
    -- minutes kept free between two consecutive jobs of a technician on the same day
    travel_buffer_minutes integer NOT NULL DEFAULT 15 CHECK (travel_buffer_minutes BETWEEN 0 AND 480),
    -- length assumed for a job without an end time
    default_job_minutes   integer NOT NULL DEFAULT 120 CHECK (default_job_minutes BETWEEN 15 AND 1440),
    -- working hours and absences are local; job times are instants
    timezone              text NOT NULL DEFAULT 'Europe/Brussels' CHECK (btrim(timezone) <> ''),
    updated_at            timestamptz NOT NULL DEFAULT now()
);

-- Several rows per weekday are allowed (a morning and an afternoon block). user_id NULL = the
-- company default, used for every user who has no rows of their own.
CREATE TABLE planning.working_hours (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    user_id     uuid REFERENCES core.users(id),
    weekday     smallint NOT NULL CHECK (weekday BETWEEN 1 AND 7),
    start_time  time NOT NULL,
    end_time    time NOT NULL,
    CHECK (end_time > start_time)
);
CREATE INDEX working_hours_idx ON planning.working_hours (company_id, user_id, weekday);

CREATE TABLE planning.absences (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    user_id     uuid NOT NULL REFERENCES core.users(id),
    from_date   date NOT NULL,
    to_date     date NOT NULL,
    kind        text NOT NULL DEFAULT 'vacation' CHECK (kind IN ('vacation', 'sick', 'training', 'other')),
    note        text NOT NULL DEFAULT '',
    created_by  uuid REFERENCES core.users(id),
    created_at  timestamptz NOT NULL DEFAULT now(),
    CHECK (to_date >= from_date)
);
CREATE INDEX absences_idx ON planning.absences (company_id, user_id, from_date);

CREATE TABLE planning.skills (
    id          uuid PRIMARY KEY,
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    code        text NOT NULL CHECK (btrim(code) <> ''),
    name        text NOT NULL CHECK (btrim(name) <> ''),
    UNIQUE (company_id, code)
);

CREATE TABLE planning.user_skills (
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    user_id     uuid NOT NULL REFERENCES core.users(id),
    skill_id    uuid NOT NULL REFERENCES planning.skills(id) ON DELETE CASCADE,
    level       smallint NOT NULL DEFAULT 1 CHECK (level BETWEEN 1 AND 3),
    PRIMARY KEY (user_id, skill_id)
);

-- Skills a field job needs (the technician has to have all of them, or the planner is warned).
CREATE TABLE field.job_required_skills (
    company_id  uuid NOT NULL REFERENCES core.companies(id),
    job_id      uuid NOT NULL REFERENCES field.jobs(id) ON DELETE CASCADE,
    skill_id    uuid NOT NULL REFERENCES planning.skills(id) ON DELETE CASCADE,
    PRIMARY KEY (job_id, skill_id)
);
CREATE INDEX job_required_skills_skill_idx ON field.job_required_skills (skill_id);
