# Technical Specification: Preparation for Public Distribution

> **Purpose:** Systematic audit and hardening checklist for converting two private repositories to public visibility on GitHub. Designed to be executed by Claude Code as an agentic task.

## Context

Two repos are being transitioned from **private → public** on GitHub:

- **`jeremycastanza/selah`** — The Rust TUI application (source code, CI, releases)
- **`jeremycastanza/homebrew-selah-tap`** — The Homebrew tap (formula for `brew install`)

### Key Decision: Keep Repos Separate

The selah source and Homebrew tap will **remain as separate repositories**. Rationale:

1. **Homebrew convention** — Homebrew expects taps at `<user>/homebrew-<name>`. Merging would break `brew tap` ergonomics or require naming the source repo `homebrew-selah`.
2. **`cargo-dist` automation** — `dist-workspace.toml` already has `[dist.homebrew]` configured. Setting `publish = true` lets cargo-dist auto-update the formula on tag push — zero manual maintenance.
3. **Going public eliminates the PAT pain** — The current `Authorization: token` headers in the formula and PAT instructions in the tap README exist only because the repos are private. Once public, formula URLs become simple release download links and no auth is needed.
4. **Separation of concerns** — The tap is a distribution channel, not source code. Keeping it separate keeps release automation clean.

### Tap Rename

The tap repo must be renamed from `homebrew-selah-tap` → **`homebrew-selah`** so that `brew tap jeremycastanza/selah` works cleanly (Homebrew strips the `homebrew-` prefix to derive the tap name).

Before flipping visibility, every phase below must be completed and verified across **both repos**. The goal is to ensure zero credential leakage, a secure CI/CD posture, and a welcoming open-source surface area.

---

## Phase 1: Secret & Credential Scan

**Objective:** Identify any secrets, tokens, keys, or credentials that have ever existed in the Git history — not just the working tree.

### Steps

