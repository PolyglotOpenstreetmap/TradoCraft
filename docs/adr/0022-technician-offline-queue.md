# 0022 — Technician offline queue

**Status:** Accepted

**Context.** [ADR 0006](0006-offline-sync.md) chose a narrow, append-only up-sync for technicians and
made the server calls idempotent; [ADR 0015](0015-repairs-field-service-and-photos.md) and
[ADR 0020](0020-field-service-reports-contracts.md) built the field jobs flow on it. What was
missing was the client: a technician in a cellar without signal could not tick a task, log time
or take a photo. This ADR is part 2 of roadmap item A3.

**Decision.**

* **Scope.** Down-sync is a plain read-through cache (the technician's job list and the details of
  their open jobs, kept in `shared_preferences`). Up-sync is a persistent FIFO queue of the writes
  a technician makes on site. No local SQL database, no PowerSync: the data is small and the
  writes are append-only facts, so a list of operations is enough. Revisit ADR 0006's Drift/PowerSync
  idea only if technicians need to *search* offline data or edit shared records.
* **Idempotency audit.** Every queued write can be sent twice with the same result:

  | Write | Endpoint | Idempotent by | Change in this ADR |
  |---|---|---|---|
  | Start job | `POST /field/jobs/{id}/start` | state: `in_progress` stays `in_progress` | none |
  | Tick / untick task | `POST .../tasks/{task}/done` | sets a state, `coalesce` keeps first `done_by/at` | none |
  | Add task | `POST .../tasks` | **was not** (a second task per retry) | optional `client_id`; migration `20261112000001` adds `field.job_tasks.client_id` with a unique index `(job_id, client_id)`; replay returns the first task, same id with another description is 409 |
  | Work / travel log | `POST .../work-logs` | client `id` (primary key) | same id with other content (kind, times, notes) is now 409 instead of silently accepted |
  | Part used | `POST .../parts` | client `id` | same id with other content is now 409 |
  | Photo | `POST .../photos?client_id=` | client id is the file id | same id with a different picture (size) is now 409 |
  | Signature image | `POST .../signature?client_id=` | client id is the file id | same id with a different image is 409; a replay no longer writes a second `signed` audit event |
  | Declined to sign | `POST .../signature/declined` | one row per job | a replay with the same name and reason changes nothing (no new `signed_at`, no second audit event) |
  | Complete job | `POST .../complete` | **was not** (409 "the job is completed" on retry) | completing a completed job with the *same* report text returns 200 unchanged; other text is 409 |
  | Notes | there is no separate note write: notes ride on work logs and the completion report | | |

  The unique index is per job rather than per company and job: a job belongs to one company, so
  the extra column would add nothing. The id fields keep their existing names (`id`, `client_id`)
  and stay optional, so old clients work as before. Planner-only actions (create, edit, cancel,
  reopen), equipment, removing a part, and sending the report are online-only.
* **Queue** (`app/lib/features/field/offline/offline_queue.dart`). Each operation has a client
  UUID (reused as the record's id), type, job, payload, `created_at`, `attempts`, `last_error`,
  state. They are stored in `shared_preferences` as an index plus one entry per operation, so
  rewriting an operation's attempt count does not rewrite a photo. The keys are prefixed with the
  server, and the dev user or company, so two people on one device do not send each other's work.
* **Writing path.** A job screen action first sends the request normally. If the request fails
  *without a response* (no signal, timeout), the operation is queued and shown at once
  (`applyPending` lays the queue over the last server copy of the job) with a "waiting to sync"
  marker. Any other error (4xx/5xx) is shown as before and not queued. While a job has queued
  operations, new writes for that job are queued without trying the network, so order is kept.
* **Replay.** First in, first out. Triggers: a 30 s timer (`OfflineSyncHost` around the signed-in
  shell), the app returning to the foreground, a queue entry made while others wait, and the
  "Sync now" button. There is no connectivity plugin: a request that gets no answer is the signal.
  After a network error the run stops (other jobs would fail the same way) and the timer stays quiet
  until a waiting time has passed: 5 s, 10 s, 20 s ... at most 5 minutes; "Sync now" ignores it.
  * 2xx (including an idempotent duplicate) removes the operation and refreshes the open job with
    the server's answer.
  * 408, 429, 5xx: tried again later, only the same job waits.
  * Any other 4xx (job reassigned = 404, cancelled = 409, no rights = 403, validation = 422) marks
    the operation **failed**: it stays in the list with the server's message, the banner turns red,
    later operations of the same job wait behind it (so a completion never overtakes a refused
    time entry), other jobs carry on. The technician chooses *Try again* or *Discard* (with
    confirmation). Nothing is dropped silently.
  * If the app is killed after the server saw a request but before the operation was removed, the
    next run sends it again; this is exactly what the idempotency above is for.
* **Photos and signatures** are stored base64 in `shared_preferences`, because `path_provider` is
  not a dependency. That caps them: 3 MB per file and 12 MB in total waiting (phone photos are
  already scaled to 2400 px / 85 % by the picker, typically 0.3-1.5 MB). Beyond the cap the
  technician is told to connect to send it; nothing is lost silently. On the web the browser
  quota (about 5 MB) is lower still. Moving to files in the documents directory needs
  `path_provider` and is a drop-in change inside the queue.
* **Read cache.** The list of `my-jobs` and the detail of up to 30 jobs (the open ones are
  prefetched after each successful list load, 20 at most) are kept. When a read fails without a
  response the cached copy is shown read-only with "Offline: data from <time>"; the job screen
  still accepts writes (queued). Photos themselves are not cached: thumbnails need a connection.
  Planner lists are not cached.
* **UI.** A banner "N changes waiting to sync" on the jobs page (all jobs) and the job screen
  (that job); tapping it opens the list with *Sync now* and, for failed operations, the server's
  message with *Try again* / *Discard*. Strings are in the five app languages.

**Consequences.**
* Conflicts are resolved by the server and reported, not merged: a technician working on a job
  that was reassigned or cancelled sees the refusal and decides.
* The optimistic view is derived (server copy + queue), so a restart shows the same thing and
  nothing needs to be rolled back when an operation is discarded.
* A task added offline cannot be ticked until it is synced (the server assigns no id until then
  that the device could use; the checkbox is disabled).
* Removing a part or equipment, editing the job and sending the report need a connection.
* Signing out leaves the queue on the device under the user's own prefix; it is sent when that
  user signs in again.
* Queued `created_at` is device time; the server stamps `done_at`, `signed_at` and audit times when
  the operation arrives. Work log start and end times are the technician's own and are kept.

**Not yet.** Files instead of base64 (needs `path_provider`), a local database and offline
search, caching photos and the customer's equipment list, queueing planner actions, background
sync while the app is closed (WorkManager / iOS background fetch), and a conflict view that shows
the server's current value next to the technician's change.
