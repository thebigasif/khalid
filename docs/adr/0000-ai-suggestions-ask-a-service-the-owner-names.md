# 0000. AI suggestions ask a service the owner names, through the Admin API

**Number:** next free. **Status:** Proposed, 2026-11-02. Yoast package WP4.

## Context

The founder asked for every Yoast SEO feature that applies to Khalid. Yoast's AI features generate SEO titles and meta descriptions, social titles and descriptions, alt text, fixes for failed checks and a summary of key takeaways. Plan section 1 keeps Khalid neutral: "the open-source build never calls SerpDad or any AI", and "Connect to SerpDad" stays off until the owner turns it on. The founder said yes to this package on 2026-11-02, so the question is how to add it without breaking that promise.

## Decision

- **Off by default, one table to turn it on.** `[ai]` in `site.toml` names a `provider`: `off`, the default, `serpdad`, `anthropic` or `openai`. Without it no request leaves the server and the editor shows no button. The setting is the typed `khalid_core::AiSettings`, which the SEO settings screen of WP3 can edit.
- **The key only in the environment.** `KHALID_AI_KEY`, read once with the other secrets into `Secret<String>`. `[ai]` refuses a `key` field, so it cannot land in a file.
- **SerpDad needs its connection on.** `provider = "serpdad"` asks only while the Connect to SerpDad page has a key, so turning SerpDad off on that page also stops the suggestions. The request is a plain JSON contract Khalid defines (`task`, `prompt`, `count`, answered with `suggestions`), so SerpDad can serve it without knowing Khalid's internals.
- **One route, `/api/admin/v1/ai/`.** `GET` says whether suggestions are on, which service writes them and whether the key is set. `POST /api/admin/v1/ai/suggestions/` takes a `task` and either a post's identity or the words themselves, and answers up to five options, each with its length checked against the same limits the linter uses. The admin's buttons, the bulk SEO editor and any API client call the same route (the API is the only door).
- **Requests go through `Outbound`,** with its timeouts, size cap and private-network guard, like every other request Khalid sends.
- **Nothing changes until a writer picks.** A suggestion is an answer, not an edit. The writer chooses one and the editor saves it the usual way.
- **Tests use a fake provider.** No test in CI calls a real AI service; the HTTP clients are tested against `wiremock`.

## Consequences

- Plan section 1's "no calls home" stays true for every site that does not add `[ai]`.
- A site that turns it on sends post text to the service it chose. `docs/deploy.md` says exactly what each button sends.
- A self-hosted model on the same machine is refused by the private-network guard. Such a model can be put behind a public address, or added later with an explicit setting.
