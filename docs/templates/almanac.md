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

Your colours, fonts and logo still come from theme sync. Almanac draws no colour of its own: the call-to-action bands sit on your surface colour with the button in your accent colour.

## What the front page shows

1. A row of your category links, with the rest under "More". The menu works without scripts.
2. The newest post marked `featured: true`, large, then a band with your call to action.
3. "Latest posts": the next four posts as short rows, each with its category, title and date.
4. "More posts": up to six more as picture cards, three in a row, with the editor's picks as a list of links beside them.
5. The newsletter form.
6. Your home sections, each with up to four cards and a link to see all.
7. The page links, then your call to action again to close the page.

## What a post shows

1. Breadcrumbs: the front page, the category, the post.
2. The title, the post's `description` as a subtitle, the writers with their photos, the date and the reading time.
3. The lead picture, wider than the text on large screens.
4. "On this page": the contents in an open box, without numbers.
5. The body.
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
| `contents` | `inline` | Where "On this page" goes: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `none` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `end` | Where share links show: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `both` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. On the front page it shows twice: after the featured post and at the end. |
| `cta_button` | `true` | The header shows your `[template.cta]` as a button. |
| `lead_image` | `true` | A post shows its lead picture under the byline. |
| `byline_end` | `true` | A post ends with its writers again and the date it was last updated. |
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
| Body text | 1.0625rem to 1.125rem |
| `h1`, `h2`, `h3` | 2.25rem to 3.5rem, 1.625rem to 2.25rem, 1.25rem to 1.375rem |
| Line height | 1.65 |
| Line length (`measure`) | 70ch |
| Corners (`radius`) | 1rem |

## Good to know

- The author box is off in Almanac, because the writers show again at the end of each post. Write `author_box = true` under `[reader]` to show it as well.
- The row of category links and the category on each short row need your categories on the front page and on post cards. Khalid does not fill them in yet, so they stay hidden until it does.
- The front page's home sections show once Khalid fills in `[[reader.home_sections]]`.
- Some resource libraries show "Popular posts". Khalid has no reading counts on a built site, and shows no public view counts, so Almanac shows "Keep reading" from the related posts in their place.
- One call to action per site: every `::cta` and band shows the same `[template.cta]`.
- Almanac reads with scripts off, and adds no script.
