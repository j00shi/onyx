# AGENTS.md

Guidance for AI agents working in this repository.

## What this is

`onyx` is a cross-platform [Dioxus](https://dioxuslabs.com/) (0.7) application written in
Rust. It is a Cargo workspace where shared UI and server logic live in library crates and
each platform is a thin binary crate.

## Workspace layout

- `frontend/` — all shared UI (crate `frontend`). Routes in `src/routes.rs`, pages in
  `src/views/`, project-owned components in `src/custom/`, styles in `src/*.scss`.
- `backend/` — shared fullstack server functions (crate `backend`).
- `packages/web`, `packages/desktop`, `packages/mobile` — per-platform binary crates. Each
  gates its platform behind a Cargo feature (`web`, `desktop`, `mobile`) plus a `server`
  feature. `packages/web` is the workspace `default-members` target.
- `scripts/` — `fmt.sh` and `clean.sh` helpers (see below).

## Vendored components — do not edit

`frontend/src/components/` holds default Dioxus components that are **overwritten by upstream
updates**. Never edit these by hand. All tooling deliberately skips this directory
(`scripts/fmt.sh` filters it, the module is `#[allow(...)]`'d in `frontend/src/lib.rs`, and
the Prettier/Stylelint globs ignore it). Put project-owned components in `src/custom/`.

## Code quality — strict, enforced in CI

CI (`.forgejo/workflows/ci.yml`) runs on every push and PR with `RUSTFLAGS="-D warnings"`, so
any warning fails the build. Match it locally before considering work done.

### Rust

- Lints are configured in `[workspace.lints]` in the root `Cargo.toml`. `clippy::pedantic`
  and `clippy::nursery` run as warnings; a curated set is denied outright: `unwrap_used`,
  `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout`,
  `print_stderr`, and more. **Do not use `.unwrap()`, `.expect()`, `panic!`, or `println!`
  — propagate errors instead.**
- Check lints: `cargo clippy --workspace --all-targets -- -D warnings`
- **Format with the helper, never bare `cargo fmt`** (bare fmt would touch the vendored
  components):

  ```bash
  ./scripts/fmt.sh          # format our files in place
  ./scripts/fmt.sh --check  # verify (used by CI)
  ```

### SCSS

Linted with Stylelint, formatted with Prettier. Run `npm install` once, then:

```bash
npm run lint:style        # lint
npm run lint:style:fix    # auto-fix lint issues
npm run format            # format
npm run format:check      # verify (used by CI)
```

### One-shot cleanup

`./scripts/clean.sh` runs format + lint-fix for both Rust and styles
(`--rust` or `--style` to scope). Review `git diff` afterward.

## Running

Install the Dioxus CLI (`dx`) if needed, then serve a platform crate, e.g.:

```bash
dx serve --package web
```

## Conventions

- `foo/foo.rs` module layout is intentional (`clippy::module_inception` is allowed).
- Follow the surrounding Dioxus `rsx!` style when adding UI.
- Before finishing: run `./scripts/fmt.sh --check`, clippy with `-D warnings`, and the style
  checks — these are exactly what CI gates on.
