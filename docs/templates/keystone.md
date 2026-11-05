---
title: "Keystone"
order: 60
---

# Keystone

Keystone is a template for a product or company blog: a team that writes about what it builds and what it learns, and wants the blog to look as calm and finished as its main site. It leads with one big story, shows the rest as a grid of framed pictures, and gives each post a quiet reading column with the author and related posts after it.

## Who it suits

- **It suits** a software company, a studio or an agency with several writers, where each post has a good lead picture.
- **It suits** product news, engineering notes, guides and case studies side by side, sorted by category.
- **It suits less well** a personal site with one writer and no pictures, or a news site that publishes many short posts a day. Ledger or Wire fit those better.

## What it shows

**The front page**

1. Your logo in the header, the menu as quiet links, and your call to action as a button when `[template.cta]` is set. On a phone the menu is one line you can swipe, and the button waits for the band further down.
2. The blog's name, with its topics on one line under it, when your blog lists its topics.
3. The newest post marked `featured: true` as the lead story: its picture large on one side on a wide screen, its title, summary and author on the other.
4. "Recently published": your other posts, first two large framed picture cards, then rows of three, then from the ninth post a text-only index of titles in two columns. On a phone, posts three to six are small rows with the picture beside the title, and the index rows show just the title and the date. Each card shows the category, the title, two or three lines of summary, then the author and the date. A post with no picture gets a drawn cover in tints of your accent colour, so the grid stays even.
5. The newsletter form on a framed panel, with one line over it such as "Join 4,000 readers", when your blog takes sign-ups.
6. One row of three cards for each home section in `[reader]`, each with a "View all" link.
7. Your call to action as a band, when `cta_band` is `home` or `both`.

Lists such as a category or the archive show the lead story and the same picture cards, with numbered page links under them.

**A post**

