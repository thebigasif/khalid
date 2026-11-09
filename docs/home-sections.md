# Home page sections

Out of the box the home page is one list of your newest posts. Home sections let you choose what the home page shows instead, and in which order. Each section is a heading, a few posts and a "View all" link to the page with all of them. Every template shows them in its own style.

## Choose the sections

List them under `[reader]` in `site.toml`, top of the page first:

```toml
[reader]
home_sections = [
  "featured",
  { category = "technical-seo" },
  "newsletter",
  { series = "link-building-101" },
  "latest",
]
```

You can also pick and order them in the admin, under Site settings, in the "Home page" card.

| Section | What it shows | "View all" goes to |
| --- | --- | --- |
| `"latest"` | Your newest posts | The archive |
| `"featured"` | Posts with `featured: true` in their front matter, newest first | A channel of featured posts if `routes.toml` has one, otherwise the archive |
| `"newsletter"` | The sign-up form for your newsletter | No link |
| `{ category = "slug" }` | The newest posts of a category from `categories.toml` | The category's page |
| `{ collection = "name" }` | The newest posts of a collection, such as `guides` | The collection's page |
| `{ channel = "name" }` | The newest posts of a channel from `routes.toml` | The channel's page |
| `{ series = "slug" }` | The parts of a series, in reading order. The slug is the end of the series page's address, such as `link-building-101` for `/series/link-building-101/` | The series page |

Each section shows up to six posts. Some templates show fewer.

## What to expect

- A section with nothing to show is left out. A category with no posts in German yet does not show on the German home page, and the newsletter section shows only on a site that sends mail.
- The newsletter form shows once. When you place it as a section, the template's usual place for it on the home page stays empty. Posts keep their form as the template sets it.
- Dispatch and Versions always show their own sign-up form, at the top and beside the changes, so a newsletter section adds nothing there. Manual shows the form after its grid of topic tiles.
- Without `home_sections`, the home page stays exactly as it was.
- With home sections, the archive page is no longer a copy of the home page, so search engines may list it.

The older form with one table per section, `[[reader.home_sections]]` with `category = "slug"`, still works for categories, collections and channels.
