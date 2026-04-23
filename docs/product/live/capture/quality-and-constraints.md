# Live Capture Quality and Constraints

## Quality Presets

- Quality is expressed as target video height plus target frame rate.
- Height presets are `360`, `480`, `720`, `1080`, `1440`, and `2160`.
- Frame-rate presets are `15`, `30`, `45`, `60`, and `120`.
- The default quality is `1080p` at `60 fps`.
- Capture constraints are browser best-effort.
- If a new runtime constraint fails, the active stream remains unchanged when possible.

## Constraint Behavior

- Constraints are passed as `ideal` values, not `exact`.
- The browser may select a different resolution or frame rate than requested.
- Screen capture constraints apply to `getDisplayMedia`.
- Camera capture constraints apply to `getUserMedia` and may include an exact `deviceId`.
