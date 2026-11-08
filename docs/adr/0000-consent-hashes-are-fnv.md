# Consent hashes are FNV-1a over length-prefixed fields

**Number:** next free.

**Status:** Proposed, 2026-11-08. Privacy plan U2, KTD3, KTD8.

## Context

The privacy plan (`docs/plans/2026-11-08-0314-feat-privacy-consent-plan.md`, KTD3 and KTD8) needs two hashes of the consent settings. The rev goes into every reader's `khalid-consent` cookie, and a change asks every reader again. The settings version keys the consent proof tables, kept for as long as the site exists. Both must give the same value for the same settings on every build. The standard library's hasher may change between Rust releases, which would ask every reader again after an upgrade and split the proof history.

## Decision

`khalid-core` hashes with FNV-1a over 64 bits, written out in `privacy_settings.rs` with no new dependency. Each field goes in with its length first, so two lists of fields never run together into the same bytes. The rev is the top 32 bits as eight hex digits; the settings version is all 64 bits as sixteen hex digits. A test pins both values for the default settings.

The rev covers the lifetime, each optional category that holds code with its sorted vendor names, and External media with the list of embed providers the caller passes, which is always the full fixed list, so publishing a post never changes it. The settings version adds the owner's text per language, the vendors' policy links, the GPC mode, the outside tool and the release's own banner words, which the caller passes in.

`counts` has no fixed default: unwritten, it is on in server mode and off in static mode, and writing `counts = true` on a static site is refused. This keeps every existing static site starting.

## Consequences

- No cryptographic strength is needed: the cookie is unsigned by design (KTD3) and the hashes only detect change.
- A 32-bit rev may collide by chance about once in four billion changes, which would skip one re-ask. That is accepted.
- Changing the field order or the hash breaks every cookie and proof key, so the pinned test must fail first.
