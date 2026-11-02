# Reading aids

Four small aids help a reader find their way through a long post. Every template has them, classic too. Each one works without scripts where it can; the reader script, a small file that loads after the page, only adds to it. Nothing here needs a setting to start.

## The reading progress bar

A thin line in your accent colour runs along the top of the window and fills as the reader scrolls a post. Current browsers draw it from the scroll alone, with no script. Older browsers fill it from the reader script. It hides for readers who ask their device for less motion, and when a page is printed.

To turn it off, set the template option `reading_progress = "none"` under `[template.options]`. Classic takes no options, so classic always shows it.

## The contents that follow you

On a wide screen the contents column beside a long post stays in view as the reader scrolls, and marks the section they are reading. On a phone, or in a template that puts the contents at the top, a small "Contents" button floats in the corner. It opens the contents as a sheet from the bottom of the screen, with the section being read marked. Escape or the Close button shuts it, and focus stays inside it while it is open. Picking a section closes the sheet and goes there.

With scripts off, the contents box at the top of the post still lists every section.

The contents themselves are on with the `table_of_contents` reader feature and placed by the template's `contents` option.

## Copy link on every heading

Each H2 and H3 of a post ends in a `#` button. Pointing at it says "Copy link". Pressing it copies the address of that section, such as `https://example.com/blog/content-refresh/#why-old-posts-slip`, and says "Link copied", out loud for screen readers too.

With scripts off, or on a page the browser will not let copy (one served over plain `http` to another computer), the `#` stays a plain link to the section, which a reader can copy from the address bar.

Turn it off with `heading_links = false` under `[reader]` in `site.toml`.

## Search in the header

On a blog that answers searches (`mode = "server"`), the header shows a search link with a magnifier. Pressing it, or `/`, or Ctrl+K (Cmd+K on a Mac), opens a search box over the page that shows matching posts as the reader types, from the same search the search page uses. Escape or Close shuts it. With scripts off, the link opens the search page.

The template option `header_search` picks what the header shows:

- `button`, the default: the search box over the page.
- `link`: always open the search page.
- `none`: no search in the header.

A static blog is plain files with no server to answer a search, so its header shows no search. Turn the header search off on any blog with `search_shortcut = false` under `[reader]`.
