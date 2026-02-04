# Network Filesystems (Guidance)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Guidance for safe file I/O when the underlying filesystem is remote (NFS/SMB/SSHFS/etc.).

Status note: kjxlkj does not need special “filesystem plugins” to operate on network mounts. However, network filesystems amplify failure modes; this document defines the expectations a robust implementation should meet.

## Core risks

Network filesystems commonly introduce:

- higher latency and variable throughput
- transient failures (disconnects, timeouts)
- weaker “atomicity” guarantees depending on mount options
- clock skew affecting mtime-based change detection

## Goals (normative)

- Never silently lose user edits.
- Never corrupt an on-disk file due to partial writes.
- Make external modification conflicts explicit (when detectable).
- Keep the editor responsive even when the filesystem is slow.

## Save strategy (target)

Recommended posture for writes (best-effort atomic save):

1. Write to a temporary file in the same directory as the target path.
2. Flush and sync the temporary file (best-effort; platform-dependent).
3. Replace the destination via an atomic rename when supported.
4. On failure, keep the buffer “dirty” and surface an actionable error to the user.

If a platform/filesystem cannot provide atomic replace, the limitation MUST be documented.

## External modification detection (target)

When writing a file that may have been modified externally, the editor SHOULD detect and report conflicts.

Typical signals (all imperfect on network mounts):

- file size
- modified time
- inode/file ID (when available)
- content hash (expensive; consider only on explicit user action)

If change detection is unreliable, the implementation should still avoid overwriting silently: require explicit confirmation before destructive writes.

## Auto-save posture (target)

Auto-save is risky on flaky mounts.

Recommended default:

- do not auto-save by default
- if auto-save exists, make it explicit and configurable per project

## Error handling (normative)

When a filesystem operation fails (read, write, rename, sync):

- the editor MUST not panic
- the buffer MUST remain intact in memory
- the user MUST receive a clear error message
- the editor SHOULD provide a “write to alternate path” escape hatch

Related: [/docs/technical/error-recovery.md](/docs/technical/error-recovery.md)

## Performance posture (target)

- avoid synchronous “scan the whole directory tree” behaviors
- avoid repeated stat calls on every keystroke
- keep the UI responsive: IO should not stall rendering/input indefinitely
