-- Technician offline queue (ADR 0022): a checklist task added from a phone that was offline can be
-- replayed without creating a second task. The client chooses the id (UUID); the same id on the same
-- job is the same task. Other offline writes already carry a client id (work logs, parts, photos,
-- signature files) or are naturally idempotent (start, tick, complete, decline).
ALTER TABLE field.job_tasks ADD COLUMN client_id uuid;
CREATE UNIQUE INDEX job_tasks_client_id_idx ON field.job_tasks (job_id, client_id) WHERE client_id IS NOT NULL;
