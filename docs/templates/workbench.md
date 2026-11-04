# Workbench

Workbench is the template for blogs that review and compare tools or products. It is built for the post that weighs five options against each other: a table that shows them at a glance, a section per option with its pros and cons and its price, and a plain list of further reading at the end.

## Who it suits

- A software company that publishes "best tools for" lists and head-to-head comparisons.
- A team that writes buying guides, where readers scan a table first and read the detail second.
- A blog that grows its newsletter, because Workbench offers the sign-up form twice on the front page and once after each post.

It suits writing with clear sections and tables. For long essays with few headings, `classic` reads more calmly.

## Pick it

```toml
[template]
name = "workbench"
```

Your colours, fonts and logo still come from theme sync. Before you sync, Workbench uses its own: a grey-white bench, graphite text and one verdict green, in your system's sans and code fonts. Labels, rules and panels always use your tokens.

## What the front page shows

1. Your blog's name in very large type on squared paper, over a thick rule, and a "Categories" button that opens a menu of your categories. The menu works without scripts.
2. The newest post marked `featured: true`, large, under a green rule. Its picture keeps a 16 by 9 shape.
3. "Latest posts": the next five posts as numbered rows, 01 to 05, each with its category, title, date and reading time. On a wide screen they sit beside the featured post.
4. The newsletter form.
5. "More posts": the rest as picture cards in ruled cells, like one sheet, up to four in a row on a wide screen.
6. The newsletter form again, then the page links and "See all posts", which opens the archive.

## What a post shows

1. The category as a small label, the title, and the post's `description` as a subtitle.
2. The writers, the date and the reading time.
3. The contents in an open box at the top.
4. The body. Each section opens under a green rule. A paragraph that starts with a bold label, such as **Price:**, shows the label in the code font, like a line on a spec sheet. Wide tables can grow past the text, up to 60rem on large screens, so a comparison table has room. The header row sits on your surface colour, and on a phone the first column stays in place while the table scrolls sideways.
5. A closing line in italics: who wrote it, when it was published and when it was updated.
6. Share links, the author box, the newsletter form, and six related posts as plain links.

## Writing a comparison post

Workbench styles the blocks a comparison needs. Write them in your post as usual:

- A criteria list: a list where each item starts with **bold words**.
- The table: a normal markdown table. Wrap a long one in `:::table{fold=8}` so it shows eight rows and a line to open the rest. To mark the winner in a row, write its cell all in **bold**: Workbench tints that cell in your accent colour.
- Each option's section: a `##` heading, then `:::proscons` for its pros and cons, then `:::review` for its rating, or a line with its price in **bold**.
- The questions readers ask: `::::faq`.
- Your call to action: `::cta` on its own line, once you have `[template.cta]`.

## Options

Set these under `[template.options]`. An option you leave out takes the default.

| Option | Default | What it does |
|---|---|---|
| `hero_stats` | none | Up to four numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the front page to it. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `image` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `card_author_role` | `false` | Each card shows its author's role. |
| `featured` | `true` | The newest post with `featured: true` shows large at the top. |
| `pagination` | `pages` | The links under a long list: `pages` for newer and older, or `numbered`. |
| `related` | `links` | How related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `6` | How many related posts show, from 3 to 10. |
| `contents` | `inline` | Where the contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `reading_progress` | `bar` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `button` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `numbered` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `share` | `end` | Where share links show: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "x", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page, `home` and `both` show it twice. |
| `cta_band` | `none` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your `[template.cta]` as a button. |
| `lead_image` | `true` | A post shows its lead picture under its title, framed on a shadow of the accent. Turn it off when your comparison posts open with their table. |
| `closing_line` | `true` | A post ends with one line naming its writers and its dates. Turned off, the updated date shows beside the published date instead. |
| `newsletter_note` | none | One line under the front page's newsletter forms, such as how often you send. |

For example:

```toml
[template.options]
newsletter_note = ["A new review every Tuesday."]
related_count = 4
share = "both"
```

The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes

Workbench sets these defaults in `crates/khalid-theme/packs/workbench/tokens.toml`. A value in your `theme/tokens.toml` wins.