1. The trail of links from the front page to the post, then the category, the title and the summary (the post's `dek`), all on one left edge.
2. One line with the author's photo, name and role, then one date (the day it was last updated, or the day it was published if it never was) and the reading time. An updated post with an `updated_note` says why it changed on the next line.
3. The lead picture, wider than the text, in a fine frame.
4. The contents and the share links. On a wide screen (from 70rem) they sit in a column on the far side of the text, behind a fine line, and stay in view as you scroll: first "On this page", the post's main sections numbered 01, 02, 03 with the one you are reading in the accent, then the share links. The text and the column sit together in the middle of the page, under the title, and a contents block in the post's own text hides, as the column shows the same list. On a phone the share links sit in one row above the post, and the contents block in the text, or else a folded contents box, comes first.
5. The post in one reading column. Callouts, steps, tables, quotes, questions and the contents block each have their own quiet look.
6. Your own lines about the site, when you set them, then the author box.
7. The newsletter form, your call to action, the posts that link here and up to five related posts, three across on a wide screen.

**Callouts** (`:::callout{kind=note}`, `tip`, `warning` and `read`) each get a small drawn icon; a warning also gets a dark edge, so the kinds differ by more than colour.

Everything reads with scripts off. Keystone adds no script.

## Options

Set these under `[template.options]` in `site.toml`. An option you leave out takes the default.

| Option | Default | Values | What it changes |
|---|---|---|---|
| `hero_stats` | none | up to four lines | Numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | `true`, `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | `true`, `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | `true`, `false` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `logo` | `true` | `true`, `false` | The header shows the logo from your main site in place of the blog's name. Without a logo, the name shows. |
| `cta_button` | `true` | `true`, `false` | The header shows your `[template.cta]` as a button. |
| `breadcrumbs` | `true` | `true`, `false` | Each page shows the trail of links from the front page to it. |
| `dek` | `true` | `true`, `false` | A post shows its summary under its title. |
| `updated_note` | `true` | `true`, `false` | An updated post says why it was updated, from its `updated_note`, under its date. |
| `featured` | `true` | `true`, `false` | The newest post marked `featured: true` shows large at the top of the front page and each list. |
| `cards` | `image` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists. |
| `pagination` | `numbered` | `pages`, `numbered` | The links under a long list: newer and older, or a link to each page. |
| `related_count` | `5` | 3 to 10 | How many related posts show after a post. Five is the most a post can pin with `related`. |
| `contents` | `side-end` | `none`, `inline`, `inline-folded`, `side-end`, `side-start` | Where a long post's contents list goes. `side-end` puts it at the top of the column beside the text on a wide screen, over the share links. A post can also carry its own contents block, which then shows only on a phone. Before November 2026 the default was `none`. |
| `reading_progress` | `bar` | `bar`, `none` | A thin line at the top of a post that fills as the reader scrolls. |
| `header_search` | `button` | `button`, `link`, `none` | What the header's search does on a served blog: open a search dialog, open the search page, or nothing. |
| `share` | `top` | `none`, `top`, `end`, `both` | Where a post shows its share links. `top` is the column beside the post. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | any of `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in order. |
| `newsletter` | `both` | `none`, `post`, `home`, `both` | Where the newsletter form shows. It shows only when your blog takes sign-ups. |
| `cta_band` | `post` | `none`, `post`, `home`, `both` | Where a band with your `[template.cta]` shows. |
| `numbered_headings` | `false` | `true`, `false` | Whether each main heading of a post gets a chapter number: 01, 02, 03. |
| `subscriber_line` | none | a list of one line | One line over the newsletter form on the front page. |
| `about` | none | a list of up to two lines | Lines about your site after each post. |

An example:

```toml
[template]
name = "keystone"

[template.options]
subscriber_line = ["Join 4,000 people who read every post."]
about = ["Field Notes is written by the team that builds Field, in Bristol."]
share_networks = ["linkedin", "x", "email"]
cta_band = "both"
```

## Turning features off

Keystone keeps every reader feature on: the author box, related posts and the reading time. A key you write in `[reader]` wins over the template:

```toml
[reader]
author_box = false      # the author's photo and bio after each post
related_posts = false   # three related posts after each post
reading_time = false    # "8 min read" beside the date
```

## Sizes and spacing

Keystone sets a bold type scale on a wide frame: up to 90rem, with the blog's name up to 6.5rem tall on the front page. Your own `theme/tokens.toml` wins over each value.

| Token | Keystone |
|---|---|
| Body text | 1.0625rem on a phone to 1.1875rem on a wide screen; a post's text grows to 1.25rem from 64rem and 1.3125rem from 100rem |
| Main title (`h1`) | 2.375rem to 4.5rem |
| Main headings (`h2`) | 1.625rem to 2.125rem |
| Headings (`h3`) | 1.25rem to 1.375rem |
| Small text | 0.875rem to 0.9375rem |
| Line height | 1.6 |
| Heading weight | 600, where your heading font has it; the blog's name on the front page uses 700 |
| Space between sections | 5.5rem |
| Corners | 0.75rem |
| Line length | 35rem, about 65 to 75 characters a line at the 19 px text; from 64rem the column grows with the larger post text, so a line keeps its length |

Keystone uses your system's own interface font, so it looks native on every device and loads nothing. It brings calm colours: a cobalt accent (`#3355e0`) and deep ink text (`#0d1526`) on a white page, with a faint cool grey (`#f4f6fa`) for panels. Your colours, fonts and logo from your main site, through theme sync, replace them, as in every template.

## Design

This is Keystone's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A software company, studio or agency that wants its blog as calm and finished as its main site. Several writers, a good lead picture on each post.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A payments company's blog. The lead story on a raised white card with its picture beside it, faint vertical guide lines behind the page, category tabs as small pills, then each post as a wide row: title and summary on one side, the date and the author's photo and role in a narrow column, the picture framed below. Refined type at medium weights and very even spacing. Its frame is near 67rem; Keystone runs wider so a large screen is not left empty.
- A note-taking software company's blog. Simple picture cards in a grid, friendly drawn covers, and a quiet reading column.
- A publishing platform's own magazine theme, the bar for ambition: a lead story that owns the top of the screen, bold display type, and lists that change shape further down the page. Keystone takes only the level of ambition from it, not its layout or type.

| Part | Keystone |
|---|---|
| Type pairing | A modern sans for headings at weight 600 or 700 over a humanist sans for text. With no theme, the pack uses the system interface font for both. |
| Colour | White ground, deep ink text and one calm accent (blue or indigo). A faint cool grey for panels and frames. |
| Spacing rhythm | A base step of 0.5rem. Even and calm: 5rem between sections, 2rem grid gaps, 1.25rem inside cards. |
| The one idea | **The drawn frame.** Two fine guides run down the page at the frame's edges, as on a drafting sheet, and everything is built inside them: the name, the lead panel, the framed pictures, the index. You can name it from its thumbnail by the huge name over a ruled row of pills and the split lead panel. |
| Type scale | The blog's name on the front page from 3rem on a phone to 6.5rem on a wide screen, weight 700, tracked tight (-0.05em). A post's title up to 4.5rem, the lead story's title up to 3.5rem, section headings up to 2.25rem over a full-width ink rule. |
| Frame at 1440 px and wider | At least **80rem**; Keystone runs up to 90rem, so a 2000 px screen is filled. |
| Front page structure | The masthead, then the lead story as one framed panel split in two (picture flush on one side, label, title, summary and a ruled author line on the other), then the newest posts: two large cards, two rows of three, and from the ninth post on a text-only index in two columns, each entry a hairline, the category, the title, the author and the date. Category lists follow the same order. The front page lists every post, so it is the path to each one. On a phone: the lead story and two large cards, then four small rows with the picture beside the title and the date, then one ruled index row per post, its title on one line and the date under it, with no photo or label; a row of videos or podcasts starts with one small row. The 40-post demo stays near 6,000 px. |
| Cards and lists | Framed picture cards: the picture inside a fine border with a small radius, then the category, a title of up to three lines, two or three lines of summary, the author and the date. |
| Phone header and menu | The logo on the left and the menu as one line you can swipe, its last 2rem fading out so it reads as more to swipe; the topic pills do the same. The button waits for the band further down. |
| Tabs and indexes at 390 px | Topic tabs become one row you can swipe, the current tab underlined. Topic tab panels show their posts as one column of compact rows. |
| Link, card and focus states | Links in the accent with an underline that appears at full strength on hover. A card's picture frame turns the accent on hover. Focus is a 2 px accent ring with 2 px of space. |
| Signature details | The keystone wedge: a small accent shape, wider at the top, before each label (the lead story, a post's category, the subscriber line). The two frame guides. The lead story's split panel. Drawn covers in tints of the accent for posts with no picture, so the grid stays even. The post's contents and share links in a column on the far side of the text that stays in view. |

**Sample brand.** The demo uses `fixtures/showcase-brands/keystone/theme/`: indigo (`#4338ca` links, `#4f46e5` accent) on white with a cool panel (`#f6f6fb`), Manrope for headings and Source Sans 3 for text, and an arch mark.
