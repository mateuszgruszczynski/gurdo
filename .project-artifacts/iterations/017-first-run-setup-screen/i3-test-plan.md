# i3-test-plan: EP-20 — First-run Setup Screen + User-Scoped Config/Secrets

**Iteration:** 017 · **Date:** 2026-05-17
**Policy:** thorough / full

> **Desktop app note:** egui UI scenarios cannot be run headlessly (no display in the dev container). Per Architecture §8, GUI rendering is verified by the developer running the app on the host. All **E2E / UI** scenarios below are owned by Verification but executed as *manual smoke checks* during Integration. All **Unit** and **Component** scenarios are owned by Development and run via `cargo test`.

---

## Development-owned scenarios (Unit / Component)

---

### TS-01 — `parse_config_arg()` default resolves to `~/.gurdo/config.toml`

**Covers AC:** AC-1
**Level:** Unit · **Type:** CLI · **Owned by:** Development

```
Given no -c / --config argument is passed on the command line
When parse_config_arg() is called
Then the returned path ends with ".gurdo/config.toml"
And it does not equal the literal string "config.toml"
```

**Notes:** Updates the pre-existing `parse_config_arg_default` test. Assert `path.ends_with(".gurdo/config.toml")`; do not hard-code `$HOME`.

---

### TS-02 — `-c` flag override still returns the custom path verbatim

**Covers AC:** AC-1 (regression)
**Level:** Unit · **Type:** CLI · **Owned by:** Development

```
Given the command line contains "-c /tmp/custom/my.toml"
When parse_config_arg() is called
Then the returned path equals "/tmp/custom/my.toml"
```

**Notes:** Regression guard — confirms the `-c` fast-path is not broken by the default-path change.

---

### TS-03 — `Config::secrets_path` returns `~/.gurdo/secrets.toml` for any input

**Covers AC:** AC-2
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given config_path is the default ~/.gurdo/config.toml
When Config::secrets_path(&config_path) is called
Then the result ends with ".gurdo/secrets.toml"

And given config_path is an arbitrary override /tmp/custom/my.toml
When Config::secrets_path(&config_path) is called
Then the result still ends with ".gurdo/secrets.toml"
And the result does not contain "/tmp/custom/"
```

**Notes:** Two distinct inputs required. The old sibling-file test (`/some/dir/secrets.toml`) is replaced by this invariant.

---

### TS-04 — `Config::load` still overlays secrets from `~/.gurdo/secrets.toml`

**Covers AC:** AC-2 (regression)
**Level:** Component · **Type:** File-batch · **Owned by:** Development

```
Given a valid config.toml exists at a temp path
And ~/.gurdo/secrets.toml contains api_key = "k", username = "u", client_id = "c"
When Config::load(&cfg_temp_path) is called
Then config.lastfm.api_key equals "k"
And config.lastfm.username equals "u"
And config.spotify.client_id equals "c"
```

**Notes:** If `~/.gurdo/` is unwritable in CI, mark `#[ignore]` with an explanatory comment. The previous `load_overlays_secrets_when_present` test used a sibling path; this replaces it.

---

### TS-05 — `create_dir_all` creates `~/.gurdo/` when absent; idempotent when present

**Covers AC:** AC-3
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given a temp path simulating ~/.gurdo/ that does not yet exist
When create_dir_all is called with that path
Then the directory is created on disk
And calling create_dir_all again with the same path does not return an error
```

**Notes:** Use `tempfile::tempdir` to avoid touching the real home.

---

### TS-06 — Missing home dir causes early exit with human-readable error

**Covers AC:** AC-3 (edge case)
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given dirs::home_dir() returns None (injectable via wrapper)
When the home-dir resolution function is called
Then an Err is returned containing the substring "Cannot determine home directory"
And execution does not reach create_dir_all
```

**Notes:** Factor the home-dir resolution into a testable wrapper to allow None injection.

---

