# Development

## Code quality

This workspace enforces strict linting and formatting. The same checks run in CI (`.forgejo/workflows/ci.yml`) on every push and pull request.

### Rust

- **Lints** are configured in `[workspace.lints]` in the root `Cargo.toml`. The `clippy::pedantic` and `clippy::nursery` groups run as warnings, with a curated set of lints promoted to hard errors (`unwrap_used`, `expect_used`, `panic`, `print_stdout`, etc.). CI runs clippy with `-D warnings`, so any warning fails the build.

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

- **Formatting** rules live in `rustfmt.toml` (stable-only options). Because the vendored components in `frontend/src/components/` must never be reformatted, use the helper script instead of `cargo fmt`:

```bash
./scripts/fmt.sh          # format our files in place
./scripts/fmt.sh --check  # verify formatting (used by CI)
```

### SCSS

SCSS is linted with [Stylelint](https://stylelint.io/) and formatted with [Prettier](https://prettier.io/). Install the tooling once with `npm install`, then:

```bash
npm run lint:style        # lint SCSS
npm run lint:style:fix    # auto-fix lint issues
npm run format            # format SCSS
npm run format:check      # verify formatting (used by CI)
```

### Vendored components

`frontend/src/components/` contains default Dioxus components that are overwritten by upstream updates. All tooling is configured to leave this directory untouched: `scripts/fmt.sh` filters it out, the `components` module is annotated with `#[allow(...)]` in `frontend/src/lib.rs`, and the Prettier/Stylelint globs ignore it. Do not edit files in this directory by hand.

## Multi-platform Development

To test the application on other platforms, use a virtual machine such as UTM. Then, in the VM settings under "Sharing", select your `onyx/` directory as path.

## Linux

### Install Dependencies

Use the respective package manager of your operating system.

1. Install Rust: `sudo dnf install rustup`
2. Install binstall (recommended): `cargo install cargo-binstall`
3. Install Dioxus: `cargo binstall dioxus-cli --force` or without binstall `cargo install dioxus-cli`

### Run and Develop Program

1. Mount shared directory (if it's not): `sudo mount -t virtiofs share /mnt/share`
2. Run `rsync -a --no-owner --no-group --delete --exclude='target/' --exclude='node_modules/' /mnt/share/ ~/Code/onyx/`
3. Check if directory exists and `cd` into it.
4. To watch for changes, run `./scripts/sync-linux-macos.sh`
5. To start Dioxus, run `dx serve --package desktop`

## Windows 11

### Install Dependencies

1. Open Powershell as administrator
2. Install Rust: `winget install -e --id Rustlang.Rustup`
3. Install Visual Studio Build Tools `winget install -e --id Microsoft.VisualStudio.BuildTools`
4. Add the C++ and ARM64 toolchain workloads
    - Find the actual install path first: `& "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe" -products * -property installationPath`
    - Then add the workloads (replace the path if different):

```powershell
& "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vs_installer.exe" `
  modify `
  --installPath "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools" `
  --add Microsoft.VisualStudio.Workload.VCTools `
  --add Microsoft.VisualStudio.Component.VC.Tools.ARM64 `
  --includeRecommended `
  --passive --norestart
```

5. Install LLVM/clang (required by crates like `ring` on ARM64)
    - Install LLVM: `winget install -e --id LLVM.LLV`
    - Then add it to PATH: `[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\LLVM\bin", [System.EnvironmentVariableTarget]::Machine)`
6. Install binstall (recommended): `cargo install cargo-binstall`
7. Install Dioxus: `cargo binstall dioxus-cli --force` or without binstall `cargo install dioxus-cli`

### Run and Develop Program

1. Important: Don't use administrator
2. Run once: `robocopy Z:\ %USERPROFILE%\Code\onyx /E /PURGE /XD target node_modules /R:3 /W:1`
    - If the `onyx/` directory does not appear in this location, make sure you're not using administrator
3. Check if the `onyx/` directory exists and enter it
4. To watch for changes, run `./scripts/sync-windows.bat`
5. To start Dioxus, run `dx serve --package desktop`