| Token | Workbench |
|---|---|
| Colours | background `#f7f7f5`, surface `#ecedea`, text `#1a1c1e`, muted `#52575c`, link `#0d6a43`, accent `#11794c`, border `#d3d6d0` |
| Fonts | the system sans for headings and text, the system code font for labels, dates and prices |
| Body text | 1.0625rem to 1.1875rem |
| `h1`, `h2`, `h3`, `h4` | 2.25rem to 3.5rem, 1.5rem to 2rem, 1.1875rem to 1.375rem, 1.0625rem to 1.125rem |
| Small text | 0.875rem |
| Line height | 1.6 |
| Line length (`measure`) | 38rem, about 70 letters a line |
| Corners (`radius`) | 0.125rem, nearly square |
| Spacing | a 0.5rem step, 3.5rem between sections |
| Frame | 84rem for the header, the lists and the footer |

## Good to know

- The "Categories" menu needs your categories on the front page. Khalid does not fill them in yet, so the menu stays hidden until it does.
- The category on the rows under "Latest posts" and above a post's title works the same way: it shows once Khalid fills in a post's category.
- Workbench reads with scripts off, and adds no script.

## Design

This is Workbench's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A team that tests tools or products and publishes "best of" lists and head-to-head comparisons. Readers scan a table first and read the detail second.

**The one idea.** A test lab's bench sheet. Heavy, tight headings over thick rules, squared paper behind the name, numbered rows, ruled cells and every fact in the code font. You should be able to name it from a small picture of its front page.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A product testing lab's review site. A count of products bought and tested above every guide, each pick under a heavy accent rule with its name, its test pictures and a row of scores, the specs as short labelled pairs, and a note on how the lab tests.
- A consumer technology review site. Buying guides with a quick list of picks, specs tables and ratings, and lists of reviews as rows.

| Part | Workbench |
|---|---|
| Type pairing | A technical grotesque for headings over a plain engineering sans for text, with mono details: prices, ratings, dates and labels. With no theme, the pack uses the system sans and the system mono. |
| Type scale | Display type is heavy (800) with tight tracking (-0.04em to -0.05em). The front page's name runs from 3rem on a phone to 6.5rem on a wide screen, a list page's title to 5rem, a post's title to 3.75rem (5rem from 1280 px) and the pick's title to 3.25rem. |
| Front page structure | The masthead owns the top: the name on squared paper over a thick rule. Below it, a lead grid: the pick on the left, a numbered rail of the five newest posts on the right. Then the rest as ruled cells of one sheet: twelve picture cells on a wide screen, then the sheet's ruled rows, two across, each a title, its date and its category. The front page still links every post. |
| Colour | A grey-white bench ground, graphite text and one verdict colour (green) for picks, labels and pros. Panels a shade darker than the ground. Workbench now has a full palette of its own. |
| Spacing rhythm | A base step of 0.5rem on a strict grid. Ruled sections 3.5rem apart. Table rows 0.75rem high in padding. |
| Frame at 1440 px and wider | At least **80rem**. A comparison table may grow past the text to 60rem, so the frame leaves it room. |
| Post at 1280 px and wider | A spec sheet across the frame: the title (up to 5rem) and a wider lead picture (2 by 1) span 61rem; below them the contents box sits in a rail that stays in view beside the text. The text grows to 1.25rem and its measure to 40rem, so a line still holds 60 to 75 letters. |
| Cards and lists | Picture cards with square corners in ruled cells that share their rules, the category as a mono label, the title, the date. The featured picture keeps a fixed ratio (16 by 9) and never fills the window: on a list page it sits beside its words. |
| Phone header and menu | The blog's name and the "Categories" button on one row; the menu opens below with no script. The site menu is one row you can swipe, its last 2rem faded so it reads as more to see. |
| Phone lists | The first six cells two across, as a contact sheet: picture, date and title. Every later post is a ruled row: the title and the date small. The 40-post demo's front page stays near 6,000 px. |
| Tabs and indexes at 390 px | Topic tabs as one row you can swipe. A wide comparison table scrolls sideways inside its frame, with the first column fixed. |
| Link, card and focus states | Links underlined in the verdict colour. A card's top rule turns the verdict colour on hover. Focus is a 2 px ring in the text colour. |
| Signature details | The comparison table as the hero of a post: header row on the panel colour, the winning cell marked. Pros and cons as two ruled columns with plus and minus marks. Mono labels for prices and ratings, and for the bold label that opens a paragraph. Numbered rows (01, 02) in the newest posts, the contents and the related links. |

**Sample brand.** The demo uses `fixtures/showcase-brands/workbench/theme/`: verdict green (`#10744a` links, `#117d50` accent) on a grey-white bench (`#fafaf8`), Space Grotesk for headings and IBM Plex Sans for text, and a hex-and-check mark.
