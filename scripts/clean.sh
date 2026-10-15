#!/usr/bin/env bash
# Quick clean up script
set -euo pipefail

cd "$(dirname "$0")/.."

step() { printf '\n\033[1;34m==> %s\033[0m\n' "$1"; }

step "Formatting Rust (rustfmt)"
./scripts/fmt.sh

step "Fixing Rust lints (clippy --fix)"
# --allow-dirty/--allow-staged so it works with uncommitted changes.
# Non-fatal: surface what clippy can't auto-fix without aborting the styles pass.
cargo clippy --fix --workspace --all-targets --allow-dirty --allow-staged || true

if command -v npm >/dev/null 2>&1; then
	step "Formatting styles (prettier)"
	npm run format

	step "Fixing styles (stylelint --fix)"
	npm run lint:style:fix
else
	step "Skipping styles (npm not found)"
fi

printf '\n\033[1;32mDone.\033[0m Review the changes with `git diff` before committing.\n'
