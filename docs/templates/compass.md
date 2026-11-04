# Compass

Compass is a template for a teaching blog: guides that explain how to do something, step by step. Every guide opens with where it sits on your site, what it covers and its key points, so a reader knows in seconds whether it answers their question. It ends with share links, the author and the newsletter.

## Who it suits

- **It suits** a company or a team that teaches its subject: a software product's guides, an agency's know-how, a course provider's free lessons.
- **It suits** posts with several main headings, a short "key takeaways" list at the top, screenshots and a FAQ.
- **It suits less well** a personal brand built on one writer's name and photos, or a news site that lives on its newest stories. Keystone fits a personal brand better.

## What it shows

**The front page**

1. Your logo in the header, and your call to action as a button when `[template.cta]` is set. On a phone the button waits for the band further down, so the header stays short.
2. A masthead that fills the top of the screen: the blog's title, very large, on a pale band drawn as a map grid that runs to the edges of the window, with a drawn compass in your accent colour. Under the title, every topic of your blog as a numbered stop (01, 02, 03) on one dashed trail, each a link to its topic. On a phone the trail runs down the screen.
3. When the page has two or more home sections, a row of links that jump to each one.
4. Each home section as three picture cards (topic, title, summary, then author and date), ending in a link to the rest. Without home sections, your newest posts under "Recently published", with numbered page links. In a list of four or more, the newest post leads across the whole row, its picture beside a large title. A post with no picture gets a drawn cover in tints of your accent colour, so the cards line up. In a long list, after the lead and six picture cards, the rest is a numbered index: one ruled row per post with its number, topic, title and date, and no picture.
5. Topic tabs as rounded pills, the open one filled in your accent colour, then the video and podcast rows.
6. The newsletter as a band before the footer, when your blog takes sign-ups.

Lists such as a category, the archive or an author's posts show the same cards and index under the same map band.

**A guide (a post)**

1. Breadcrumbs: your blog's home, the post's category, then the post, centred on a pale band that runs to the edges of the window.
2. The title, large and centred on the same band, then one quiet line with the author's name (without a photo), the reviewer, the dates and the category.
3. The lead picture, wider than the text on a large screen, with the band stopping halfway down it.
4. The contents: an open box listing the main headings as a numbered route (01, 02, 03), each number in a small circle, in two columns when there is room. A `::toc` block in the post repeats it, so Compass shows that block only when the box is off.
5. The post. Write a summary callout first and it sits right after the contents as the key points, each with a drawn check mark:

   ```markdown
   :::callout{kind=summary}
   - A sitemap helps search engines find pages.
   - It does not make a page rank.
   :::
   ```

6. Share links as round buttons between two thin rules.
7. The author box: the author's round photo, name, role and short bio.
8. The newsletter on a pale panel when your blog takes sign-ups, and the band with your call to action when `[template.cta]` is set.
9. The posts that link to this one, in two columns.

Compass shows no reading time and no related posts. Each can come back: see "Turning features back on" below.

**In the body**, how-to steps (`::::howto`) are numbered stops joined by a line. Each callout has its own drawn mark: a bulb for a tip, an "i" for a note, a triangle for a warning, an arrow for a read callout. A stat callout shows its number large. Quotes sit under a drawn quote mark in your heading font, screenshots and figures sit on a pale mat with a thin frame and the caption under them, tables fill the column with a pale header row, and FAQ questions open with a plus.

Everything reads with scripts off. Compass adds no script.

## Options

Set these under `[template.options]` in `site.toml`. An option you leave out takes the default.

| Option | Default | Values | What it changes |
|---|---|---|---|
| `hero_stats` | none | up to four lines | Numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `true` | `true`, `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | `true`, `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | `true`, `false` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `logo` | `true` | `true`, `false` | The header shows the logo from your main site in place of the blog's name. Without a logo, the name shows. |
| `cta_button` | `true` | `true`, `false` | The header shows your `[template.cta]` as a button. |
| `breadcrumbs` | `true` | `true`, `false` | Each page shows the trail of links from the home page to it. |
| `cards` | `image` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists. |
| `pagination` | `numbered` | `pages`, `numbered` | The links under a long list: newer and older, or a link to each page. |
| `contents` | `inline` | `inline`, `inline-folded`, `side-end`, `side-start`, `none` | Where a long post's contents go. `inline` is the open box at the top. |
| `reading_progress` | `bar` | `bar`, `none` | A thin line at the top of a post that fills as the reader scrolls. |
| `header_search` | `button` | `button`, `link`, `none` | What the header's search does on a served blog: open a search dialog, open the search page, or nothing. |
| `contents_numbers` | `numbered` | `none`, `numbered`, `nested` | How the contents are numbered: not at all, as a route (01, 02, 03), or 1, 1.1, 2. |
| `share` | `end` | `none`, `top`, `end`, `both` | Where a post shows its share links. |
| `share_networks` | `["linkedin", "x", "facebook", "email"]` | any of `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in order. |
| `newsletter` | `both` | `none`, `post`, `home`, `both` | Where the newsletter form shows. It shows only when your blog takes sign-ups. |
| `cta_band` | `post` | `none`, `post`, `home`, `both` | Where a band with your `[template.cta]` shows. |

An example:

```toml
[template]
name = "compass"

