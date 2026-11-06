---
title: "Taste"
order: 85
---

# Taste

Taste is a template for a baking blog: bread, cakes, pastry and the bakes people make at home. The front page opens with a large, centred line between two thin rules and a sign-up field under it. A magazine block follows, with the newest bake large beside its picture and the next four in a row. Then each main topic, such as Bread or Cakes, gets its own picture grid, three across and then four across. Each post is centred, with its picture as wide as the page and a sign-up box after it.

Based on Ghost's Taste theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "taste"
```

## Who it suits

- A home baker or a small baking school that publishes recipes with good photos.
- A blog whose readers browse by topic: bread, cakes, biscuits, pastry.
- A blog that wants readers to sign up for the next bake by email.
- A blog moving from Ghost that wants to keep Taste's look.

If your blog is about quick weeknight meals, with times and servings on every card, [Harbor](harbor.md) suits it better.

## What the front page shows

1. The header: the menu at the start, your logo in the middle, then the search link and your call to action at the end. A long menu wraps onto a second row. On a tablet the logo moves to the start, and on a phone the menu opens from a Menu button.
2. A large, centred header between two thin rules: the blog's name, its intro under it (or its description when it has no intro), then what `header_action` picks:
   - **Subscribe**, the default: a field for an email address with a **Subscribe** button inside its end, when your blog takes sign-ups. A blog that takes no sign-ups shows your call to action as a button.
   - **Search**: a field that opens the search page, when your blog answers searches.
   - **None**: nothing under the intro.
3. The magazine block: the newest post across the row, its picture on one side and its topic, title and summary in the middle of the other. The next four posts sit in a row under it, each with its picture, its topic in the accent colour and its title.
4. One picture grid per topic, as many as `topic_sections` says (three by default). Each grid has the topic's name large in the accent colour and a link to all of its posts. The first grid shows three posts, the middle one twice as wide; the second shows four; and so on. The topics are your `[[reader.home_sections]]` when you set them, otherwise the topics with the newest posts.
5. The rest of the page's posts, four across, under **More posts**, then numbered page links.
6. The sign-up box between two thin rules, when your blog takes sign-ups.

Later pages show the header, then the page's posts four across.

## What a post shows

1. One centred line: the post's topic in the accent colour, the date and the reading time.
2. The title, large, and the subtitle under it.
3. The authors' names, the date of the last update and who reviewed the post.
4. The lead picture, as wide as the page.
5. The contents, folded under one line.
6. The post in one column, 22 px text.
7. A **Share this post** pill in the middle. It opens the share links and needs no script.
8. The tags.
9. The sign-up box between two thin rules, when your blog takes sign-ups.
10. Four more posts as picture cards, under a heading in the accent colour.

Taste turns the author box off. Turn it on in `[reader]` with `author_box = true` if you want it.

## List pages

A topic's or a tag's page shows its name large at the start, its intro, then its posts four across. Each card shows its picture, its topic in the accent colour and its title, centred. An author's page shows the author's round photo, name, role, profiles and bio, then their posts in the same grid.

## Your brand

Taste brings its own colours: near-black ink on a cream page, warm thin rules and a tomato red for the topic names and buttons. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site.

Taste's two styles map to your font tokens. Its "Elegant" style, the default here, sets everything in EB Garamond. For its "Playful" style, set `[fonts] heading` and `body` to Poppins. Both fonts are open fonts under the SIL Open Font License. Serve them from your blog: put the WOFF2 files in `theme/fonts/` with each font's `OFL.txt` beside them and list them under `[[fonts.files]]`, as the demo's sample brand does. Without the files, Taste shows Garamond or Georgia in their place.

Taste's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Taste's default |
|---|---|
| `[colors] background` | `#faf6e9`, Taste's cream |
| `[colors] accent` | `#b83a22`, a tomato red, deep enough for light text on it |
| `[fonts] heading` | EB Garamond, then Garamond and Georgia |
| `[fonts] body` | EB Garamond, then Garamond and Georgia |
| `[type] body` | `1.125rem`; a post's text is 22 px |
| `[type] h1` | `2.25rem` on a phone to `3.75rem` on a wide screen |
| `[type] h2` | `1.75rem` to `2.375rem` |
| `[type] h3` | `1.375rem` to `1.5rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `600` |
| `[shape] radius` | `0px`: square pictures, fields and buttons |
| `[shape] measure` | `36rem`, the width of a post's text |
| `[space] section` | `5rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `header_action` | `"subscribe"` | What the front page's header offers under its words: `subscribe`, `search` or `none`. |
| `topic_sections` | `3` | How many topics the front page shows as picture grids, from `0` to `6`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on topic, tag and author pages: `image` (Taste's grid), `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card on the front page shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"grid"` | How the posts after a post are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `4` | How many posts show after a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline-folded`, `inline`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"end"` | Where a post shows its Share pill: `end` after the post, `top` under the byline, `both`, or `none`. |
| `share_networks` | `["facebook", "x", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the sign-up box shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button, as Taste's Subscribe button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header button and the band do not show.

For example, a baking blog with a search field in its header and four topic grids:

```toml
[template]
name = "taste"
[template.options]
header_action = "search"
topic_sections = 4
```

## What Taste leaves out

Taste's Ghost features that a Khalid blog does not have are mapped or left out:

- Members and the Subscribe button: Taste shows Khalid's newsletter form where Ghost shows its sign-up field, and your call to action where Ghost shows its Subscribe button.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- Infinite scroll: numbered page links.
- The setting for the homepage's tag slugs: your `[[reader.home_sections]]`, or the topics with the newest posts.
- The header text and sign-up text settings: the header shows the blog's name and intro, and the sign-up box uses Khalid's own words.
- The site cover picture behind the header: Taste here keeps words off pictures, so the header sits on the page's own colour, as Taste draws it without a cover.
- The background colour setting: set `[colors] background` instead.
- The navigation layout setting: the logo stays in the middle, Taste's default.
- The gallery lightbox, social icons and "Powered by Ghost" in the footer. Khalid's own enlarge feature can open pictures.

## Good to know

- Everything reads with scripts off. Taste adds no script.
- The sign-up field and box show only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. They sign readers up to your default list.
- Pictures keep their own shape in the grids, as Taste shows them. Photos of the same shape make the tidiest rows.

## Design

This is Taste's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A baking blog with good photos that wants a calm, magazine-like front page and a sign-up box readers cannot miss.

**Reference.** Ghost's Taste theme, its templates and its stylesheet.

| Part | Taste |
|---|---|
| Type pairing | EB Garamond for titles and text, weight 600 for titles. Poppins for titles in the "Playful" style. |
| Colour | A cream page, near-black ink, warm thin rules. One accent for topic names, list headings, buttons and the bar beside quotes. |
| Spacing rhythm | 64 px between rows of cards and 48 px between columns. 100 px to 160 px between sections. |
| Frame at 1440 px and wider | At least **75rem** (1200 px). |
| Cards and lists | No box: the picture at its own shape with square corners, the topic in the accent, then the title, all centred. |
| Phone header and menu | The logo at the start and a Menu button at the end; the call to action moves into the menu. |
| Tabs and indexes at 390 px | Every card takes the whole width; the magazine block's first post stacks its picture over its words. |
| Link, card and focus states | Links and card titles fade a little on hover. Focus is a 3 px accent ring. |
| Signature details | The logo in the middle of the header. The large centred header between two thin rules with the square sign-up field. The magazine block's first post beside its picture. Topic grids that alternate three and four across, the first with a wide middle post. The topic names in the accent. |

**Sample brand.** The demo uses `fixtures/showcase-brands/taste/theme/`: a home baking blog on flour-cream paper (`#fdf8f1`) with cocoa ink (`#2a1d16`), a cinnamon accent (`#a8461a`) and a warm panel (`#f5ebdd`). Poppins for headings and EB Garamond for the text, both served from the blog with their OFL licences, taken from each font's own project (github.com/itfoundry/Poppins and github.com/octaviopardo/EBGaramond12). The mark is a scored loaf.
