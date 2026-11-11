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
sed "s|@EXEC@|$bundle/tradocraft_app|" "$here/$id.desktop.in" > "$apps/$id.desktop"
command -v update-desktop-database >/dev/null && update-desktop-database "$apps" || true
command -v gtk-update-icon-cache >/dev/null && gtk-update-icon-cache -q "${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor" || true
echo "installed $apps/$id.desktop"
