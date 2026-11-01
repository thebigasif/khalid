# Compass

Compass is a template for a teaching blog: guides that explain how to do something, step by step. Every guide opens with where it sits on your site, what it covers and its key points, so a reader knows in seconds whether it answers their question. It ends with share links, the author and the newsletter.

## Who it suits

- **It suits** a company or a team that teaches its subject: a software product's guides, an agency's know-how, a course provider's free lessons.
- **It suits** posts with several main headings, a short "key takeaways" list at the top, screenshots and a FAQ.
- **It suits less well** a personal brand built on one writer's name and photos, or a news site that lives on its newest stories. Keystone fits a personal brand better.

## What it shows

**The front page**

1. Your logo in the header, and your call to action as a button when `[template.cta]` is set.
2. The blog's title, large, on a tinted band across the window.
3. When the page has two or more home sections, a row of links that jump to each one.
4. Each home section as three picture cards (author, date, title, summary), ending in a link to the rest. Without home sections, your newest posts under "Recently published", with numbered page links. A post with no picture gets a drawn cover in tints of your accent colour, so the cards line up.
5. The newsletter as a band before the footer, when your blog takes sign-ups.

Lists such as a category or the archive show the same outlined picture cards.

**A guide (a post)**

1. Breadcrumbs: a house for your blog's home, the post's category, then the post.
2. The title, the author's name (without a photo) and the date it was published, and updated if it was.
3. The lead picture.
4. The contents: an open box listing the main headings, without numbers.
5. The post. Write a summary callout first and it sits right after the contents as the key points, each with a drawn check mark:

   ```markdown
   :::callout{kind=summary}
   - A sitemap helps search engines find pages.
   - It does not make a page rank.
   :::
   ```

6. Share links between two thin rules.
7. The author box: the author's round photo, name, role and short bio.
8. The newsletter, set off by rules, when your blog takes sign-ups, and the band with your call to action when `[template.cta]` is set.

Compass shows no reading time and no related posts. Each can come back: see "Turning features back on" below.

**In the body**, read callouts (`:::callout{kind=read}`) sit on a tint of your accent colour, tips on a soft grey box, quotes in your heading font, screenshots with a thin frame, a banner block centred on a tint, and FAQ questions open with a plus.

Everything reads with scripts off. Compass adds no script.

## Options

Set these under `[template.options]` in `site.toml`. An option you leave out takes the default.

| Option | Default | Values | What it changes |
|---|---|---|---|
| `logo` | `true` | `true`, `false` | The header shows the logo from your main site in place of the blog's name. Without a logo, the name shows. |
| `cta_button` | `true` | `true`, `false` | The header shows your `[template.cta]` as a button. |
| `breadcrumbs` | `true` | `true`, `false` | Each page shows the trail of links from the home page to it. |
| `cards` | `image` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists. |
| `pagination` | `numbered` | `pages`, `numbered` | The links under a long list: newer and older, or a link to each page. |
| `contents` | `inline` | `inline`, `inline-folded`, `side-end`, `side-start`, `none` | Where a long post's contents go. `inline` is the open box at the top. |
| `contents_numbers` | `none` | `none`, `numbered`, `nested` | How the contents are numbered: not at all, 1, 2, 3, or 1, 1.1, 2. |
| `share` | `end` | `none`, `top`, `end`, `both` | Where a post shows its share links. |
| `share_networks` | `["linkedin", "x", "facebook", "email"]` | any of `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in order. |
| `newsletter` | `both` | `none`, `post`, `home`, `both` | Where the newsletter form shows. It shows only when your blog takes sign-ups. |
| `cta_band` | `post` | `none`, `post`, `home`, `both` | Where a band with your `[template.cta]` shows. |

An example:

```toml
[template]
name = "compass"

[template.options]
contents_numbers = "numbered"
share_networks = ["linkedin", "email"]

[template.cta]
label = "Try it free"
href = "https://example.com/signup"
note = "Put these guides to work in your own account."
```

## Turning features back on

Compass switches off three reader features. A key you write in `[reader]` wins over the template:

```toml
[reader]
byline = true           # the author's photo and role at the top of a post
related_posts = true    # up to five related posts after each post
reading_time = true     # "8 min read" beside the date
```

## Sizes and spacing

Your own `theme/tokens.toml` wins over each value.

| Token | Compass |
|---|---|
| Body text | 1.0625rem on a phone to 1.125rem on a wide screen |
| Main title (`h1`) | 2rem to 3rem |
| Main headings (`h2`) | 1.5rem to 2rem |
| Headings (`h3`) | 1.25rem |
| Small headings (`h4`) | 1.125rem |
| Line height | 1.7 |
| Space between sections | 3.5rem |
| Corners | 0.5rem |
| Line length | 68 characters |

Compass brings its own colours and heading type: a teal accent (`#0e7490`) and rounded headings where the reader's system has them. Your colours, fonts and logo from your main site, through theme sync, replace them, as in every template.
