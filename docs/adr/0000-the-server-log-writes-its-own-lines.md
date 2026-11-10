# 0000. The server log writes its own lines, without tracing-subscriber

**Number:** next free. **Status:** Proposed, 2026-11-10. Gap briefs G38 and G40, KH-OPS-2.

## Context

`khalid serve` logs through `tracing` (G38). The first version installed `tracing-subscriber`'s `fmt` writer, with its `json` feature for `[log] format = "json"`. `docs/standards/rust.md` named `tracing-subscriber` for logging. With it, the binary was 133,720 bytes over the 30 MB budget on this server, and about 159,000 in CI (PR 1161, "Binary size" job). `tracing-subscriber` brings `sharded-slab`, `thread_local` and `tracing-serde`, and it builds its writer twice, once for plain lines and once for JSON.

## Decision

- **`server_log.rs` has its own `tracing::Subscriber`, `Lines`**, about 110 lines. It keeps events at `[log] level` and above and writes each as one line to standard error: `timestamp LEVEL message name=value`, or one JSON object with `timestamp`, `level` and `fields`, the shape `tracing-subscriber` wrote. Numbers and true or false stay numbers and booleans in JSON.
- **Spans are not kept.** Khalid opens none. A span from a dependency gets an ID and adds nothing to a line.
- **The time comes from `SystemClock`** (BL-RUN-1), in UTC with microseconds.
- `docs/standards/rust.md` now says Khalid uses `tracing` and writes the serve log itself.

## Consequences

- The binary is 66,432 bytes smaller, and four dependencies leave the tree.
- That alone does not fit the budget: it is still 67,288 bytes over here. The SQLite options in `.cargo/config.toml`, handed to the CTO as a patch, close the rest, with about 232,000 bytes to spare here.
- A future need for spans, per-module filters or another output (such as journald fields) means adding to `Lines` or bringing `tracing-subscriber` back and finding the room again (`docs/standards/performance.md`, "Room for the server log").
