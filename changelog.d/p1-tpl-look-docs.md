Added
- A Look screen in the admin: pick one of the built-in templates, see what each one turns on and off, and set that template's options. It saves to `site.toml`, and your blog shows the new look after Khalid restarts. See `docs/templates.md`.
- `GET /api/admin/v1/templates/` lists every built-in template with its options, and the settings route reads and saves the site's `template`.
