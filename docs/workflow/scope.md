# Current Scope

_No active iteration._

## Latest Release

v0.4.0 — Released 2026-05-04. Plan at `docs/plans/v0.4.0-implementation.md`.

Included:
- `--version` / `-V` CLI flag
- `?` tabbed menu overlay (Navigation, Actions, App, Overlays)
- Clickable status bar segments (menu, translation, theme)
- Public distribution prep (gitleaks audit, CI hardening, .gitignore, LICENSE, SECURITY.md)

## Outstanding (Post-Release)

- ~~Homebrew tap rename (`homebrew-selah-tap` → `homebrew-selah`)~~ ✅ Done
- ~~Tap formula rewrite for public URLs (remove PAT auth)~~ ✅ Done
- ~~`dist-workspace.toml`: set `publish = true`~~ ✅ Done
- Make tap repo **public** — GitHub → Settings → Danger Zone
- Create fine-grained PAT with `contents:write` on `homebrew-selah` → store as `HOMEBREW_TAP_TOKEN` in selah repo
- Branch rulesets on both repos (after tap is public)
- Verify clean `brew install` without auth

## Will NOT Build (Next)

- OAuth / Sign In With YouVersion
- Syncing highlights/bookmarks to YouVersion
- Translation search/discovery UI
- Multiple simultaneous providers
- FTS5 search on API-fetched translations
- x86_64 build targets

## Previous Releases

- v0.3.1 — Translation API (Phases 1–6). Archived at `docs/archive/plans/v0.3.0-implementation.md`.
- v0.2.0 — Released 2026-04-27. Archived at `docs/archive/plans/v0.2.0-implementation.md`.