1. Install and run [`gitleaks`](https://github.com/gitleaks/gitleaks) against the **full commit history**:
   ```bash
   gitleaks detect --source . --verbose --report-path gitleaks-report.json
   ```
2. Review `gitleaks-report.json`. For each finding, classify as:
   - **True positive** — an actual secret (API key, token, password, connection string, private key, etc.)
   - **False positive** — a placeholder, example value, or hash that isn't sensitive
3. Document all true positives in a `SECRET_AUDIT.md` (do NOT commit this file) with:
   - The commit SHA where it appeared
   - The file path
   - The type of secret (e.g., "Stripe secret key", "Azure SP credential", "GitHub PAT")
   - Whether it's still active or already rotated

### What to look for specifically

- `.env` files or `.env.*` variants committed at any point
- Hardcoded values in CI/CD workflow files (`.github/workflows/*.yml`)
- API keys, tokens, or passwords in source code, config files, or scripts
- Private keys (`*.pem`, `*.key`, `id_rsa`, etc.)
- Connection strings (database URLs, Redis URLs, Azure connection strings)
- Homebrew formula/cask files with authenticated download URLs or embedded tokens
- Base64-encoded secrets or obfuscated credentials
- Internal hostnames, staging URLs, or infrastructure endpoints that reveal organizational structure

---

## Phase 2: History Rewrite (If Needed)

**Objective:** Remove any true-positive secrets from the Git history entirely.

> ⚠️ **Only proceed if Phase 1 found true positives.** History rewrites are destructive and require coordination with all contributors.

### Steps

1. Use [`git filter-repo`](https://github.com/newren/git-filter-repo) (NOT `git filter-branch`) to remove or redact sensitive content:

   ```bash
   # Remove entire files that should never have been committed
   git filter-repo --invert-paths --path <sensitive-file>

   # Or use blob callback for surgical redaction of specific strings
   git filter-repo --replace-text expressions.txt
   ```

   Where `expressions.txt` contains `literal:ACTUAL_SECRET_VALUE==>REDACTED` entries.

2. After rewriting, verify the secret is gone:
   ```bash
   gitleaks detect --source . --verbose
   ```
3. Force-push the rewritten history. All collaborators must re-clone.

### Mandatory: Rotate ALL Exposed Credentials

Regardless of whether history is rewritten, **every secret identified as a true positive must be rotated immediately.** Assume it has already been scraped. This includes:

- API keys (Stripe, Azure, Neon, GitHub, etc.)
- Service principal credentials
- Database passwords and connection strings
- Personal access tokens
- SSH keys if committed
- Any OAuth client secrets

---

## Phase 3: Working Tree & Tracked File Audit

**Objective:** Ensure the current state of the repo is clean and `.gitignore` is comprehensive.

### Steps

1. Run `git ls-files` and review the full list of tracked files. Flag anything that shouldn't be public:
   - `.env`, `.env.local`, `.env.production`, `.env.*.local`
   - `*.pem`, `*.key`, `*.p12`, `*.pfx`, `*.jks`
   - `*.sqlite`, `*.db` (local databases with real data)
   - Config files with hardcoded environment-specific values
   - Internal documentation, meeting notes, or proprietary specs
   - Vendor-specific license files tied to paid accounts
2. Verify `.gitignore` covers all of the above patterns. A recommended baseline:

   ```gitignore
   # Environment
   .env
   .env.*
   !.env.example

   # Secrets & keys
   *.pem
   *.key
   *.p12
   *.pfx

   # Local databases
   *.sqlite
   *.db

   # OS & editor
   .DS_Store
   Thumbs.db
   .vscode/settings.json
   .idea/

   # Dependencies
   node_modules/
   .pnpm-store/

   # Build output
   dist/
   .next/
   out/

   # Debug & logs
   *.log
   npm-debug.log*
   ```

3. If sensitive files are currently tracked, remove them:
   ```bash
   git rm --cached <file>
   ```
   Then add the pattern to `.gitignore` and commit.

---

## Phase 4: CI/CD & GitHub Actions Hardening

**Objective:** Ensure workflows are safe for public-repo execution model, where anyone can fork and submit PRs.

### Steps

1. **Audit all workflow files** in `.github/workflows/`:
   - Verify NO secrets are hardcoded — all must use `${{ secrets.SECRET_NAME }}` or `${{ vars.VAR_NAME }}`
   - Check that no step logs or echoes secret values (watch for debug commands like `echo $SECRET` or `env | grep`)
   - Remove or redact any references to internal infrastructure (staging URLs, internal service names, private registry endpoints)
2. **Check for dangerous triggers:**
   - `pull_request_target` — runs with write permissions in the context of the _base_ repo, not the fork. If combined with `actions/checkout` of the PR head, an attacker's PR can exfiltrate secrets. Either remove this trigger or ensure it never checks out untrusted code.
   - `workflow_dispatch` with unconstrained inputs — validate inputs in the workflow.
   - `issue_comment` or `issues` triggers that execute arbitrary content.
3. **Set restrictive defaults** (configure in GitHub UI after publicizing):
   - Repository Settings → Actions → General:
     - Set "Fork pull request workflows from outside collaborators" to **"Require approval for first-time contributors"** (or stricter)
     - Set default `GITHUB_TOKEN` permissions to **"Read repository contents and packages permissions"**
   - Use `permissions:` block in each workflow to explicitly grant only what's needed:
     ```yaml
     permissions:
       contents: read
       packages: write # only if needed
     ```

4. **Pin third-party actions to full commit SHAs**, not tags:

   ```yaml
   # Bad — tags can be moved to point at malicious code
   - uses: actions/checkout@v4

   # Good — immutable reference
   - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1
   ```

---

## Phase 5: Homebrew Tap Audit & Migration

**Objective:** Rename the tap repo, rewrite the formula for public access, update all documentation, and verify unauthenticated installs.

### 5a: Rename the Tap Repository

1. Rename `jeremycastanza/homebrew-selah-tap` → `jeremycastanza/homebrew-selah` via GitHub Settings → General → Repository name.
2. GitHub will set up a redirect from the old name, but update all references proactively:
   - `dist-workspace.toml`: change `tap = "jeremycastanza/homebrew-selah-tap"` → `tap = "jeremycastanza/homebrew-selah"`
   - `README.md`: update `brew tap` command from `jeremycastanza/selah-tap` → `jeremycastanza/selah`
   - Any CI/CD references to the old tap name

### 5b: Rewrite the Formula for Public Access

The current `Formula/selah.rb` uses authenticated GitHub API asset URLs with `Authorization: token` headers. This must be replaced with standard public release URLs.

1. Replace the current formula's `url` entries:
   ```ruby
   # BEFORE (private — requires PAT)
   url "https://api.github.com/repos/jeremycastanza/homebrew-selah-tap/releases/assets/...",
       headers: ["Authorization: token #{ENV["HOMEBREW_GITHUB_API_TOKEN"]}", "Accept: application/octet-stream"]

   # AFTER (public — no auth needed)
   url "https://github.com/jeremycastanza/selah/releases/download/v#{version}/selah-aarch64-apple-darwin.tar.gz"
   ```
2. Update SHA256 checksums to match the public release asset hashes.
3. Remove all `headers:` blocks and any references to `HOMEBREW_GITHUB_API_TOKEN`.

### 5c: Update Tap README

Rewrite `homebrew-selah/README.md` to remove all PAT/authentication instructions:
- Remove the "Prerequisites" section about GitHub PATs
- Remove the "Setup" section about exporting `HOMEBREW_GITHUB_API_TOKEN`
- Simplify install instructions to:
  ```bash
  brew tap jeremycastanza/selah
  brew install selah
  ```

### 5d: Enable cargo-dist Auto-Publishing

In the selah repo's `dist-workspace.toml`, set:
```toml
[dist.homebrew]
tap = "jeremycastanza/homebrew-selah"
publish = true
```

This lets cargo-dist auto-generate and push the formula on each tagged release. The tap repo must grant the selah release workflow write access (see Phase 7).

### 5e: Verify Unauthenticated Install

After publicizing, test a clean install with no GitHub tokens:
```bash
# In a clean environment or container
unset HOMEBREW_GITHUB_API_TOKEN
brew tap jeremycastanza/selah
brew install selah
selah --version
```

---

## Phase 6: Repository Metadata & Open-Source Readiness

**Objective:** Add the standard files and configuration that signal a well-maintained public project.

### Steps

1. **`LICENSE`** — Add an explicit license file. Without one, the code is "all rights reserved" by default even when public. Choose one:
   - `MIT` — permissive, minimal obligations
   - `Apache-2.0` — permissive with patent grant
   - `GPL-3.0` — copyleft, derivatives must also be open
   - Use https://choosealicense.com if unsure.
2. **`SECURITY.md`** — Provide responsible disclosure instructions:

   ```markdown
   # Security Policy

   ## Reporting a Vulnerability

   If you discover a security vulnerability, please report it responsibly.

   **Do NOT open a public GitHub issue.**

   Instead, email [security contact] with:

   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact

   You can expect an initial response within 72 hours.
   ```

3. **`CONTRIBUTING.md`** — Set expectations for outside contributions:
   - How to set up the dev environment
   - Coding standards and linting rules
   - PR process and review expectations
   - Code of conduct reference (if applicable)
4. **`README.md`** — Review and update:
   - Remove any internal jargon, private URLs, or references to internal systems
   - Ensure setup instructions work without access to private infrastructure
   - Add badges (CI status, license, version) if desired
5. **`.env.example`** — If the project uses environment variables, provide a template:
   ```env
   # Copy to .env.local and fill in values
   STRIPE_SECRET_KEY=sk_test_...
   DATABASE_URL=postgresql://...
   ```

---

## Phase 7: Branch Protection & Repository Settings

**Objective:** Configure GitHub repository settings appropriate for public projects. Branch protection / rulesets are only available once the repos are public (GitHub Free plan limitation for private repos).

### 7a: Branch Rulesets — `selah` (Main Repo)

Apply a ruleset to `main` via GitHub UI (Settings → Rules → Rulesets) or API:

1. **Require pull request before merging:**
   - Minimum approvals: 0 initially (solo maintainer), increase when contributors join
   - Dismiss stale reviews on new pushes: enabled
2. **Require status checks to pass:**
   - Add the `Release / plan` job as a required check (runs on push to main)
3. **Block force pushes** to `main`
4. **Block branch deletion** of `main`
5. **Do not allow bypassing** — even admin pushes go through PRs

### 7b: Branch Rulesets — `homebrew-selah` (Tap Repo)

The tap repo needs the same protections but with a **bypass for automation**:

1. **Require pull request before merging** — same as above
2. **Block force pushes and branch deletion**
3. **Add a bypass actor** for the cargo-dist release workflow:
   - Allow the GitHub Actions bot (or a dedicated deploy key / GitHub App) to push directly to `main`
   - This is required because cargo-dist pushes formula updates to the tap on each tagged release of selah
   - Configure via: Rulesets → Bypass list → Add "GitHub Actions" or a specific app

### 7c: Repository Settings (Both Repos)

1. **GitHub Actions permissions** (Settings → Actions → General):
   - Default `GITHUB_TOKEN` permissions: **Read repository contents and packages**
   - Fork PR workflow approval: **Require approval for first-time contributors**
2. **Enable "Private vulnerability reporting"** under Security settings
3. **Disable unused features** (wiki, projects, discussions) unless planned
4. **Review collaborators** — remove any that shouldn't have access to the public repos

### 7d: Cross-Repo Access for cargo-dist

For cargo-dist in the selah repo to push formula updates to the tap repo:

1. Create a fine-grained PAT or GitHub App with `contents: write` on `homebrew-selah` only
2. Store it as a repository secret in the selah repo (e.g., `HOMEBREW_TAP_TOKEN`)
3. Reference it in the release workflow where cargo-dist publishes the formula

---

## Phase 8: Final Verification

**Objective:** One last pass before making the repo public.

### Checklist

**Both repos:**
- [ ] `gitleaks detect` returns zero true positives on the full history
- [ ] All previously-exposed credentials have been rotated and the old values no longer work
- [ ] `git ls-files` contains no sensitive files
- [ ] `.gitignore` is comprehensive
- [ ] All GitHub Actions workflows use `${{ secrets.* }}` — no hardcoded values
- [ ] No `pull_request_target` + untrusted checkout pattern exists
- [ ] Workflow permissions are explicitly scoped with `permissions:` blocks
- [ ] Third-party actions are pinned to SHAs
- [ ] `LICENSE` file exists and is correct
- [ ] `SECURITY.md` exists with disclosure instructions
- [ ] `README.md` contains no internal references or private URLs
- [ ] Branch rulesets are ready to apply post-publicize

**selah repo:**
- [ ] `dist-workspace.toml` has `publish = true` and `tap = "jeremycastanza/homebrew-selah"`
- [ ] `.env.example` provided if env vars are required
- [ ] Cross-repo PAT/App for tap publishing stored as `HOMEBREW_TAP_TOKEN` secret

**homebrew-selah (tap) repo:**
- [ ] Renamed from `homebrew-selah-tap` → `homebrew-selah`
- [ ] Formula uses public release download URLs (no `Authorization` headers)
- [ ] No references to `HOMEBREW_GITHUB_API_TOKEN` in formula or README
- [ ] README install instructions use `brew tap jeremycastanza/selah`
- [ ] Bypass actor configured for cargo-dist to push formula updates
- [ ] Clean `brew install` works without any GitHub authentication

---

## Execution Notes for Claude Code

When executing this spec:

1. **Run phases sequentially.** Phase 2 depends on Phase 1 findings. Phase 8 validates all prior phases.
2. **Do not commit `SECRET_AUDIT.md` or `gitleaks-report.json`** — these contain sensitive finding details.
3. **Pause and report** after Phase 1 if true positives are found — credential rotation requires human action.
4. **Pause and confirm** before any history rewrite in Phase 2 — this is destructive and irreversible.
5. **Both repos require Phases 1–4 independently** — run secret scans, history checks, working tree audits, and CI hardening on each.
6. **Phase 5 spans both repos** — rename + formula rewrite on the tap, config updates on selah.
7. Apply Phase 6 and Phase 7 to each repo.
8. **Phase 7 (branch rulesets) can only be applied after repos are made public** — GitHub Free does not support branch protection on private repos.
9. **Order of operations for go-live:**
   a. Complete Phases 1–6 while repos are still private
   b. Rename the tap repo (`homebrew-selah-tap` → `homebrew-selah`)
   c. Make both repos public simultaneously
   d. Immediately apply Phase 7 rulesets via API or UI
   e. Run Phase 8 verification
