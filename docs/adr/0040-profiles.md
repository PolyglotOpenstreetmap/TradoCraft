# 0040 — Profiles: isolated sessions in separate windows

**Status:** Accepted

**Context.** The Flutter desktop app kept one session (server, dev user, token, company), one
language and the offline queue in `SharedPreferences`. Testing roles, or working as an admin and
as a salesperson at the same time, meant signing out and in. Two processes cannot share
`SharedPreferences` on Linux and Windows: it is one file per app and the last writer wins. The API
needs no change: with `AUTH_MODE=dev` the user comes from the `X-Dev-User` header of each request,
and roles belong to the user ([ADR 0004](0004-authentication.md)).

**Decision.**

* **A profile is a name.** `--profile=<name>` (or `--profile <name>`) on the command line, else the
  environment variable `TRAYD_PROFILE`. The name is reduced to letters, digits, `_`, `.` and `-`
  (at most 32 characters), because it becomes a file name.
* **One JSON file per profile** in the application support directory
  (`profiles/<name>.json`, via `path_provider`), written through a temporary file and a rename.
  It holds what the app stored before: the session (`SessionStore`), the language and the offline
  queue and job cache (`kvStoreProvider`), plus the open tabs
  ([ADR 0041](0041-workspace-tabs.md)). All of it goes through the existing `KvStore` seam, which
  `main` overrides, so no feature code knows about profiles.
* **Without a profile nothing changes**: the same `SharedPreferences` keys as before, so existing
  installs keep their session.
* **Identity.** The shell shows a badge with the profile name and the user's roles (colour derived
  from the name), the account menu names the profile, and the native window title is
  `tradocraft - <profile>` (`linux/runner/my_application.cc`; the Windows runner is not changed yet), so
  windows can be told apart in the task bar. A profile's login page pre-fills the e-mail that was
  last used in it.
* `scripts/run-profile.sh <name> [--debug]` starts a built binary, or `flutter run -d linux`.

**Consequences.**

* Two profiles never overwrite each other. Two windows of the *same* profile would (whole-file,
  last writer wins, as before); nothing prevents that, and it is not a supported use.
* The file holds the access token in plain text, as `SharedPreferences` did. Moving tokens to the
  platform keychain is a separate change.
* The customer portal (a web build) is untouched and still uses `SharedPreferences`.
* On web there are no command-line arguments or files: profiles are a desktop feature.