### TS-07 — `migrate_secrets_if_needed` copies source to dest when only source exists

**Covers AC:** AC-5
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given a temp dir (simulating CWD) contains secrets.toml with content [lastfm]\napi_key = "abc"
And a temp dir (simulating ~/.gurdo/) does not contain secrets.toml
When migrate_secrets_if_needed(home_dir, cwd) is called
Then ~/.gurdo/secrets.toml is created with the same content as the source
And a tracing info message is emitted (not containing the key value "abc")
And the function returns Ok(())
```

**Notes:** Function must accept injectable home_dir and cwd paths for hermeticity.

---

### TS-08 — `migrate_secrets_if_needed` is a no-op when neither file exists

**Covers AC:** AC-7
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given neither ./secrets.toml (CWD) nor ~/.gurdo/secrets.toml exists
When migrate_secrets_if_needed(home_dir, cwd) is called
Then no file is created at ~/.gurdo/secrets.toml
And the function returns Ok(())
```

---

### TS-09 — `migrate_secrets_if_needed` is a no-op when dest already exists

**Covers AC:** AC-6
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given ~/.gurdo/secrets.toml exists with content api_key = "existing"
And ./secrets.toml exists with content api_key = "old"
When migrate_secrets_if_needed(home_dir, cwd) is called
Then ~/.gurdo/secrets.toml still contains api_key = "existing"
And the function returns Ok(())
```

---

### TS-10 — `migrate_secrets_if_needed` is a no-op when only dest exists

**Covers AC:** AC-6 (complementary edge)
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given ~/.gurdo/secrets.toml exists
And ./secrets.toml does not exist in CWD
When migrate_secrets_if_needed(home_dir, cwd) is called
Then ~/.gurdo/secrets.toml is unchanged
And the function returns Ok(())
```

---

### TS-11 — `needs_setup()` returns true when secrets file is absent

**Covers AC:** AC-8
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given the injected secrets_path does not exist on disk
When needs_setup(secrets_path) is called
Then the return value is true
```

---

### TS-12 — `needs_setup()` returns false when all three keys are non-empty

**Covers AC:** AC-9
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given a temp secrets.toml contains:
  [lastfm]
  api_key = "mykey"
  username = "myuser"
  [spotify]
  client_id = "myclient"
When needs_setup(secrets_path) is called
Then the return value is false
```

---

### TS-13 — `needs_setup()` returns true when api_key is whitespace-only

**Covers AC:** AC-8
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given api_key = "   ", username = "myuser", client_id = "myclient" in the temp secrets file
When needs_setup(secrets_path) is called
Then the return value is true
```

---

### TS-14 — `needs_setup()` returns true when username is empty

**Covers AC:** AC-8
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given api_key = "mykey", username = "", client_id = "myclient" in the temp secrets file
When needs_setup(secrets_path) is called
Then the return value is true
```

---

### TS-15 — `needs_setup()` returns true when client_id is absent

**Covers AC:** AC-8
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given the temp secrets file omits the client_id key entirely
And api_key = "mykey", username = "myuser"
When needs_setup(secrets_path) is called
Then the return value is true
```

---

### TS-16 — `needs_setup()` returns true when file is unparseable TOML

**Covers AC:** AC-8
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given the temp secrets file contains "NOT VALID TOML {{{"
When needs_setup(secrets_path) is called
Then the return value is true
And no panic occurs
```

---

### TS-20a — Credentials write helper writes trimmed values to secrets.toml

**Covers AC:** AC-14
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given api_key = "  my_api_key  ", username = "  my_user  ", client_id = "  my_client  "
When the secrets-write helper function is called with a temp path
Then the written file contains api_key = "my_api_key"
And username = "my_user"
And client_id = "my_client"
```

**Notes:** Extract the file-write logic from the setup window into a testable function. This tests trimming and serialization without opening a window.

---

### TS-20b — Secrets file receives `0o600` permissions on Unix after write

**Covers AC:** AC-23
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given the secrets-write helper writes to a temp path
When the file is inspected on Unix
Then stat(path).permissions().mode() & 0o777 == 0o600
```

