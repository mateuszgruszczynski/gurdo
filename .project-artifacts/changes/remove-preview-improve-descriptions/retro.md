# Retrospective — Remove recommendation preview + improve settings descriptions

## What went well

- Clean removal: Preview touched exactly 3 files, all references gone in one pass.
- Removing the Preview dispatch arm made `settings_draft` unused in `ops_dispatcher_loop` — caught immediately by the compiler, easy fix.
- Delegating the description copy to a subagent produced noticeably better plain-English phrasing in one pass.

## What was harder than expected

Nothing significant. Straightforward change.

## Follow-up items

None.
