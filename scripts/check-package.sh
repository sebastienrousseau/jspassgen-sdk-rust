#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Fails when the crate package would ship a file that belongs only in the
# repository: docs, scripts or CI configuration. The allow-list is
# `include` in Cargo.toml; this catches a change that widens it by
# mistake.
#
#   scripts/check-package.sh
set -euo pipefail

cd "$(dirname "$0")/.."

list="$(cargo package --list --locked --allow-dirty)"
if stray="$(printf '%s\n' "$list" \
  | grep -E '^(\.github/|\.cargo/|docs/|scripts/|LICENSES/)')"; then
  echo "check-package: these files would ship in the crate:" >&2
  printf '  %s\n' "$stray" >&2
  exit 1
fi
echo "check-package: $(printf '%s\n' "$list" | wc -l | tr -d ' ') files, none from the repository only"
