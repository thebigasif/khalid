# 0000. The Linux binary packs its relocations

**Number:** next free. **Status:** Proposed, 2026-11-20. Task kw-binroom, KH-OPS-4, BL-PERF-1.

## Context

After PR 1251 the release binary built 31,432,248 bytes in CI, 25,032 under its 30 MB budget, and PRs 1256 and 1257 failed the size check. Every lever in the release profile had been measured before (`docs/standards/performance.md`, "Binary size"): opt-level per crate is used up, the API router is ordinary handler code, and SQLite's unused parts need a `.cargo/config.toml` that is not in the repository.

The binary is position-independent, so the loader can place it at a random address (ASLR). For every pointer stored in the binary, the file keeps a 24-byte relocation entry saying where it is, about 1.4 MB in all. Building without position independence was rejected on 2026-10-31 because it gives up ASLR.

## Decision

`crates/khalid/build.rs` passes `-z pack-relative-relocs` to the linker for the `khalid` binary on Linux. The linker then stores those relocations as `DT_RELR`: one address, then a bitmap word for the next 63 pointers. The binary stays position-independent and still loads at a random address. Test binaries and other platforms are unchanged.

## Consequences

- The binary is 1,490,872 bytes smaller (31,424,728 to 29,933,856 on main here), about 1.5 MB of room under the budget. No feature, dependency or optimisation level changes.
- A full build of the 10,000-post site writes the same files, byte for byte, and takes no longer (17.2 and 17.7 s against 19.4 and 20.0 s, back to back). Fewer relocations to apply at start can only make start-up faster.
- The binary needs a C library that reads `DT_RELR`: glibc 2.36 or newer, or musl 1.2.4 or newer. The release binaries and the Docker image are static musl builds whose start-up code comes from Rust's own musl or Alpine's. A static musl test program built here with the option ran and applied its packed relocations, and the release workflow still starts each release binary before it is published. A binary built from source on Linux with an older glibc (Ubuntu 22.04, Debian 11, RHEL 9) refuses to start with "version `GLIBC_ABI_DT_RELR' not found"; `CONTRIBUTING.md` says to use the release binary there.
- The linker must know the option: GNU ld 2.38 or newer, or LLVM's lld, which Rust uses on Linux by default.