[template.options]
contents_numbers = "none"
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
| Body text | 1.0625rem on a phone to 1.1875rem on a wide screen |
| Main title (`h1`) | 2.5rem to 4rem; a guide's title 4.5rem and the front page's title up to 6.75rem on a wide screen |
| Main headings (`h2`) | 1.625rem to 2.125rem |
| Headings (`h3`) | 1.25rem to 1.5rem |
| Small headings (`h4`) | 1.125rem |
| Line height | 1.7 |
| Space between sections | 4.5rem |
| Corners | 0.75rem |
| Text column | 36rem, about 70 characters |
| Lists and header | 80rem, and 88rem on a screen 1760 px or wider |

Compass brings its own colours and type: a teal accent (`#0e7490`), a near-black ink with a hint of teal, a pale cool surface, heavy rounded headings where the reader's system has them, and the system's own sans for the text. Lines are hairlines and boxes are tints; there is no dark section anywhere. Your colours, fonts and logo from your main site, through theme sync, replace them, as in every template.

## Design

This is Compass's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A team that teaches its subject in guides. Readers want to know in seconds whether a guide answers their question, then follow it step by step.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A large SEO software company's free course pages. Courses as big cards in two staggered columns: a bold colour picture, a large title, one or two lines on what you learn, then a small button with the number of lessons and the time it takes. Plenty of white space. Its dark header band is not part of this brief.
- An SEO expert's library of long guides. Very large titles, chapters as numbered cards, key points in designed panels, and drawn diagrams instead of photos.

| Part | Compass |
|---|---|
| Type pairing | A rounded, heavy sans for headings over a sans drawn for easy reading. With no theme, the pack uses the system's rounded face for headings (`ui-rounded`) and the system sans for text. |
| Colour | White ground, deep blue-black text and a clear sky or teal accent. Pale tints of the accent for panels: the masthead map, key points and steps. |
| Spacing rhythm | A base step of 0.5rem. Generous: 1.5rem inside panels, 4.5rem between sections. Steps sit 2rem apart so each reads as one stop. |
| Type scale | Bold and large: the front page's title 3rem on a phone to 6.75rem, tight tracking (-0.045em) on every display title, card titles heavy (800). The type carries the idea: a map title, then numbered stops. |
| Frame at 1440 px and wider | At least **76rem** for the front page and lists: 80rem, and 88rem from 1760 px. A guide's text keeps its own reading column inside it, under a wider title. |
| Cards and lists | Rounded picture cards on a white ground with a thin border, the topic in small capitals, the title heavy in the heading font, two or three lines of summary. No shadows. The newest post leads with a large title. A long list turns into a numbered index after seven posts. |
| Phone header and menu | The logo on the left and the menu on the right in one short row; the menu scrolls sideways and fades at its edge. The call to action waits for the band further down. |
| Tabs and indexes at 390 px | Topic tabs become rounded pills that wrap onto two rows at most. The numbered contents box stays open in one column, with each number in a small circle on the pale box. The topic trail in the masthead runs down the screen. |
| Link, card and focus states | Links underlined in the accent; on hover the underline becomes a pale accent highlight. A card's border turns the accent on hover. Focus is a 3 px accent ring with rounded corners. |
| Signature details | The masthead as a full-width pale map grid with the drawn compass, and the blog's topics as numbered stops on a dashed trail. The contents as a numbered route (01, 02, 03) in two columns. Key points with drawn check marks, how-to steps as numbered stops joined by a line, and a long list as a numbered index. |

**Sample brand.** The demo uses `fixtures/showcase-brands/compass/theme/`: sky blue (`#03679d` links, `#0277b6` accent) with pale blue panels (`#eef7fc`), Nunito for headings and Atkinson Hyperlegible for text, and a rising-trail mark.
