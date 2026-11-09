---
title: "Charts, embedded tools and tracking tags"
order: 90
---

# Charts, embedded tools and tracking tags

Three ways to make a post do more: draw numbers as a chart, show an interactive tool from a site you trust, and see which posts send visitors to your product.

## Charts

A chart turns a few lines of numbers into a picture when the blog is built. It needs no script, fits a phone and a wide screen, takes its colours from your template, and carries a table of the same numbers for screen readers.

```markdown
:::chart{type="bar" title="Visits from search by page in October 2026, in thousands" unit="k"}
Page, Visits
Pricing guide, 4.2
Setup checklist, 2.9
Home page, 1.1
:::
```

- `type` is `bar` (across, good for long labels), `column` (upwards, the default) or `line` (a change over time).
- `title` is required. It is the chart's caption: say what the numbers are and when they held, such as "in October 2026".
- `unit` is optional and shows beside each number, such as `%`, `k` or `visits`.
- The first line names the labels' column, then one to four series. Each line after it is a label and one number per series, split by commas. Write numbers plainly, such as `12` or `-3.5`, with no unit or thousands commas. Put a label with a comma in double quotes.
- A chart holds up to 40 rows.

To keep the numbers in a file, put a CSV file with the same lines next to the post, in the post's folder, and name it with `src`:

```markdown
:::chart{type="line" title="Visits from search in 2026, in thousands" unit="k" src="visits.csv"}
:::
```

The build reads the file each time, so updating the file updates the chart. The file is published beside the post, and the chart's caption links to it. The linter reads the chart's caption, not its numbers, so name the year or the source there.

## Interactive tools from sites you allow

`::embed` shows video and sound players from YouTube, Vimeo, Spotify, SoundCloud, CodePen and Loom. To embed another site's page, such as a calculator or a live chart, list its host in `site.toml`:

```toml
[media]
embed_domains = ["tools.example.com"]
```

Then embed any `https` page on that host:

```markdown
::embed{src="https://tools.example.com/roi" title="Content ROI calculator" height="560"}
```

The page loads in a frame only when a reader scrolls to it, keeps its height so nothing jumps (`height` in pixels, 120 to 2000, 480 by default), runs sandboxed, and has a link to the page under it. `title` is what a screen reader says for the frame. Each listed host is exact: `tools.example.com` does not allow `example.com`. Only listed hosts join the blog's Content Security Policy, so a browser refuses any other frame. An address on any other host shows as a plain link.

## Tracking tags on links to your product

To see in your product's analytics which posts send visitors, list the product's domains:

```toml
[seo.links.tracking]
domains = ["app.example.com"]
source = "blog"        # optional, the default
medium = "content"     # optional, the default
campaign = "spring"    # optional
```

Every link to one of those domains or its subdomains, in a post, a page, a button, the `::cta` block, the call-to-action band and the header button, gets `utm_source`, `utm_medium` and `utm_campaign`. Without `campaign`, a link in a post is named after the post's slug, a link in another page after `blog`, and the call to action after `cta`. No other link ever gets a tag, and a link that already has `utm_source` keeps its own. Values are letters, digits, hyphens, underscores and dots.
