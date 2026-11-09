---
title: "Almanac"
order: 10
---

# Almanac

Almanac is the template for a library of long guides. It suits the post that is a numbered list of forty ideas or a step-by-step guide of five thousand words: readers jump to the part they need, and when they finish, they find the next guide straight away.

## Who it suits

- A company blog that works as a resource centre, with guides grouped by topic.
- Long list posts and how-to guides with many pictures, tables and a short summary at the top.
- A team with a product to offer, because Almanac places your call to action near the top of the front page, after each post and inside posts where you write `::cta`.

For short news posts, `classic` or Workbench fit better.

## Pick it

```toml
[template]
name = "almanac"
```

Your colours, fonts and logo still come from theme sync. Until you sync a theme, Almanac uses its own palette: warm paper, ink-dark text and a deep green accent. The accent marks numbers, labels, links and the one button; tints of it shade the call-to-action band, the newsletter band and callouts.

Almanac numbers everything a reader may want to find again: each section of a post opens with its number, the contents beside the post show the same numbers, and the newest posts and the editor's picks on the front page are numbered lists.

## What the front page shows

1. A row of your category links, with the rest under "More". The menu works without scripts.
2. The newest post marked `featured: true`, its picture beside its title on a wide screen, then a band with your call to action.
3. The month index, like an almanac's "In the garden this month". The month of the newest guide (the featured guide when there is one) is set large in the accent colour, with the year under it. Under that is a strip of the twelve month initials: the current month filled, and the months that have guides marked. Beside it, the guides from that month and the two months before it, up to four, are a numbered list, each with its category and reading time. When no guide is that recent, this place shows "Latest posts" instead: the next four posts as a numbered row, each with its category, title and date.
4. "More posts": up to six more, leaving out the posts shown above, as picture cards, two in a row, with the editor's picks as a numbered list beside them.
5. The newsletter form.
6. Your home sections, each with up to four cards and a link to see all.
7. The page links, then your call to action again to close the page.

## What a post shows

1. Breadcrumbs: the front page, the category, the post.
2. The title, the post's `description` as a subtitle, the writers with their photos, the date and the reading time.
3. The lead picture, across the text and the contents on large screens.
4. "On this page": on a large screen a column beside the text that stays in view as the reader scrolls; on a phone a folded box above the text. Each section shows its number.
5. The body, each section opened by a thin rule and its number. From 1024 px the number hangs in the left margin under a short accent rule, level with the first line of the heading. On a phone, and on a screen where "On this page" leaves no margin, the number starts the heading's line, and a second line of the heading lines up with the text, not under the number.
6. The writers again with their roles, the date it was last updated, and why, from `updated_note`.
7. Share links, the newsletter form and your call to action.
8. "Keep reading": four related posts as short rows. Then "Related posts": up to six more as picture cards.

## Writing a guide

Almanac styles the blocks a long guide needs. Write them in your post as usual:

- A short summary at the top: `:::callout{kind=summary}` with your key takeaways. It shows as a tinted box.
- A jump list for a list post: `::toc` on its own line, where you want the list of the post's headings.
- Pictures with captions: `:::figure{src="..." alt="..."}` with the caption inside. Captions show in small italic type.
- Data tables, and long ones folded with `:::table{fold=8}`.
- A note on how you did your research: `:::callout{kind=note}`.
- Further reading at the end: `:::callout{kind=read}` with a list of links.
- The questions readers ask: `::::faq`.
- Your call to action inside the post: `::cta` on its own line, once you have `[template.cta]`. You may place it up to three times.

## Options

Set these under `[template.options]`. An option you leave out takes the default.

| Option | Default | What it does |
|---|---|---|
| `hero_stats` | none | Up to four numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the front page to it. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `true` | An updated post says why it was updated, from its `updated_note`, at the end. |
| `cards` | `image` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `card_author_role` | `false` | Each card shows its author's role. |
| `featured` | `true` | The newest post with `featured: true` shows large at the top. |
| `picks` | `5` | How many posts marked `featured: true` the front page lists beside the grid, from 0 to 8. |
| `pagination` | `numbered` | The links under a long list: `numbered`, or `pages` for newer and older only. |
| `related` | `grid` | How related posts are laid out. `grid` shows four as short rows, then the rest as picture cards. `list`, `row`, `scroll` and `links` show one set. |
| `related_count` | `10` | How many related posts show in all, from 3 to 10. |
| `contents` | `side-end` | Where "On this page" goes: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. |
| `reading_progress` | `bar` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `button` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `none` | How the contents are numbered: `none`, `numbered` or `nested`. With `none`, Almanac shows its own section numbers. |
| `share` | `end` | Where share links show: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `both` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. On the front page it shows twice: after the featured post and at the end. |
| `cta_button` | `true` | The header shows your `[template.cta]` as a button. |
| `lead_image` | `true` | A post shows its lead picture under the byline. |
| `byline_end` | `true` | When the author box is off, a post ends with its writers again and the date it was last updated. |
| `newsletter_note` | none | One line under the newsletter form, such as what readers get and how you treat their address. |

