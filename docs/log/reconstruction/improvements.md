# Improvement Ideas

Back: [README.md](README.md)

## Architecture

- Consider a plugin-like trait interface for services even without external plugins
- Evaluate whether snapshot cloning should use `Arc<Rope>` to reduce overhead
- Consider arena allocation for frequently created small types
- Visual mode selection tracking (anchor + cursor range) for visual operations
- Replace mode overwrite semantics (replace existing character instead of insert)
- Explorer file operations (create/rename/delete) via shell commands

## Documentation

- Add sequence diagrams for key dispatch pipeline
- Add sequence diagrams for service request/response lifecycle
- Consider adding ADR (Architecture Decision Record) format for design decisions
- Document the ops/ and paint/ subdirectory reorganization rationale

## Testing

- Consider property-based testing for cursor arithmetic
- Consider fuzzing for the VT parser
- Consider snapshot testing for render output
- Add integration tests that exercise full key→action→state→render pipeline
- Consider randomized stress testing for concurrent terminal + editing

## Performance

- Evaluate lazy line-width caching strategies
- Consider line-level dirty tracking to avoid full-buffer grapheme iteration
- Evaluate incremental syntax highlighting approaches
- Profile fuzzy matching with large file lists (>100k entries)
