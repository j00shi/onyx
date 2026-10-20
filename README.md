# Development

## Code quality

This workspace enforces strict linting and formatting. The same checks run in CI
(`.forgejo/workflows/ci.yml`) on every push and pull request.

### Rust

- **Lints** are configured in `[workspace.lints]` in the root `Cargo.toml`. The
  `clippy::pedantic` and `clippy::nursery` groups run as warnings, with a curated
  set of lints promoted to hard errors (`unwrap_used`, `expect_used`, `panic`,
  `print_stdout`, etc.). CI runs clippy with `-D warnings`, so any warning fails
  the build.

    ```bash
    cargo clippy --workspace --all-targets -- -D warnings
    ```

- **Formatting** rules live in `rustfmt.toml` (stable-only options). Because the
  vendored components in `frontend/src/components/` must never be reformatted, use the
  helper script instead of `cargo fmt`:

    ```bash
    ./scripts/fmt.sh          # format our files in place
    ./scripts/fmt.sh --check  # verify formatting (used by CI)
    ```

### SCSS

SCSS is linted with [Stylelint](https://stylelint.io/) and formatted with
[Prettier](https://prettier.io/). Install the tooling once with `npm install`,
then:

```bash
npm run lint:style        # lint SCSS
npm run lint:style:fix    # auto-fix lint issues
npm run format            # format SCSS
npm run format:check      # verify formatting (used by CI)
```

### Vendored components

`frontend/src/components/` contains default Dioxus components that are overwritten by
upstream updates. All tooling is configured to leave this directory untouched:
`scripts/fmt.sh` filters it out, the `components` module is annotated with
`#[allow(...)]` in `frontend/src/lib.rs`, and the Prettier/Stylelint globs ignore it.
Do not edit files in this directory by hand.
