#!/usr/bin/env bash
# Local mirror of .forgejo/workflows/ci.yml so you can catch failures before pushing.
# Runs every CI check, continuing past failures so you see all errors in one pass.
# scripts/ci.sh          # run all checks
# scripts/ci.sh --rust   # Rust checks only (fmt + clippy)
# scripts/ci.sh --style  # style checks only (prettier + stylelint + css)
set -uo pipefail

cd "$(dirname "$0")/.."

do_rust=true
do_style=true
case "${1:-}" in
	--rust) do_style=false ;;
	--style) do_rust=false ;;
	"") ;;
	*) echo "Unknown option: $1" >&2; echo "Usage: scripts/ci.sh [--rust|--style]" >&2; exit 1 ;;
esac

# Matches the CI env: any warning is an error.
export RUSTFLAGS="-D warnings"
export CARGO_TERM_COLOR=always

step() { printf '\n\033[1;34m==> %s\033[0m\n' "$1"; }

failures=()
# Run a named check; record the name if it exits non-zero, but keep going.
run() {
	local name="$1"; shift
	step "$name"
	if ! "$@"; then
		failures+=("$name")
	fi
}

if [[ "$do_rust" == true ]]; then
	run "Rust formatting (fmt.sh --check)" ./scripts/fmt.sh --check
	run "Clippy (deny warnings)" cargo clippy --workspace --all-targets -- -D warnings
fi

if [[ "$do_style" == true ]]; then
	if command -v npm >/dev/null 2>&1; then
		run "SCSS formatting (prettier)" npm run format:check
		run "SCSS lint (stylelint)" npm run lint:style
		run "Compiled CSS up to date" npm run build:css:check
	else
		step "Skipping styles (npm not found)"
		failures+=("Styles skipped — npm not installed")
	fi
fi

if [[ ${#failures[@]} -eq 0 ]]; then
	printf '\n\033[1;32mAll checks passed.\033[0m\n'
	exit 0
fi

printf '\n\033[1;31mFailed checks:\033[0m\n'
for f in "${failures[@]}"; do
	printf '  \033[1;31m✗\033[0m %s\n' "$f"
done
printf '\nRun \033[1m./scripts/clean.sh\033[0m to auto-fix formatting/lint, and \033[1mnpm run build:css\033[0m to recompile CSS.\n'
exit 1
