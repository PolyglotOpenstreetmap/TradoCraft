#!/usr/bin/env bash
# Starts the desktop app as a named profile: an isolated session (own sign-in, language, offline
# queue and open tabs), so several windows can run side by side, e.g. admin and sales.
#
#   scripts/run-profile.sh <name> [--debug]
#
# Uses a built binary (app/build/linux/x64/release/bundle/tradocraft_app, or the debug build with
# --debug) when there is one, and `flutter run -d linux` otherwise. Set APP_BIN to use another
# binary. The profile is stored in <app support dir>/profiles/<name>.json.
set -euo pipefail

if [[ $# -lt 1 || "$1" == -* ]]; then
  echo "usage: $0 <profile-name> [--debug]" >&2
  exit 2
fi
profile="$1"
shift
mode=release
for a in "$@"; do
  case "$a" in
    --debug) mode=debug ;;
    *) echo "unknown option: $a" >&2; exit 2 ;;
  esac
done

cd "$(dirname "$0")/../app"
bin="${APP_BIN:-build/linux/x64/$mode/bundle/tradocraft_app}"
if [[ -x "$bin" ]]; then
  exec "$bin" "--profile=$profile"
fi
flag=()
[[ "$mode" == release ]] && flag=(--release)
exec nice flutter run -d linux "${flag[@]}" "--dart-entrypoint-args=--profile=$profile"
