# Spec — Secrets hardening: move credentials out of config.toml

## Context

`config.toml` holds live Last.fm `api_key`, `username`, and Spotify `client_id`. It is untracked but not gitignored — one `git add .` from leaking. The `secrets.toml` overlay in `Config::load()` is already implemented and tested; `secrets.toml` is already gitignored. The only gaps are: the file layout not yet established, `config.toml` not gitignored, and a Last.fm API key already baked into git history (commit with `config.toml.example` before it was cleaned up).

**Critical finding:** The Last.fm API key is in git history. A history rewrite via `git filter-repo` is required before publishing.

## Before / After

| Item | Before | After |
|---|---|---|
| `config.toml` | Untracked, not gitignored | Untracked, gitignored via `/config.toml` in `.gitignore` |
| `config.toml` content | Real api_key, username, client_id | Placeholder values only |
| `secrets.toml` | Does not exist | Created, `chmod 600`, gitignored — holds 3 real values |
| `config.toml.example` | Modified with placeholders (unstaged) | Committed with placeholder values |
| Last.fm API key in git history | Present in initial baseline commit | Scrubbed via `git filter-repo` |
| App functionality | Working | Working — `Config::load()` reads from `secrets.toml` |

## Out of scope

- Changes to `Config::load()`, `SecretsConfig`, or any Rust source.
- Moving `config.toml` or `secrets.toml` to a different location.
- Env-var or keychain secret backends.
- Schema changes to `secrets.toml`.
- Packaging or distribution.
- Scrubbing the Last.fm username from `.project-artifacts/` prose (contextual text, not a functional secret).

## Edge cases

- `secrets.toml` does not currently exist — Step 1 is safe to execute unconditionally.
- `config.toml` has non-default engine/UI tuning — Step 2 only replaces the 3 credential fields; all other fields remain intact.
- `git filter-repo` rewrites all commit SHAs automatically — no manual branch fixup needed since there is no remote.
- If any remote has ever received the key, rotation at last.fm is mandatory. Spec assumes no remote push has occurred.

## Security notes

- `secrets.toml` must be `chmod 600`.
- The Last.fm API key has been in the local git object store since the initial commit. Consider rotating it at last.fm regardless — history rewrite cleans the repo but the key should be treated as potentially exposed.
- Spotify client_id was never committed; no rotation needed.
- Run `git reflog expire --expire=now --all && git gc --prune=now` after `git filter-repo` to purge unreachable blobs from the local object store.
- Publish only after AC-7 confirms no key in history.

## Acceptance Criteria

**AC-1: `config.toml` is gitignored**
Check: `git check-ignore -v config.toml` prints a line referencing `.gitignore` with the `/config.toml` pattern.
Rationale: Primary safeguard. Without this, `config.toml` (even with placeholders) is one `git add .` from being tracked.

**AC-2: `secrets.toml` exists, is gitignored, and has mode 600**
Check: `ls -l secrets.toml` confirms existence; `git check-ignore -v secrets.toml` references `.gitignore`; `stat -f "%A" secrets.toml` (macOS) returns `600`.
Rationale: `secrets.toml` holds live credentials at runtime and must never enter version control.

**AC-3: `secrets.toml` holds the 3 real credential values (no placeholders)**
Check: `grep api_key secrets.toml | grep -v YOUR_` produces non-empty output; same for username and client_id.
Rationale: The app must continue to work. Placeholder values in `secrets.toml` would cause all API calls to fail.

**AC-4: `config.toml` contains only placeholder values for the 3 sensitive fields**
Check: `grep api_key config.toml` returns `YOUR_LASTFM_API_KEY`; same for username and client_id.
Rationale: `config.toml` must not contain real credentials even though it is gitignored.

**AC-5: No tracked file in the working tree contains the real Last.fm API key**
Check: `git grep "ccc2281a177faccd7eb7835515ee1ed9"` produces no output.
Rationale: Tracked files are what gets pushed to a remote.

**AC-6: No tracked file in the working tree contains the real Spotify client_id**
Check: `git grep "b5e0f935d5b74b1cb7c2fc40a0e9b45e"` produces no output.
Rationale: Belt-and-suspenders confirmation no accidental inclusion occurred.

**AC-7: Git history contains no occurrence of the real Last.fm API key**
Check: `git log -p --all | grep "ccc2281a177faccd7eb7835515ee1ed9"` produces no output.
Rationale: Without history rewrite, the key is permanently reachable by anyone who clones the repo.

**AC-8: `config.toml.example` in HEAD uses placeholder values**
Check: `git show HEAD:config.toml.example | grep -E "(api_key|username|client_id)"` shows `YOUR_*` strings only.
Rationale: `config.toml.example` is the template for new users; must be a clean starting point.

**AC-9: `cargo build` produces no new warnings**
Check: Baseline is 1 pre-existing warning (`last_track_uri`). Warning count after change must be ≤ 1.
Rationale: No Rust source changes, so no new warnings are expected.

**AC-10: All 16 existing tests pass**
Check: `cargo test 2>&1 | grep "^test result"` prints `test result: ok. 16 passed; 0 failed`.
Rationale: The two `config.rs` secrets-overlay tests directly exercise the runtime mechanism.
