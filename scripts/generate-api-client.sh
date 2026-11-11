#!/usr/bin/env bash
# Regenerates api/openapi.json from the Rust code and the Dart client from it.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo run -q -p tradocraft-api -- openapi > api/openapi.json
cd app
dart run swagger_parser
dart run build_runner build -d