**Notes:** `#[cfg(unix)]` gate. Windows: compile but skip the chmod assertion.

---

### TS-21 — Config.toml default write skips when file already exists

**Covers AC:** AC-15
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given ~/.gurdo/config.toml does not exist
When the config-default-write helper is called with a temp path
Then the file is created with default TOML content

Given the same temp path already contains "custom = true"
When the config-default-write helper is called again
Then the file still contains "custom = true"
And no overwrite occurred
```

**Notes:** Extract the "write if absent" logic for isolated testing.

---

### TS-29 — `dirs` crate listed in Cargo.toml

**Covers AC:** AC-4
**Level:** Unit · **Type:** CLI · **Owned by:** Development

```
Given Cargo.toml is read
When [dependencies] is inspected
Then it contains an entry for "dirs"
```

**Notes:** Verified by `grep` or by `cargo check` exiting 0 (task T01 done-when).

---

### TS-31 — `home_dir()` returning None propagates as human-readable error

**Covers AC:** AC-3 (edge case)
**Level:** Unit · **Type:** File-batch · **Owned by:** Development

```
Given HOME is unset and dirs::home_dir() returns None (injected via wrapper)
When main() enters its setup preamble
Then an Err is returned before any window opens
And the error message contains "Cannot determine home directory"
```

---

### TS-28 — No credential values appear in captured log output

**Covers AC:** AC-24
**Level:** Component · **Type:** File-batch · **Owned by:** Development

```
Given RUST_LOG=trace is set and a tracing subscriber captures output to a buffer
When the secrets-write helper runs with api_key="SENTINEL_KEY", username="SENTINEL_USER", client_id="SENTINEL_CLIENT"
And migrate_secrets_if_needed runs with a source file containing those values
Then the captured log buffer does not contain the string "SENTINEL_KEY"
And does not contain "SENTINEL_USER"
And does not contain "SENTINEL_CLIENT"
And field names such as "api_key" may appear in logs
```

**Notes:** Use `tracing-subscriber` with a `Vec<u8>` writer to capture output in tests.

---

## Verification-owned scenarios (E2E / UI — manual smoke)

> These cannot be automated headlessly. They are executed manually during the Integration phase by running the binary on the host.

---

### TS-17 — Setup window opens with correct title, size, and Phase 1 fields

**Covers AC:** AC-10, AC-12
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given the app is launched with no ~/.gurdo/secrets.toml
When the process starts
Then an eframe window titled "Gurdo — Setup" appears
And the window is 440 × 400 px and not resizable
And three labeled full-width TextEdit fields are visible in order:
  "Last.fm API Key", "Last.fm Username", "Spotify Client ID"
```

---

### TS-18 — Continue button gating on non-empty fields

**Covers AC:** AC-13
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given the setup window is open in Phase 1
When all three fields are empty
Then the Continue button is disabled

When two fields are filled and one is blank
Then the Continue button remains disabled

When all three fields contain at least one non-whitespace character
Then the Continue button becomes enabled
```

---

### TS-19 — Closing setup window before Continue exits with error message

**Covers AC:** AC-11
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given the setup window Phase 1 is open
When the user clicks the OS window close button
Then the process exits with a non-zero code
And the terminal shows: "Setup cancelled — please re-run Gurdo to complete setup."
```

---

### TS-22 — Write failure shows inline error label; retry without restart

**Covers AC:** AC-16
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given ~/.gurdo/ has permissions 0o500 (read + execute, no write)
When the user fills all three fields and clicks Continue
Then no panic occurs
And an inline error label appears containing the OS error (e.g. "Permission denied")
And all three fields remain editable
And after restoring write permissions and clicking Continue again, the files are written and Phase 2 appears
```

---

### TS-23 — Phase 2 layout: status label, Connect button, Skip button

**Covers AC:** AC-17
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given Phase 1 has been completed (credentials entered and Continue clicked)
When the UI transitions to Phase 2
Then Phase 1 text fields are no longer visible
And a status label reads "Connect your Spotify account to enable playback."
And a "Connect Spotify" button is visible
And a "Skip for now" button is visible
```

