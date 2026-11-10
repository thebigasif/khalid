# Menus

Every page can show two menus: the main menu in the header and a short menu in the footer. Both are empty out of the box, and an empty menu shows nothing.

## Set the menus

Write them in `site.toml`, one table per link, in the order readers see them:

```toml
[[nav.primary]]
label = "Guides"
href = "/blog/guides/"

[[nav.footer]]
label = "Impressum"
href = "/blog/de/impressum/"
language = "de"
```

You can also change them in the admin, on the Navigation screen under Site. It saves the same tables into `site.toml`, and `khalid serve` shows the new menu on the next page it sends, with no restart.

| Field | What it holds |
| --- | --- |
| `label` | The words a reader sees. One line, 160 characters at most. |
| `href` | Where the link goes: a path on this site that starts with `/`, such as `/blog/about/`, or a full address that starts with `http://` or `https://`. |
| `language` | Optional. The one language whose pages show the link, such as `de`. Without it, every page shows the link. |

## What to expect

- A link that is neither a path on this site nor a full address is refused, and nothing is saved. That keeps `javascript:` and `mailto:` links out of the menu.
- Columns of footer links under a heading, `[[nav.footer_groups]]`, are set in `site.toml` only. Saving the menus in the admin keeps them as written.
- Menus are plain links in the page, so they work with scripts off.

## Through the API

The Admin API reads the menus as `nav` in `GET /api/admin/v1/settings/`, and changes them with `PUT /api/admin/v1/settings/`:

```json
{
  "settings": {
    "updated_at": "2026-11-10T09:00:00Z",
    "nav": { "primary": [{ "label": "Guides", "href": "/blog/guides/" }] }
  }
}
```

Each menu you send replaces that menu whole. A menu you leave out stays as it is, and an empty list removes the menu. The key needs `settings:write`.
