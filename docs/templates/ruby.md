---
title: "Ruby"
order: 85
---

# Ruby

Ruby is a template for a magazine of picture cards. The front page is a grid of white cards with round corners, four across on a wide screen, on a page that fades from white to light grey. Each card shows its picture, its topic, its title, a short summary, the date and the author's photo. A post reads in one calm column, and more cards follow it in a grey band.

Based on Ghost's Ruby theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "ruby"
```

## Who it suits

- A magazine or blog with a good picture for every post.
- A blog with many short posts that readers browse rather than read in order.
- Readers who skim: every card shows the topic, the title and two or three lines of summary.
- A blog moving from Ghost that wants to keep Ruby's look.

## What the front page shows

1. The header: the menu at the start, the logo or name in the middle, and the search link and your call to action at the end.
2. The card grid: four cards across on a wide screen, three on a laptop, two on a tablet and one on a phone. Each card shows:
   - the post's picture at 16 by 9, on the surface colour while it loads
   - the topic in small capitals in the link colour, with a **Featured** mark before it when the post is featured
   - the title in bold
   - up to three lines of summary
   - a footer under a thin line with the date, and the author's round photo at the end.
   The whole card is the link to the post. The topic keeps its own link to the topic's page.
3. Numbered page links.
4. The topics, when the blog lists them.

Sections set in `site.toml` take the grid's place, each as its own grid of cards under a small grey heading.

## What a post shows

1. One line in small grey capitals: the authors, the topic in the link colour, the date and the reading time.
2. The title, large and bold, and the subtitle under it.
3. Who reviewed the post, when it was reviewed.
4. The lead picture, as wide as the text.
5. The contents, folded under one line.
6. The post in one column 640 px wide, 18 px text.
7. The tags.
8. The share links, centred under the post.
9. The sign-up box, when your blog takes sign-ups.
10. A grey band across the whole window with four more posts as Ruby's cards, under a small heading.

Ruby turns the author box off. Turn it on in `[reader]` with `author_box = true` if you want it.

## List pages

A topic's or a tag's page shows its name and intro at the start, then its posts as Ruby's cards, as many across as fit. An author's page shows the author's round photo above their name, role, profiles and bio, then their posts in the same grid.

## The header

The logo sits in the middle of the header. The menu takes the start and stops before the logo; a long menu scrolls sideways. The search link and your call to action sit at the end. On a phone, the logo moves to the start and the menu opens from a Menu button, with no script.

## Your brand

Ruby brings its own colours: dark grey ink on white, a light grey page under the cards and a deep teal for topics, links and buttons. Ghost's teal is too light for text, so Ruby's is deeper. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site.

Ruby's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Ruby's default |
|---|---|
| `[colors] surface` | `#efefef`, the grey the page fades to |
| `[colors] link` | `#0f716a`, a deep teal |
| `[colors] accent` | `#117a71` |
| `[fonts] heading` and `body` | The system's sans |
| `[type] body` | `1.125rem`; a post's text is 18 px |
| `[type] h1` | `1.875rem` on a phone to `2.375rem` on a wide screen |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.5rem`: cards and pictures have round corners |
| `[shape] measure` | `38rem`, the width of a post's text |
| `[space] section` | `4rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on topic, tag and author pages: `image` (Ruby's card), `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card on the front page shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"grid"` | How the posts after a post are laid out: `grid` (Ruby's cards), `list`, `row`, `scroll` or `links`. |
| `related_count` | `4` | How many posts show after a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline-folded`, `inline`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"end"` | Where a post shows its share links: `top`, `end`, `both` or `none`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the sign-up box shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button. |
| `hero_stats` | none | Up to four numbers above the front page's cards, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header button and the band do not show.

For example, a blog with the contents beside long posts and six posts after each one:

```toml
[template]
name = "ruby"
[template.options]
contents = "side-end"
related_count = 6
```

## What Ruby leaves out

Ruby's Ghost features that a Khalid blog does not have are mapped or left out:

- Members, paid tiers and the sign-in link: Ruby shows Khalid's newsletter form and your call to action in their place.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- Infinite scroll: numbered page links.
- Featured cards drawn with their title over the picture: a featured card keeps Ruby's normal shape and says **Featured** beside its topic, since Khalid never sets text over a picture.
- "Read more" in each card's footer: the footer shows the date.
- The previous and next post links under a post: the grey band of related posts takes their place.
- The logo-on-the-left and stacked header settings: the logo always sits in the middle.
- The title and body font settings: your font tokens set both.
- Social icons and "Powered by Ghost" in the footer: the footer shows your footer links.
- The gallery lightbox. Khalid's own enlarge feature can open pictures.

## Good to know

- Everything reads with scripts off. Ruby adds no script.
- The sign-up box shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.
- A card without a picture starts with its topic and keeps the same footer.

## Design

This is Ruby's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A magazine or picture-led blog that wants Ghost's Ruby look: many tidy cards, a quiet page and a calm post.

**Reference.** Ghost's Ruby theme, its templates and its stylesheet.

| Part | Ruby |
|---|---|
| Type pairing | One sans for everything: bold titles, small bold capitals for topics, dates, menus and buttons. |
| Colour | A page that fades from white at the top to the surface grey. White cards. Dark grey ink, a mid grey for labels and one accent for topics and links. A post page is plain white. |
| Spacing rhythm | 32 px between cards, 20 px inside them. 96 px between a post and the related band. |
| Frame at 1440 px and wider | **75rem** (1200 px), the header and the grid alike. |
| Cards and lists | A white card with 8 px corners and a faint shadow that grows on hover. The picture at 16 by 9, the topic, the title at 16 px, up to three lines of 14 px summary, then a 42 px footer under a thin line. Four across, then three, two and one. |
| Phone header and menu | The logo at the start and a Menu button at the end; the call to action moves into the menu. |
| Tabs and indexes at 390 px | One card across, each the full width. |
| Link, card and focus states | A card's shadow grows on hover. Focus draws a 3 px accent ring around the whole card. Links in a post are underlined in the link colour. |
| Signature details | The fading grey page under white cards. The logo in the middle of the header. The meta line in small grey capitals over a post's title. The grey related band across the window. |

**Sample brand.** The demo uses `fixtures/showcase-brands/ruby/theme/`: a deep ruby red (`#a10f38` links, `#b3123f` accent) on white with a warm light grey (`#f1eeee`), the system's sans for everything, as Ruby's "Modern sans-serif" setting, and a cut gem as its mark.
