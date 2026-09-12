# Phase 1: File Integrity Monitoring (FIM) Core — Decisions & Plans

**Project:** Host-based Intrusion Detection System (HIDS) in Rust **Phase:** 1 of 5 — File Integrity Monitoring core 
**Status:** Design complete, implementation in progress

---

## 1. Scope & Exclusions

- Hash **every file** under a target directory tree by default.
- Support **include/exclude patterns** so specific files/directories can be skipped from tracking (e.g. noisy log dirs, `.git`, huge binaries not worth monitoring).
- Exclusion pattern design is deferred — noted as an open item to revisit once core scanning works.

## 2. Large File Handling

Two distinct problems were identified and solved separately:

- **Performance/memory:** Files are hashed via **streamed reading** (buffered chunks fed incrementally into the SHA-256 hasher), never loaded fully into memory.
- **TOCTOU (time-of-check-to-time-of-use) race:** A file can change while it's being hashed. Decision: **flag as `Unstable`** if size or mtime shifts between the start and end of the hashing read, rather than silently trusting a possibly-inconsistent hash.
    - This is treated as an accepted v1 limitation, not a security guarantee — a determined attacker who resets mtime (`touch -r`) after modifying a file could still evade detection. Acceptable tradeoff for now; revisit if needed.

## 3. Baseline Storage Shape

- **Flat `HashMap<PathBuf, FileRecord>`** — not a nested tree mirroring the filesystem structure.
- Rationale: diffing two flat maps is simple (key-by-key comparison); a nested tree only pays off for per-directory operations or visualization, which aren't v1 needs.

## 4. Data Model

### `FileRecord`

Represents a **snapshot of one file at one point in time**. Fields include:

- Path metadata (size, mtime, permissions, owner)
- `hash: Option<...>` or similar — hash result
- `status: HashStatus` — replaces a simple boolean

### `HashStatus` (enum, not a bool)

Distinguishes _why_ a file's hash may not be straightforwardly trustworthy:

```
enum HashStatus {
    Verified,
    Unstable,          // size/mtime shifted during read
    PermissionDenied,
    Error(String),      // catch-all IO error, message for debugging
}
```

Rationale: a boolean `is_stable` collapses meaningfully different signals (mid-write vs. access denied vs. IO error) into one bit, losing information that matters for security response — e.g., a file that suddenly becomes unreadable is itself a signal worth alerting on.

### `ChangeType` (enum) — granular, not a single verdict

Produced only when **comparing two `FileRecord`s** (old vs. new), never stored inside a single `FileRecord`. Key conceptual clarification reached during design:

> `FileRecord` = a fact about **one moment**. `ChangeType` = a relationship between **two facts** (two snapshots).

Putting `ChangeType` inside `FileRecord` was identified as a conceptual mistake — it would conflate two different temporal moments and leave no sensible value for a first-ever baseline scan (nothing to compare against yet).

Example variants:

```
enum ChangeType {
    New,
    Deleted,
    HashChanged,
    SizeChanged { old: u64, new: u64 },
    PermissionsChanged { old: u32, new: u32 },
    OwnerChanged { old: (u32, u32), new: (u32, u32) },
    AccessLost,   // HashStatus transitioned e.g. Verified -> PermissionDenied
}
```

Granular change types were chosen over a single `Changed` verdict because different change types warrant different responses (e.g. a permission change alone may signal privilege escalation attempts, distinct from a content/hash change).

### `DiffResult`

The **output of comparing two baselines** (old snapshot vs. new snapshot), e.g.:

```
struct DiffResult {
    path: PathBuf,
    changes: Vec<ChangeType>,
}
```

Produced by a `diff(old: &Baseline, new: &Baseline) -> Vec<DiffResult>` function — not stored as part of either `FileRecord`.

## 5. Symlink Handling

- **Symlinks are never followed** (no dereferencing into target content).
- Each symlink is tracked as **its own entity**: its own metadata (`lstat`, not `stat`) plus the target path string it points to.
- A change to the target string itself is a trackable event (e.g. `HashChanged`-equivalent on the symlink record), since redirecting a symlink to a malicious target is a real attack vector.
- This choice also sidesteps infinite loops and cross-filesystem/mount traversal issues (e.g. wandering into `/proc`), since the scanner never descends through a symlink.
- Note: `walkdir`'s `.follow_links()` defaults to `false`, aligning with this decision — but symlink entries still need to be explicitly detected and routed to "record symlink metadata" instead of being hashed as regular files.

## 6. Persistence Format

- **Flat file, JSON** (via `serde`/`serde_json`) for Phase 1.
- Rationale: human-readable (can `cat`/`diff` the file while debugging), trivial to implement with `serde`'s derive macros, no schema/migration overhead, and entirely sufficient at the scale of a single host's monitored directories (thousands of records, not millions).
- **Explicitly deferred:** a database (e.g. SQLite) is planned for **Phase 2**, once there's an actual UI to serve and a real need for querying, partial updates, or historical tracking. Introducing a database now would solve a problem that doesn't yet exist.
- `bincode` (compact binary format) noted as a possible future swap purely for performance, once logic is stable — `serde` makes this a near-free change later since both formats plug into the same `Serialize`/`Deserialize` traits.

## 7. CLI / Binary Shape

- **Single binary, two subcommands**: `hids baseline <path>` and `hids scan <path>`.
- Both subcommands share a common core function: `scan_directory(path) -> HashMap<PathBuf, FileRecord>`, which has no knowledge of "baseline" vs. "scan" — it just produces a snapshot.
- `baseline` subcommand: calls `scan_directory()`, serializes the result to disk. One-way write, nothing to diff.
- `scan` subcommand: calls `scan_directory()` for a fresh snapshot, loads the previously stored baseline from disk, and calls `diff(old, new) -> Vec<DiffResult>` to report findings.

## 8. Error Handling

- **`anyhow::Result` throughout for now** — low friction while the core logic is still being discovered/written.
- Plan to **formalize into a typed `HidsError` enum later**, once it's clear which failure modes matter for security/alerting logic (mirroring the same reasoning that produced `HashStatus` as a typed enum rather than a boolean).

## 9. Project Structure

```
src/
  main.rs       // clap parsing + dispatch — thin, ~50-100 lines
  lib.rs        // module re-exports
  record.rs     // FileRecord, HashStatus
  hasher.rs     // streamed SHA-256, symlink-aware hashing logic
  scanner.rs    // scan_directory() — walks tree, builds FileRecords
  baseline.rs   // save_baseline(), load_baseline() — JSON I/O
  diff.rs       // ChangeType, DiffResult, diff()
  cli.rs        // clap Args/Subcommand definitions
```

Module boundary rationale:

- `hasher.rs` is separate from `scanner.rs` so hashing algorithm improvements have one obvious home.
- `diff.rs` holds `ChangeType`/`DiffResult` separately from `record.rs` (which holds `FileRecord`/`HashStatus`), physically reinforcing the snapshot-vs-comparison conceptual split.

## 10. Explicitly Deferred (Not Phase 1)

- Exclude/include pattern syntax and matching logic.
- Any UI or presentation layer for baseline/scan results — core logic comes first.
- Database-backed persistence (planned trigger point: Phase 2, when a UI needs to query results).
- Typed `HidsError` enum (currently using `anyhow`).

---