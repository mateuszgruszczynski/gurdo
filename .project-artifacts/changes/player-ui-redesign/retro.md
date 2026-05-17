# Retrospective — Player UI polish: consistent ghost-style controls

## What went well

- Setting ghost visuals once on the parent `ui` inside `vertical_centered` cleanly propagated to
  all three child rows — no per-button `.fill()` boilerplate needed.
- Splitting the action row into feedback (Like/Dislike) + utility (Queue/Settings) required only
  layout arithmetic changes; no logic moved.
- Progress bar colour swap (dark → white-tinted) was a two-line change with clear before/after.

## What was harder than expected

Nothing significant. The change was well-scoped and the egui visuals API did exactly what was
needed.

## Follow-up items

None. The pre-existing `last_track_uri` unused-assignment warning is unrelated to this change
and should be addressed in a future dead-code pass if it bothers the build output.
