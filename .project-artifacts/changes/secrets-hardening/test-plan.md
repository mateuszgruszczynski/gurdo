# Test Plan — Secrets hardening

Policy: thorough / full

## Level notes

This change introduces no new Rust code. All observable outcomes are file-system state or git
object-database state. Scenarios S-01 through S-10 are System-integration / CLI — they require
the assembled working tree + git object store; they cannot be verified in-process. S-11 through
S-13 are Component-level (in-process) and owned by Development; S-12 and S-13 are pre-existing
regression tests that cover the `Config::load()` overlay path this change relies on.

---

## Scenarios

### S-01 — `config.toml` is permanently gitignored
**Covers AC:** AC-1
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given the .gitignore file contains `/config.toml`
When  I run `git check-ignore -v config.toml`
Then  the output names `.gitignore` as the matching file
And   the pattern `/config.toml` appears in the output
```
**Notes:** Also run `git status` and confirm `config.toml` does not appear as an untracked file.

---

### S-02 — `secrets.toml` is gitignored
**Covers AC:** AC-2
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given secrets.toml exists in the project root
When  I run `git check-ignore -v secrets.toml`
Then  the output names `.gitignore` as the matching file
And   secrets.toml does not appear in `git status` output
```

---

### S-03 — `secrets.toml` has restricted permissions
**Covers AC:** AC-2
**Level:** System-integration | **Type:** File-batch
**Owned by:** Verification

```
Given secrets.toml exists in the project root
When  I inspect the file permissions with `stat -f "%A" secrets.toml`
Then  the result is `600`
```
**Notes:** On Linux use `stat -c "%a" secrets.toml`. Fail if result is `644` or wider.

---

### S-04 — `secrets.toml` holds real credential values
**Covers AC:** AC-3
**Level:** System-integration | **Type:** File-batch
**Owned by:** Verification

```
Given secrets.toml exists in the project root
When  I inspect its content for each of the three credential fields
Then  the api_key field is not a placeholder string (does not contain "YOUR_")
And   the username field is not a placeholder string
And   the client_id field is not a placeholder string
```
**Notes:** `grep api_key secrets.toml | grep -v "YOUR_"` must produce non-empty output; same for username and client_id.

---

### S-05 — `config.toml` contains only placeholder values for credentials
**Covers AC:** AC-4
**Level:** System-integration | **Type:** File-batch
**Owned by:** Verification

```
Given config.toml exists in the project root
When  I inspect the api_key, username, and client_id fields
Then  api_key contains the string "YOUR_LASTFM_API_KEY"
And   username contains the string "YOUR_LASTFM_USERNAME"
And   client_id contains the string "YOUR_SPOTIFY_CLIENT_ID"
```
**Notes:** Complements AC-3. Even though `config.toml` is gitignored, it must not contain real credentials to prevent sharing via other channels.

---

### S-06 — No tracked file contains the real Last.fm API key
**Covers AC:** AC-5
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given all changes have been committed
When  I run `git grep "ccc2281a177faccd7eb7835515ee1ed9"`
Then  no output is produced
```
**Notes:** An empty result confirms the key is absent from every file currently tracked by git. If any output appears, the match must be investigated before publishing.

---

### S-07 — No tracked file contains the real Spotify client_id
**Covers AC:** AC-6
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given all changes have been committed
When  I run `git grep "b5e0f935d5b74b1cb7c2fc40a0e9b45e"`
Then  no output is produced
```

---

### S-08 — Git history contains no real Last.fm API key after rewrite
**Covers AC:** AC-7
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given git filter-repo has been run to scrub the API key from all commits
And   git reflog expire + git gc have been run to purge unreachable blobs
When  I run `git log -p --all | grep "ccc2281a177faccd7eb7835515ee1ed9"`
Then  no output is produced
```
**Notes:** This is the definitive gate before any remote push. If output is produced, the rewrite did not complete successfully and the repo must not be published.

---

### S-09 — `config.toml.example` committed to HEAD uses placeholders
**Covers AC:** AC-8
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given config.toml.example has been staged and committed
When  I run `git show HEAD:config.toml.example`
And   inspect the api_key, username, and client_id lines
Then  all three contain "YOUR_" placeholder strings
And   none match a 32-character hex string (real API key pattern)
```

---

### S-10 — `cargo build` produces no new warnings
**Covers AC:** AC-9
**Level:** System-integration | **Type:** CLI
**Owned by:** Verification

```
Given secrets.toml is in place and config.toml has placeholder values
When  I run `cargo build`
Then  the warning count is at most 1 (the pre-existing `last_track_uri` warning)
And   no new warnings appear in the output
```
**Notes:** Baseline warning count is 1. Record the count before and after. Any increase is a failure.

---

### S-11 — All 16 tests pass with the secrets.toml overlay in place
**Covers AC:** AC-10
**Level:** Component | **Type:** CLI
**Owned by:** Development

```
Given secrets.toml exists with real credentials
And   config.toml has placeholder values for the three credential fields
When  I run `cargo test`
Then  the result shows 16 passed and 0 failed
```
**Notes:** Exercises the full in-process test suite including the two overlay-specific tests below.

---

### S-12 (regression) — Config::load() overlays secrets.toml fields over config.toml
**Covers AC:** AC-10 (regression for existing overlay mechanism)
**Level:** Component | **Type:** CLI
**Owned by:** Development (pre-existing test: `load_overlays_secrets_when_present`)

```
Given a config.toml with placeholder credential values
And   a secrets.toml alongside it with real values for api_key, username, client_id
When  Config::load() reads the config
Then  the loaded Config has the real values from secrets.toml
And   the placeholder values from config.toml are overridden
```
**Notes:** Covered by existing test `config::tests::load_overlays_secrets_when_present`. Must continue to pass — this is the runtime mechanism the entire change relies on.

---

### S-13 (regression) — Config::load() uses config.toml values when secrets.toml is absent
**Covers AC:** AC-10 (regression for overlay absence path)
**Level:** Component | **Type:** CLI
**Owned by:** Development (pre-existing test: `load_uses_config_values_when_secrets_absent`)

```
Given a config.toml with direct (non-placeholder) credential values
And   no secrets.toml file alongside it
When  Config::load() reads the config
Then  the loaded Config has the values from config.toml directly
```
**Notes:** Covered by existing test `config::tests::load_uses_config_values_when_secrets_absent`. Ensures the fallback path is not broken by this change.

---

## AC coverage summary

| AC | Scenario(s) | Level | Phase |
|----|-------------|-------|-------|
| AC-1 | S-01 | System-integration | Verification |
| AC-2 | S-02, S-03 | System-integration | Verification |
| AC-3 | S-04 | System-integration | Verification |
| AC-4 | S-05 | System-integration | Verification |
| AC-5 | S-06 | System-integration | Verification |
| AC-6 | S-07 | System-integration | Verification |
| AC-7 | S-08 | System-integration | Verification |
| AC-8 | S-09 | System-integration | Verification |
| AC-9 | S-10 | System-integration | Verification |
| AC-10 | S-11, S-12 (regression), S-13 (regression) | Component | Development |
