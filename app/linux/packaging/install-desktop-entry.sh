#!/usr/bin/env bash
# Installs a launcher entry and icon for the user so the dock / app menu show the
# tradocraft icon (the window icon alone is not used by GNOME on Wayland).
# Usage: install-desktop-entry.sh <path to the bundle directory containing tradocraft_app>
set -euo pipefail
bundle="$(realpath "${1:?usage: $0 <bundle dir>}")"
here="$(cd "$(dirname "$0")" && pwd)"
id="be.tradocraft.tradocraft_app"
[ -x "$bundle/tradocraft_app" ] || { echo "no tradocraft_app in $bundle" >&2; exit 1; }
icons="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor/256x256/apps"
apps="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
mkdir -p "$icons" "$apps"
cp "$here/../runner/resources/app_icon.png" "$icons/$id.png"

# Serialize the executable path as ONE double-quoted Exec argument (desktop entry spec):
# 1. inside quotes, backslash, double quote, backtick and dollar are backslash-escaped;
# 2. backslashes are doubled again because string values are unescaped before Exec is parsed;
# 3. a literal percent is written %%.
exec_path="$bundle/tradocraft_app"
exec_val=${exec_path//\\/\\\\}
exec_val=${exec_val//\"/\\\"}
exec_val=${exec_val//\`/\\\`}
exec_val=${exec_val//\$/\\\$}
exec_val=${exec_val//\\/\\\\}
exec_val=${exec_val//%/%%}
exec_val="\"$exec_val\""
# Escape backslash, ampersand and the | delimiter for the sed replacement (shell layer only).
sed_repl=$(printf '%s' "$exec_val" | sed -e 's/[\\&|]/\\&/g')
sed "s|@EXEC@|$sed_repl|" "$here/$id.desktop.in" > "$apps/$id.desktop"
command -v update-desktop-database >/dev/null && update-desktop-database "$apps" || true
command -v gtk-update-icon-cache >/dev/null && gtk-update-icon-cache -q "${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor" || true
echo "installed $apps/$id.desktop"
