# Tasks — Secrets hardening

## Task list

### T-1 [SECURITY] Confirm git history exposure
**Role:** SECURITY
**Description:** Run `git log -p --all | grep "ccc2281a177faccd7eb7835515ee1ed9"` and `git log -p --all | grep "b5e0f935d5b74b1cb7c2fc40a0e9b45e"` to confirm exactly which commits contain which credentials. Record findings.
**Dependencies:** none (first step)
**Done when:** Findings documented; confirms Last.fm API key is in history and Spotify client_id is not.

---

### T-2 [DEV] Create `secrets.toml` with real credentials
**Role:** DEV
**Description:** Create `/workspace/secrets.toml` with the three real values copied from `config.toml`. Set `chmod 600` on the file.
**Dependencies:** T-1 (confirms what needs protecting)
**Done when:** `ls -l secrets.toml` shows the file exists; `stat -f "%A" secrets.toml` returns `600`; `cargo run -- --help` (or a quick parse test) shows the app still loads config without error.

---

### T-3 [DEV] Redact sensitive fields in `config.toml`
**Role:** DEV
**Description:** Replace `api_key`, `username` (under `[lastfm]`), and `client_id` (under `[spotify]`) in `config.toml` with the same placeholder strings used in `config.toml.example` (`YOUR_LASTFM_API_KEY`, `YOUR_LASTFM_USERNAME`, `YOUR_SPOTIFY_CLIENT_ID`). All other fields unchanged.
**Dependencies:** T-2 (secrets.toml must exist and work before credentials are removed from config.toml)
**Done when:** `grep api_key config.toml` returns `YOUR_LASTFM_API_KEY`; same for username and client_id; `cargo test` still passes.

---

### T-4 [SECURITY] Add `/config.toml` to `.gitignore`
**Role:** SECURITY
**Description:** Append `/config.toml` to `.gitignore` (below the existing `secrets.toml` entry).
**Dependencies:** T-3
**Done when:** `git check-ignore -v config.toml` prints a match against `.gitignore`; `git status` no longer shows `config.toml` as untracked.

---

### T-5 [DEV] Commit `config.toml.example` and `.gitignore` updates
**Role:** DEV
**Description:** Stage `config.toml.example` (already cleaned up to placeholders in the working tree) and `.gitignore` (now includes `/config.toml`). Commit with message: `security: gitignore config.toml; use placeholders in config.toml.example`.
**Dependencies:** T-4
**Done when:** `git show HEAD:config.toml.example | grep api_key` shows `YOUR_LASTFM_API_KEY`; `git show HEAD:.gitignore | grep config.toml` shows `/config.toml`; commit is in `git log --oneline`.

---

### T-6 [SECURITY] Rewrite git history with `git filter-repo`
**Role:** SECURITY
**Description:** Install `git-filter-repo` if absent (`pip install git-filter-repo` or `brew install git-filter-repo`). Run:
```
git filter-repo \
  --replace-text <(printf 'ccc2281a177faccd7eb7835515ee1ed9==>YOUR_LASTFM_API_KEY\n') \
  --force
```
This rewrites all commit SHAs. No remote exists, so blast radius is zero.
**Dependencies:** T-5 (must commit clean state first so filter-repo sees it)
**Done when:** `git log -p --all | grep "ccc2281a177faccd7eb7835515ee1ed9"` produces no output.

---

### T-7 [SECURITY] Prune unreachable objects from local object store
**Role:** SECURITY
**Description:** After filter-repo rewrites refs, purge old blobs that are now unreachable:
```
git reflog expire --expire=now --all
git gc --prune=now
```
**Dependencies:** T-6
**Done when:** Commands complete without error; `git fsck --unreachable 2>&1 | wc -l` returns 0 or a small number of pack-related lines (no dangling blob for the API key).

---

### T-8 [QA] Verify no credentials in tracked working tree
**Role:** QA
**Description:** Run AC-5 and AC-6 checks:
```
git grep "ccc2281a177faccd7eb7835515ee1ed9"  # must be empty
git grep "b5e0f935d5b74b1cb7c2fc40a0e9b45e"  # must be empty
```
**Dependencies:** T-5
**Done when:** Both commands produce no output.

---

### T-9 [QA] Verify no credentials in git history
**Role:** QA
**Description:** Run AC-7 check:
```
git log -p --all | grep "ccc2281a177faccd7eb7835515ee1ed9"
```
**Dependencies:** T-6, T-7
**Done when:** Command produces no output.

---

### T-10 [DEV] Run `cargo build` and `cargo test` — final verification
**Role:** DEV
**Description:** Run `cargo build` and confirm warning count ≤ 1 (pre-existing `last_track_uri` warning). Run `cargo test` and confirm 16/16 pass.
**Dependencies:** T-3 (secrets.toml in place, config.toml redacted — app must still compile and pass tests via overlay)
**Done when:** `cargo test` reports `test result: ok. 16 passed; 0 failed`.

---

### T-11 [DOCS] Update README with secrets setup instructions
**Role:** DEV (DOCS)
**Description:** Add a "Configuration" or "Getting started" section to `README.md` (or create it if absent) explaining: copy `config.toml.example` → `config.toml`, create `secrets.toml` with real credentials, run `cargo run --release`. Reference the `secrets.toml` format documented in `config.toml.example`.
**Dependencies:** T-5
**Done when:** `README.md` has a section explaining the secrets.toml setup pattern; new users can follow it without consulting the source.