For example:

```toml
[template.options]
newsletter_note = ["One guide a week. We never share your address."]
picks = 3
cta_band = "post"
```

The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes

Almanac sets these defaults. A value in your `theme/tokens.toml` wins.

| Token | Almanac |
|---|---|
| Colours | background `#fcfbf8`, surface `#f3f0e8`, text `#1c201d`, muted `#575c56`, link `#17604a`, accent `#1d6b53`, border `#e2ddd1` |
| Body text | 1.0625rem to 1.125rem |
| `h1`, `h2`, `h3`, `h4` | 2.25rem to 3.375rem, 1.5rem to 2rem, 1.1875rem to 1.375rem, 1.0625rem to 1.125rem |
| Heading weight | 800 |
| Line height | 1.7 |
| Line length (`measure`) | 42rem |
| Corners (`radius`) | 0.5rem |
| Space between sections | 4.5rem |

## Good to know

- Each post ends with the author box: the writers' photos, roles, why to trust them, a short bio and their profiles. Write `author_box = false` under `[reader]` to show the shorter line of writers and the date instead.
- The row of category links and the category on each short row need your categories on the front page and on post cards. Khalid does not fill them in yet, so they stay hidden until it does.
- The front page's home sections show once Khalid fills in `[[reader.home_sections]]`.
- Some resource libraries show "Popular posts". Khalid has no reading counts on a built site, and shows no public view counts, so Almanac shows "Keep reading" from the related posts in their place.
- One call to action per site: every `::cta` and band shows the same `[template.cta]`.
- Almanac reads with scripts off, and adds no script.

## Design

This is Almanac's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A team that publishes a library of long guides: one subject at a time, explained in depth, read with care and kept up to date. Readers arrive from a search, read for twenty minutes and come back to the same guide later.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A large web design magazine with long technical guides. It sets articles in a warm reading serif near 18 px with a quick summary above the text, and the author sits in a side column on wide screens.
- A research group that publishes usability guides. Plain white pages, a strict text column, and lists of articles that read as a quiet index of titles and dates.
- One writer's blog of long interactive tutorials. Generous space, big friendly headings, and asides drawn as panels inside the text.

| Part | Almanac |
|---|---|
| Type pairing | A soft display serif for headings over a reading serif for text. With no theme, the pack uses the system's book serif (Iowan Old Style, Charter, Georgia). |
| Colour | Warm paper ground, ink text and one deep natural accent (moss or forest green). Panels are a shade darker than the paper, never grey. |
| Spacing rhythm | A base step of 0.5rem. Sections 5rem apart on a wide screen, 3rem on a phone. Paragraphs one line apart. Headings get twice the space above as below. |
| Frame at 1440 px and wider | At least **76rem**: the reading column plus "On this page" beside it. |
| Cards and lists | Guide cards are text first: number or category, title, two lines of summary, then reading time. Pictures sit in a fine frame and never fill the card edge to edge. Lists are ruled rows, not boxes. |
| Phone header and menu | The logo on the left, the menu on the right. The row of category links becomes one line you can swipe, and "More" opens below it with no script. |
| Tabs and indexes at 390 px | Topic tabs become one row you can swipe, the current tab underlined in the accent. "On this page" is the folded box above the text, one line high until opened, with the section numbers in the accent. |
| Link, card and focus states | Links in text are underlined in the accent. On hover the underline thickens. A card's title takes the underline on hover and the whole card is the target. Focus is a 2 px accent ring with 2 px of space around it. |
| Signature details | Each section opens with a thin rule and its number set large in the accent, hanging in the margin on a wide screen, and "On this page" repeats the same numbers. The front page's month index: the month set large, a strip of month initials and that season's guides as a numbered list of jobs. "Latest posts" and the editor's picks are numbered lists, like a book's contents. The summary callout at the top of a guide is a paper-tinted panel with a fine rule, not a coloured box. |

**Sample brand.** The demo uses `fixtures/showcase-brands/almanac/theme/`: moss green (`#4a6418` links, `#56731c` accent) on warm paper (`#f9f6ee`), Fraunces for headings and Source Serif 4 for text, and a leaf mark.
