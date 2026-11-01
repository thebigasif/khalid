# Templates: the look of your blog

A template sets the layout and features of your blog's pages: where the contents list sits, which cards the home page shows, and the sizes of text and space. Your colours, fonts and logo still come from your main site through theme sync, so every template looks like your brand.

Every template is built into Khalid. You pick one in `site.toml` and adjust that template's options. You cannot mix parts of different templates.

## Picking a template

```toml
[template]
name = "classic"
```

Without a `[template]` table your blog uses `classic`, the look Khalid has always had. Updating Khalid never changes your template.

Classic is the only template today. The ten templates in the templates spec arrive one or two at a time, and each one lists its options here.

## Options

Each template has its own options, set under `[template.options]`:

```toml
[template]
name = "classic"
[template.options]
# classic takes no options
```

An option a template does not know, or a value it does not take, stops every command with the line in `site.toml` and the fix. For example:

> site.toml, line 12, field "lead_image": the classic template has no option lead_image; it takes no options, so remove the line

An option you leave out takes the template's default.

## The templates

| Template | For | Its page |
|---|---|---|
| `classic` | Today's look, every blog's default: a calm column of text, cards on the home page and the contents beside long posts. | This page |
| `folio` | A product team's essays under their own masthead: a featured story, outlined picture cards, share links and an "All posts" link on every essay. | [Folio](templates/folio.md) |

## Parts a template can switch on

Every template shares one set of page parts. A template switches on the ones it uses, and offers each as an option with its own default. Classic switches none of them on, so it looks as it always has. The words below are the option keys and the values each takes; a template lists only the parts it offers.

| Option | Values | What it changes |
|---|---|---|
| `logo` | `true`, `false` | The header shows the logo from `[brand] logo` in `theme/tokens.toml` in place of the blog's name. Without a logo, the name shows. |
| `back_link` | `true`, `false` | A post shows an "All posts" link back to the front page above its title, in place of breadcrumbs. |
| `breadcrumbs` | `true`, `false` | Each page shows the trail of links from the home page to it: home, the post's category, the page. Search engines read the same trail. |
| `dek` | `true`, `false` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `true`, `false` | An updated post shows its `updated_note` (why it was updated) under its dates. |
| `cards` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists: with a picture, text only, a small picture beside the text, title and date only, or a numbered list. |
| `card_reading_time` | `true`, `false` | Each card shows how long the post takes to read. |
| `card_author_role` | `true`, `false` | Each card shows its author's role after their name. |
| `category_tabs` | `true`, `false` | The home page shows a row of tabs: "All", then each category your posts use, in the order `categories.toml` lists them. |
| `featured` | `true`, `false` | The newest post with `featured: true` shows large above the home page and each list, and not again below. |
| `picks` | `0` to `8` | How many posts with `featured: true` the home page lists as editor's picks. `0` shows none. |
| `pagination` | `pages`, `numbered` | The links under a long list: newer and older, or a numbered link to each page near this one. |
| `related` | `grid`, `list`, `row`, `scroll`, `links` | How the related posts under a post are laid out. `scroll` is one row of pictures that scrolls sideways. |
| `related_count` | `3` to `10` | How many related posts show. |
| `contents` | `side-end`, `side-start`, `inline`, `inline-folded`, `none` | Where a long post's contents go: a column beside the post on wide screens (on the end or start side), an open box at the top, a folded box at the top, or nowhere. |
| `contents_numbers` | `numbered`, `nested`, `none` | How the contents are numbered: 1, 2, 3, or 1, 1.1, 2, or not at all. |

Every part reads with scripts off. Every page also shows `[brand] icon` in the browser tab, in every template.

## Options many templates share

Some parts of a page work the same in every template that has them. Each template lists the ones it takes, with its own defaults. Classic takes none of them, so it shows none of them.

| Option | Values | What it does |
|---|---|---|
| `share` | `none`, `top`, `end`, `both` | Where a post shows its share links: under the title, after the post, or both. |
| `share_networks` | a list from `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in the order they show. Each name once. |
| `newsletter` | `none`, `post`, `home`, `both` | Where the newsletter form shows: after each post, on the front page, or both. |
| `cta_band` | `none`, `post`, `home`, `both` | Where the band with your call to action shows. |
| `cta_button` | `true`, `false` | Whether the header shows your call to action as a button. |

Share links are plain links to each network's own share page. They need no script, load nothing from the network, and tell the network nothing until a reader follows one.

The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. A static blog has nowhere to send the form, so it shows none.

## Your call to action

A call to action is the one thing you ask readers to do, such as start a trial. Write it once:

```toml
[template.cta]
label = "Start a free trial"          # the words on the button
href = "https://example.com/signup"   # a full address, or a path on your blog such as /blog/start/
note = "No card needed."              # optional: one line beside the button
```

Templates show it as a band after a post or on the front page (`cta_band`), and as a button in the header (`cta_button`). Without `[template.cta]` neither shows.

To place the band inside a post or page, write `::cta` on a line of its own. It works in every template, classic too. Without `[template.cta]` the line shows nothing. Feeds and the Content API keep it as the comment `<!--khalid-cta-->`, so another front end can put its own band there.

## Footer links in groups

Besides the one row of `[[nav.footer]]` links, the footer can hold columns of links, each under its own heading. Every template shows them.

```toml
[[nav.footer_groups]]
heading = "Product"
[[nav.footer_groups.links]]
label = "Pricing"
href = "https://example.com/pricing/"
[[nav.footer_groups.links]]
label = "Docs"
href = "https://docs.example.com/"

[[nav.footer_groups]]
heading = "Rechtliches"
language = "de"            # optional: only pages in this language show the group
[[nav.footer_groups.links]]
label = "Impressum"
href = "/blog/de/impressum/"
```

A group needs at least one link. A link may also name a `language`, like any menu link.

## Reader features

A template may switch some `[reader]` features on or off by default, such as the reading time. A key you write in `[reader]` always wins over the template.

## Text sizes and spacing

`theme/tokens.toml` takes two more tables. Each value is optional; a value you leave out takes the template's default.

```toml
[type]
body = "1.0625rem 1.125rem"   # smallest and largest, or one size such as "1.125rem"
h1 = "2.25rem 3.25rem"
h2 = "1.5rem 2rem"
h3 = "1.25rem"
h4 = "1.125rem"
small = "0.9375rem"           # dates, captions and other small text
line_height = 1.65            # 1 to 2.5
heading_weight = 700          # 100 to 900, in steps of 100

[space]
unit = "0.5rem"               # the smallest gap
section = "3rem"              # the space between large parts of a page
```

Two sizes grow with the screen: the first on a phone 320 pixels wide, the second on a screen 1280 pixels wide and wider, and in between in step. Sizes are in `rem`, the browser's base text size.

`khalid theme sync` keeps both tables as you wrote them. A file in `theme/templates/` still replaces the template's file of the same name.
