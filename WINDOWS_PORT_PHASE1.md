# Windows Port Phase 1 (Compile-First)

## Goal
Get the app to compile and produce a first unsigned Windows preview artifact with minimal behavioral changes.

## Guardrails
- Do not break existing macOS behavior.
- Use `cfg(target_os = "...")` gates for platform-specific code.
- Keep provider/plugin feature parity for later phases; compile-first now.

## Step Order

1. **Dependency gating (Cargo.toml)**
   - Move `tauri-nspanel` to macOS-only dependencies.
   - Keep base Tauri deps cross-platform.

2. **Module split (src-tauri/src)**
   - Introduce `tray_macos.rs` (current tray behavior) and `tray_windows.rs` (basic tray toggle).
   - Keep `panel.rs` mac-only initially; add simple window toggle path for Windows in lib wiring.

3. **lib.rs platform wiring**
   - Gate mac-only modules and plugin init (`tauri_nspanel::init()`).
   - Route tray creation by platform.
   - Replace direct nspanel use in `hide_panel`/`init_panel` with platform-safe branches.

4. **Path portability fix**
   - Update `get_log_path()` to use Tauri app log dir/path APIs instead of hardcoded `~/Library/Logs`.

5. **Windows config split**
   - Add `src-tauri/tauri.windows.conf.json` and `src-tauri/tauri.macos.conf.json` overlays.
   - Keep `macOSPrivateApi` only in macOS config.

6. **Build checkpoint**
   - Run local checks and `tauri build` on Windows runner.

## Checkpoints / Fail-Fast

- **CP1**: `cargo check` passes on Linux/mac after cfg changes.
- **CP2**: `cargo check --target x86_64-pc-windows-msvc` passes in CI.
- **CP3**: Windows artifact generated (unsigned NSIS/MSI or exe bundle).
- **CP4**: Smoke test: app launches, tray icon shows, window toggles.

## Rollback Plan

- Keep each step in separate commit.
- If CP2 fails, revert latest platform split commit first.
- If CP3 fails due to packaging, keep compile-success branch and isolate CI packaging changes in separate commit.

## CI Preview Sketch (unsigned)

```yaml
name: windows-preview
on:
  workflow_dispatch:
  push:
    branches: ["main", "windows-port/**"]

jobs:
  build-windows-preview:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: oven-sh/setup-bun@v2
        with:
          bun-version: 1.2.22
      - name: Install deps
        run: bun install
      - name: Bundle plugins
        run: bun run bundle:plugins
      - name: Build windows preview (unsigned)
        run: bunx tauri build --target x86_64-pc-windows-msvc --bundles nsis
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: openusage-windows-preview
          path: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/**
```