---

### TS-26 — In-progress OAuth disables buttons and updates label

**Covers AC:** AC-18
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given Phase 2 is displayed and the user clicks "Connect Spotify"
While the browser callback has not yet returned
Then the status label reads "Waiting for Spotify authorisation…"
And both buttons are disabled
```

---

### TS-24 — Successful OAuth closes setup window; player opens normally

**Covers AC:** AC-19, AC-22
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given Phase 2 is displayed
When the user clicks "Connect Spotify" and completes the OAuth flow in the browser
Then the setup window closes
And the main player window opens
And the launch sequence is identical to a returning user (no special post-setup UI)
```

---

### TS-25 — OAuth failure: Retry re-invokes flow; Skip proceeds to player

**Covers AC:** AC-20
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given Phase 2 is displayed and OAuth fails (e.g. the browser callback times out)
When the flow returns an error
Then the status label turns red and shows "OAuth failed: <message>"
And the button label changes to "Retry"
And "Skip for now" is re-enabled

When the user clicks "Retry"
Then the OAuth flow is re-attempted

When the user clicks "Skip for now"
Then the setup window closes and the main player opens
```

---

### TS-27 — Closing setup window during Phase 2 exits with distinct error message

**Covers AC:** AC-21
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given the user is in Phase 2 (OAuth phase)
When the user clicks the OS window close button
Then the process exits with a non-zero code
And the terminal shows: "Setup cancelled during OAuth — Spotify not connected."
```

---

### TS-30 — Returning user bypasses setup; player opens directly

**Covers AC:** AC-8, AC-9 (integration regression)
**Level:** E2E · **Type:** UI · **Owned by:** Verification (manual)

```
Given ~/.gurdo/secrets.toml exists with non-empty api_key, username, and client_id
When the app is launched
Then no setup window appears
And the main player window opens immediately
```

---

## Coverage summary

| AC | Scenario(s) | Level | Phase |
|---|---|---|---|
| AC-1 | TS-01, TS-02 | Unit | Development |
| AC-2 | TS-03, TS-04 | Unit, Component | Development |
| AC-3 | TS-05, TS-06, TS-31 | Unit | Development |
| AC-4 | TS-29 | Unit | Development |
| AC-5 | TS-07 | Unit | Development |
| AC-6 | TS-09, TS-10 | Unit | Development |
| AC-7 | TS-08 | Unit | Development |
| AC-8 | TS-11, TS-13, TS-14, TS-15, TS-16 | Unit | Development |
| AC-9 | TS-12 | Unit | Development |
| AC-10 | TS-17 | E2E/UI | Verification (manual) |
| AC-11 | TS-19 | E2E/UI | Verification (manual) |
| AC-12 | TS-17 | E2E/UI | Verification (manual) |
| AC-13 | TS-18 | E2E/UI | Verification (manual) |
| AC-14 | TS-20a | Unit | Development |
| AC-15 | TS-21 | Unit | Development |
| AC-16 | TS-22 | E2E/UI | Verification (manual) |
| AC-17 | TS-23 | E2E/UI | Verification (manual) |
| AC-18 | TS-26 | E2E/UI | Verification (manual) |
| AC-19 | TS-24 | E2E/UI | Verification (manual) |
| AC-20 | TS-25 | E2E/UI | Verification (manual) |
| AC-21 | TS-27 | E2E/UI | Verification (manual) |
| AC-22 | TS-24, TS-30 | E2E/UI | Verification (manual) |
| AC-23 | TS-20b | Unit | Development |
| AC-24 | TS-28 | Component | Development |
