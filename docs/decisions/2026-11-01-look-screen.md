# The Look screen, 2026-11-01

The founder said yes in advance, on 2026-10-27, to every admin mockup, and again to everything in this task. This records that yes for the Look screen (templates spec item 8 of section 4.6), so the screen can be built without stopping for a review.

## What was agreed

- The mockup is `docs/design/admin-mockups/look.html`, beside the other admin mockups.
- The screen is "Look" in the side menu, before Settings. One focus point: which template the blog uses.
- It lists every template the running Khalid has, read from the engine through `GET /api/admin/v1/templates/`. No name is written into the admin, so a template another builder adds shows up with no change to the screen.
- Each template shows its one-sentence summary and what it turns on and off: the reader features it switches, and its options that start on.
- Under the list sit the chosen template's own options, each with the engine's one sentence of help and the template's default already filled in. A site picks one template and sets that template's options. Templates are not mixed (CTO decision, 2026-11-01).
- Saving goes through the existing settings route, `PUT /api/admin/v1/settings/`, with the `updated_at` of the copy it was made on. Only options that differ from the template's default are written to `site.toml`, so the file stays short.
- The running server reads `site.toml` at start, so the screen says when a restart applies the change, as the Settings screen does.
- Both routes need `settings:write`, as the Settings screen does.

## Left for later

- A preview of the demo site in each template with your own colours (spec 4.1). It needs `khalid template preview` first.
- Editing `[template.cta]`, the call to action, on this screen. Its label and address are words and a link, not a look, so they belong beside the other settings.
- Showing which features your own `[reader]` table already sets, since those win over the template.
