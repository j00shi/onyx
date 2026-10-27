# AGENTS.md

Guidance for AI agents working in this repository.

## What this is

`onyx` is a cross-platform [Dioxus](https://dioxuslabs.com/) (0.7) application written in
Rust. It is a Cargo workspace where shared UI and server logic live in library crates and
each platform is a thin binary crate.

## Workspace layout

- `frontend/` — all shared UI (crate `frontend`). Routes in `src/routes.rs`, pages in
  `src/views/`, project-owned components in `src/elements/`. SCSS lives next to the code it
  styles (`src/*.scss`, `src/elements/*.scss`, `src/views/*.scss`); see
  [Styles](#styles--scss-is-the-source-css-is-a-committed-artifact) below.
- `backend/` — shared fullstack server functions (crate `backend`).
- `packages/web`, `packages/desktop`, `packages/mobile` — per-platform binary crates. Each
  gates its platform behind a Cargo feature (`web`, `desktop`, `mobile`) plus a `server`
  feature. `packages/web` is the workspace `default-members` target.
- `scripts/` — `fmt.sh` and `clean.sh` helpers (see below).

## Vendored components — do not edit

`frontend/src/components/` holds default Dioxus components that are **overwritten by upstream
updates**. Never edit these by hand. All tooling deliberately skips this directory
(`scripts/fmt.sh` filters it, the module is `#[allow(...)]`'d in `frontend/src/lib.rs`, and
the Prettier/Stylelint globs ignore it). Put project-owned components in `src/elements/`.

The components are generated from the [`DioxusLabs/components`](https://github.com/DioxusLabs/components)
registry and **must stay at the same revision as the `dioxus-primitives` crate** (both are
pinned to the same `rev` in `frontend/Cargo.toml` — currently `9a758255`). The generated code
also pulls in `dioxus-icons`, `dioxus-code` (for the `css_module` macro), `palette`, and
`time`, plus `dioxus-ssr` as a dev-dependency for the components' own tests.

To update: bump the `rev` on `dioxus-primitives` **and** regenerate the vendored files at the
same rev, then review. Regeneration is `dx components add --all --rev <rev> --module-path
frontend/src/components --global-assets-path frontend/assets` run **from the workspace root**.
Gotcha: `dx` caches the registry at `~/.dx/components/<hash>/` and `--rev` does **not** refresh
it — `git checkout <rev>` inside that cache dir first, or you'll get stale templates. `dx` also
rewrites dependency lines (re-adding `dioxus-primitives` to `packages/web`, dropping the pin);
re-check `git diff` on every `Cargo.toml` afterward.

## Styles — SCSS is the source, CSS is a committed artifact

Stylesheets are authored in `.scss`, but Dioxus loads **`.css`** via the `asset!` macro
(e.g. `asset!("./main.css")` in `lib.rs`, `asset!("./hero.css")` in `elements/hero.rs`). The
compiled, minified `.css` files are **committed next to their `.scss` source** and are what
actually ships — the `.css.map` sourcemaps are gitignored.

Consequences for agents:

- **Editing a `.scss` is not enough — you must recompile the matching `.css`,** or the change
  will not reach the app. The repo's VS Code config (`.vscode/settings.json`) drives the
  _Live Sass Compiler_ extension to emit `compressed` CSS beside each source file; reproduce
  that output if you compile another way (e.g. `sass --style=compressed in.scss out.css`).
- Prettier and Stylelint only touch `.scss` — `.css` is ignored by both (and by
  `.prettierignore`), so the style gates will not catch a stale or hand-edited `.css`.
- Shared SCSS lives in `frontend/src/_theme.scss` and `frontend/src/_mixins.scss`, pulled in
  with `@use` from `main.scss`.

This SCSS→CSS workflow covers **project-owned** styles (`main.scss`, `elements/`, `views/`).
The **vendored components** style themselves differently: each uses the `#[css_module("…/style.css")]`
macro (from `dioxus-code`), which reads its committed `style.css` at compile time and generates
typed class-name constants — there is no `asset!("./style.css")` link. Their shared theme
variables live in `frontend/assets/dx-components-theme.css`, loaded once as a stylesheet in
`frontend/src/lib.rs`. Do not hand-edit either; they are regenerated (see
[Vendored components](#vendored-components--do-not-edit)).

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

You can compile SASS with `sass frontend/src:frontend/src --style=compressed`.

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
- Don't rewrite my comments unless you actually changed something that would make the comment outdated!
