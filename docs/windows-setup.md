# Windows Preview Setup (What we used)

This is the exact flow used to get the Windows preview build running.

## 1) Download and install preview build

1. Open Actions run artifacts for a successful `windows-preview` run.
2. Download artifact: `usageforge-windows-preview`.
3. Extract zip.
4. Run installer: `UsageForge_0.6.8_x64-setup.exe` (or the latest UsageForge installer name).

## 2) Install Node.js (required for Codex/Gemini CLIs)

Install Node LTS for Windows:
- https://nodejs.org/en/download

Open a **new** PowerShell after install and verify:

```powershell
node -v
```

## 3) Install Codex CLI

PowerShell execution policy may block `npm.ps1`, so use `npm.cmd` directly:

```powershell
npm.cmd install -g @openai/codex
codex login
```

Complete browser auth.

## 4) Install Gemini CLI

```powershell
npm.cmd install -g @google/gemini-cli
gemini.cmd
```

Complete auth flow in browser. If prompted to trust `System32`, choose **Don’t trust** and run from your user folder:

```powershell
cd $HOME
gemini.cmd
```

## 5) Refresh UsageForge UI

In app, click refresh and confirm providers show usage instead of "Not logged in".

## Notes / gotchas we hit

- `npm` not found -> install Node and open new shell.
- `npm.ps1 cannot be loaded` -> use `npm.cmd`.
- `codex` or `gemini` not recognized -> use `codex.cmd` / `gemini.cmd`.
- Preview CI originally failed on updater signing until Windows config disabled updater artifacts and base updater config was moved out of shared config.

## Project naming / branding for forks

This repository includes `TRADEMARK.md`.
Summary:
- Do **not** ship a fork under the name **OpenUsage**.
- Do **not** use OpenUsage logo/brand for your derivative.
- Do credit the original project and maintainers.
