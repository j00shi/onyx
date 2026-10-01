#!/usr/bin/env bash
#
# Formatting helpers that respect the vendored `ui/src/standard` directory.
#
# `ui/src/standard` contains Dioxus components that are overwritten by upstream
# updates, so they must never be reformatted. `cargo fmt` has no stable way to
# ignore a path, so we format the files we own explicitly via `git ls-files`.
#
# Usage:
#   scripts/fmt.sh          # format in place
#   scripts/fmt.sh --check  # check only, non-zero exit on diff (used in CI)
set -euo pipefail

cd "$(dirname "$0")/.."

mode="write"
if [[ "${1:-}" == "--check" ]]; then
	mode="check"
fi

# All tracked Rust files except the vendored standard components.
mapfile -t files < <(git ls-files '*.rs' ':!:ui/src/standard/**')

if [[ ${#files[@]} -eq 0 ]]; then
	echo "No Rust files to format."
	exit 0
fi

if [[ "$mode" == "check" ]]; then
	rustfmt --edition 2021 --check "${files[@]}"
else
	rustfmt --edition 2021 "${files[@]}"
fi
