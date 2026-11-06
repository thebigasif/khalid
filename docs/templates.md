---
title: "Templates: the look of your blog"
order: 30
---

# Templates: the look of your blog

A template sets the layout and features of your blog's pages: where the contents list sits, which cards the home page shows, and the sizes of text and space. Your colours, fonts and logo still come from your main site through theme sync, so every template looks like your brand.

Every template is built into Khalid. You pick one in `site.toml` and adjust that template's options. You cannot mix parts of different templates.

## Picking a template

```toml
[template]
name = "classic"
```

Without a `[template]` table your blog uses `classic`, Khalid's default look. Updating Khalid never switches your template to another one, but a release may restyle a template.

These templates are built in. Every template but classic has its own page, linked below, which says who it suits, what each page shows, and every option it takes with its default.

- `classic`: a wide frame with a large lead story and a grid of cards on list pages, a readable column for posts and the contents beside long posts. It takes no options.
- `almanac`: a library of long guides set like a field guide, with numbered sections, a contents column beside each post, a featured guide, editor's picks and a call to action that opens and closes the front page. See [Almanac](templates/almanac.md).
- `alto`: a clean, minimal blog with a sideways strip of featured posts, then one post under another with a wide picture, and a calm column for each post. Based on Ghost's Alto theme (MIT). See [Alto](templates/alto.md).
- `aperture`: one photographer's site, with each post one picture the full width of the page, its story and a plate of its camera, lens and exposure, and older pictures in a light contact sheet. See [Aperture](templates/aperture.md).
- `atelier`: a fashion label, stylist or fashion writer, with a lookbook of numbered looks, each a tall picture with the pieces worn beside or under it, collections, a journal of notes and posts that open as a two-page spread. See [Atelier](templates/atelier.md).
- `atlas`: a publication for a blog with many topics, with a map-sheet masthead that indexes every topic, featured stories and picture cards. See [Atlas](templates/atlas.md).
- `beacon`: a charity or a cause, with campaigns that each carry a goal meter, impact figures, a news feed and a donate call in the header, the hero and a band on every page. See [Beacon](templates/beacon.md).
- `bulletin`: a newsletter-first blog with a sign-up panel in your accent colour beside the latest issue, a ruled list of every other issue and a centred reading column. Based on Ghost's Bulletin theme (MIT). See [Bulletin](templates/bulletin.md).
- `casper`: a classic blog with a cover band over a picture, one large card, two half cards and then a grid, and a calm serif column for each post. Based on Ghost's Casper theme (MIT). See [Casper](templates/casper.md).
- `chapter`: an author's site, with the newest book beside its cover, the books as a shelf of covers, excerpts and essays as a book's contents page, events as dated rows and excerpts set as a page of the book. See [Chapter](templates/chapter.md).
- `casper`: a classic blog with a coloured cover band and a picture under it, one large card, two half cards and then a grid, and a calm serif column for each post. Based on Ghost's Casper theme (MIT). See [Casper](templates/casper.md).
- `channel`: a video creator's home, with the newest video large as Now showing, thumbnail cards with a play bar and running time, a shelf for each playlist, and posts that open on a wide video with Up next under them. See [Channel](templates/channel.md).
- `compass`: a teaching blog where every guide opens with its trail, contents and key points. See [Compass](templates/compass.md).
- `dawn`: a calm, text-led blog and newsletter with a small centred cover and sign-up, a row of featured posts, then every post as one line with a day and month block. Based on Ghost's Dawn theme (MIT). See [Dawn](templates/dawn.md).
- `digest`: a compact newsletter blog that leads with the latest issue, then a subscribe box, featured issues in soft grey panels and a ruled list of past issues. Based on Ghost's Digest theme (MIT). See [Digest](templates/digest.md).
- `dope`: a blog arranged by topic, with a sideways shelf of tall topic tiles on the front page, wide rows with a large picture beside each summary, and posts that open on their picture from edge to edge. Based on Ghost's Dope theme (MIT). See [Dope](templates/dope.md).
- `ease`: a help centre or documentation site, with a large search box on a band in your accent colour, a strip of featured articles, then each category as a list of links, two across. Based on Ghost's Ease theme (MIT). See [Ease](templates/ease.md).
- `edge`: a picture-led site for creative work, with a wall of lead pictures packed like masonry on the front page and posts that open with a wide picture over one calm column. Based on Ghost's Edge theme (MIT). See [Edge](templates/edge.md).
- `edition`: a newsletter with a big cover, a picture from edge to edge with your pitch and the sign-up form, then a row of featured posts and a quiet list of the latest. Based on Ghost's Edition theme (MIT). See [Edition](templates/edition.md).
- `encore`: a musician's or a band's site, with the newest release on its sleeve, tour dates with ticket links, a shelf of every record and the band's notes from the studio and the road. See [Encore](templates/encore.md).
- `episode`: a podcast's blog that leads with the newest episode and its player, under a dark band with the show's name and where to listen, then every episode as a boxed row with a square picture. Based on Ghost's Episode theme (MIT). See [Episode](templates/episode.md).
- `fixture`: a sports club, with the next match and the club's form on the front page, results as scoreboard rows with the score in large figures, upcoming fixtures, club news, a squad page and a scoreboard on every match report. See [Fixture](templates/fixture.md).
- `folio`: a product team's notebook under a large masthead on ruled paper, with category tabs, a featured story, picture cards, and notes with a contents rail, framed code and an author box. See [Folio](templates/folio.md).
- `harbor`: a food and recipe blog with a featured post beside smaller posts, a strip of recipe cards, a grid of picture cards, an offer in each post and a sideways row of related reading. See [Harbor](templates/harbor.md).
- `headline`: a local news site with the logo over the menu, the latest stories, a section for each topic with a lead story and small rows, short lists of headlines for more topics, and a dark footer. Based on Ghost's Headline theme (MIT). See [Headline](templates/headline.md).
- `journal`: one writer's newsletter, with the latest issue in large type, every other issue as text beside a side column about the writer, and a sign-up band in your accent colour. Based on Ghost's Journal theme (MIT). See [Journal](templates/journal.md).
- `hub`: a creator's link-in-bio home in one narrow column, with the creator's photo and name, a stack of large link buttons led by the call to action, and the latest posts as rows with a square picture. See [Hub](templates/hub.md).
- `itinerary`: a travel diary with trips filed by country as passport stamps, each trip a ticket with its days, route, season and cost, a day-by-day route on every trip and checklists for packing. See [Itinerary](templates/itinerary.md).
- `keystone`: a calm product or company blog with a lead story, a grid of framed pictures and a quiet reading column. See [Keystone](templates/keystone.md).
- `lattice`: a science explainer site, with every picture a numbered figure, key numbers from each post's facts, footnotes as numbered tabs and marked glossary terms. See [Lattice](templates/lattice.md).
- `ledger`: a calm, text-led blog for studies and long how-tos, with a masthead and ruled lists. See [Ledger](templates/ledger.md).
- `london`: a picture-led blog or portfolio with the blog's description set large in the middle, tall picture tiles from edge to edge, two side by side and every third across the whole width, and a centred reading column. Based on Ghost's London theme (MIT). See [London](templates/london.md).
- `locale`: a city guide with places listed by area, each as a guidebook row with its kind, opening hours and a price meter, an At a glance card on every place, and notes from the city. See [Locale](templates/locale.md).
- `meridian`: a calm company journal with category tabs, wide rows and a reference layout for long guides. See [Meridian](templates/meridian.md).
- `reel`: film and TV reviews, with a star rating on a ticket, the cast and where to watch on every review, poster cards and a score sheet ranked by rating. See [Reel](templates/reel.md).
- `plinth`: an architecture or interiors practice, with each project as a spread of its photograph beside a title block of its facts, plans framed as numbered drawing sheets, studio notes and types of work. See [Plinth](templates/plinth.md).
- `research`: a research lab or academic group, with papers as a numbered reference list with venue, DOI and citations, a news feed, research areas, a people page and a cite box on every paper. See [Research](templates/research.md).
- `ruby`: a magazine of picture cards, four white cards across on a page that fades to light grey, and a calm column for each post with more cards after it. Based on Ghost's Ruby theme (MIT). See [Ruby](templates/ruby.md).
- `respawn`: a games site with news, reviews and a release calendar, where every review shows its score as a dark square and a health bar of ten cells, on cards, lists and the review itself. See [Respawn](templates/respawn.md).
- `solo`: a personal site for one writer or creator, with a large headline and a sign-up field over a feed of posts with large titles. Based on Ghost's Solo theme (MIT). See [Solo](templates/solo.md).
- `source`: a general publication with the logo in the middle of the header, a front page that opens with a landing, highlight or magazine header, a list or a grid of posts with hairlines between them, and a calm column for each post. Based on Ghost's Source theme (MIT). See [Source](templates/source.md).
- `wave`: an interview show's blog on a quiet white page, with the newest episode in a slim player card, then every episode as a row with square artwork and a round play mark. Based on Ghost's Wave theme (MIT). See [Wave](templates/wave.md).
- `taste`: a baking blog with a centred header and sign-up field, a magazine block of five posts and a picture grid for each main topic. Based on Ghost's Taste theme (MIT). See [Taste](templates/taste.md).
- `ticker`: a markets newsletter, with a dark tape of figures, each issue's moves as a bar chart drawn without script, and every list as a data table shaded by how far each figure moved. See [Ticker](templates/ticker.md).
- `summit`: a conference, festival or event, with the programme as a timetable by day and time, speakers, venues with their sessions and a ticket stub on every session. See [Summit](templates/summit.md).
- `syllabus`: an online course, with a course map of one square per lesson, an outline of modules and lessons, course news, and on every lesson a progress ruler that shows how far through the course it is. See [Syllabus](templates/syllabus.md).
- `vital`: a health and wellness site of programmes and routines, with each routine's time in a ring on its picture card, a session panel and a care note on every routine, its steps drawn as a track, and a notes feed. See [Vital](templates/vital.md).
- `versions`: a software product's changelog, with every change on a version rail, a filter row for each type of change (new, improved, fixed) that works without a script, the versions as anchor links and a box to get updates by email or feed. See [Versions](templates/versions.md).
- `wire`: a news and columns site with a lead story beside the latest headlines, a dated feed and the editor's picks in a rail. See [Wire](templates/wire.md).
- `workbench`: a blog that reviews and compares tools, with a ruled, exact look and wide comparison tables. See [Workbench](templates/workbench.md).

## Starting a blog in a template

To start a new blog in the template you liked in its demo, name it when you make the site:

```text
khalid init blog --address https://blog.example.com --template atlas --yes
```

`--template` takes any name from the list above. It writes the `[template]` table into the new `site.toml`, and starts `theme/tokens.toml` with that template's own colours, text sizes and spacing, so the new blog looks like the demo. `khalid theme sync` or a hand edit changes them later, as on any site.

A name Khalid does not know stops `init` before it makes anything, and the message lists every name. Without `--template`, the blog starts in `classic`.

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

## Picking a template in the admin

The admin's **Look** screen does the same without opening `site.toml`. It lists every template your Khalid has, each with one sentence on what it is for and what it turns on and off. Pick one, change the options you want different, and press **Save the look**.

Khalid checks the choice as it checks `site.toml` at start, so a template or value it would refuse is never saved. It writes only the options that differ from the template's defaults, and keeps your comments and `[template.cta]`. Your blog shows the new look after Khalid restarts.

The screen reads two Admin API routes, which need `settings:write`:

- `GET /api/admin/v1/templates/` lists the templates, each with its options: the values each takes and its default.
- `PUT /api/admin/v1/settings/` saves the choice as `template`, with its `name` and `options`, beside the other settings.

## Parts a template can switch on

Every template shares one set of page parts. A template switches on the ones it uses, and offers each as an option with its own default. Classic switches none of them on but the reading progress bar. The words below are the option keys and the values each takes; a template lists only the parts it offers.

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
| `reading_progress` | `bar`, `none` | A thin accent line at the top of a post that fills as the reader scrolls. Every template shows it, classic too; `none` turns it off in any template but classic, which takes no options. It hides when the reader asks for less motion, and in print. |
| `header_search` | `button`, `link`, `none` | What the search in the header does on a blog that answers searches (`mode = "server"`): `button` opens a search dialog that shows results as the reader types, and the search page with scripts off; `link` always opens the search page; `none` hides it. Classic uses `button`. A static blog has no search, so its header shows none. |
| `hero_stats` | up to four lines | Numbers under the home page's title, each written `number \| what it counts`, such as `"140 \| blogs studied"`. Khalid never makes one up: with no lines, nothing shows. |
| `topic_tabs` | `true`, `false` | The home page shows tabs, one per topic, each with that topic's four newest posts. Each tab is a link to its panel, so the tabs work with scripts off. |
| `video_section` | `true`, `false` | The home page shows a row of the newest posts with a video, when there are any. |
| `podcast_section` | `true`, `false` | The home page shows a row of the newest posts with a podcast or other sound player, when there are any. |

What the reading progress bar, the contents sheet, the copy link button and the header search do for a reader is in [Reading aids](reading.md).

Every part reads with scripts off. Every page also shows `[brand] icon` in the browser tab, in every template.

## Facts on a post

A post can list a few short facts in its front matter. A template can show them on the post's cards and on its page. A review might list its scores, a project its client and year, and a recipe its time and servings.

```yaml
---
title: A reading room for Calder Street
facts:
  - label: Client
    value: Calder Street Library
  - label: Year
    value: "2025"
---
```

- A post lists at most four facts, in the order they should show.
- A label has 1 to 24 characters. A value has 1 to 40 characters.
- Each label and value is one line of plain text. Spaces at either end are dropped.
- Put a label or value in quotes when it holds a colon, or to keep a number exactly as written, such as `"2.50"`.
- An empty list is the same as no facts.

`khalid build` stops and names the line when a fact breaks one of these rules.

These templates show facts. Each reads them by their place in the list, not by their label, so the labels can be in any language:
Several templates show facts. Each reads them by their place in the list, not by their label, so the labels can be in any language:
Five templates show facts. Each reads them by their place in the list, not by their label, so the labels can be in any language:

- [Fixture](templates/fixture.md#matches-are-posts-with-facts) treats posts with facts as matches: the opponent, the result or kick-off, the ground and the competition. Results show as scoreboard rows with a W, D or L chip, the front page shows the next match and the club's form, and a match report opens with a scoreboard.
- [Aperture](templates/aperture.md#camera-details-are-facts) shows the facts as a plate of camera details under each picture: the camera, the lens, the exposure and the date it was taken.
- [Folio](templates/folio.md#release-notes) treats posts with facts as releases. The front page lists them in a dated release log beside the essays, tagged with the first fact, such as the version. A release's page shows its facts above the title.
- [Workbench](templates/workbench.md#score-panels) shows a score panel on review cards and at the top of a review: the first fact large, with a meter when it is a score out of ten, then up to three figures.
- [Keystone](templates/keystone.md) lists posts with facts in a project index on the front page, one column per fact, and shows the facts as a credits line on each post.
- [Ticker](templates/ticker.md#issues-are-posts-with-facts) treats posts with facts as issues, each fact a market and its move. It draws a post's facts as a bar chart and lists posts in a table with one column per fact, each figure cell shaded by how far it moved.
- [Lattice](templates/lattice.md#what-you-write) shows a post's facts as a row of key numbers under its lead picture, the value large and the label under it, and does the same for the lead explainer on the front page.
- [Versions](templates/versions.md#changes-are-posts) reads the first fact as the version a change shipped in, and groups the changes of each version on its version rail. A change's page shows every fact in a ruled row.
- [Harbor](templates/harbor.md#recipe-facts) shows up to four posts with facts in a recipe card strip on the front page, with the first two facts as tags, and all the facts as tags under a post's title.
- [Respawn](templates/respawn.md#reviews-are-posts-with-facts) treats posts with facts as reviews. The first fact is the score out of 10, shown as a dark square and a health bar on cards, lists and the review, and the second, the platforms, shows under it.
- [Locale](templates/locale.md#places-are-posts-with-facts) treats posts with facts as places: 1 the kind of place, 2 the price level, drawn as a price meter of four marks, 3 the opening hours and 4 the address. They show on listing rows and in an At a glance card on each place.

The other templates do not show facts. A custom template that wants them reads `post.facts` on a card and `page.facts` on a post page. Each fact has a `label` and a `value`:

```html
{% for fact in page.facts %}<dt>{{ fact.label }}</dt><dd>{{ fact.value }}</dd>{% endfor %}
```

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

## A preferred source on Google

Google lets each reader pick news sources it shows more often in Top stories. To ask your readers, turn on the button:

```toml
[template.preferred_source]
place = "both"   # optional: post after each post, footer on every page, or both (the default)
```

The button reads "Add us as a preferred source on Google" and opens Google's page for your blog's domain. Every template shows it, classic too. It is a plain link: no script, no Google logo, and nothing loads from Google until a reader follows it. Without the table it shows nowhere.

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

## Your own CSS rules

A file named `theme/site.css` adds your own rules to the blog's one stylesheet, after the template's rules, so yours win. Use it for a few touches the tokens cannot make, such as a home page of your own in `theme/templates/home.html`.

```css
.hero { padding-block: 4rem; background: var(--color-surface); }
```

- Read the tokens as CSS variables, such as `var(--color-accent)` and `var(--space)`, so `khalid theme sync` still sets your colours and fonts.
- Comments and blank lines are removed when the site is built.
- The whole stylesheet must stay small: Khalid's own pages keep it under 20 KB once compressed.
- The file must sit inside the site folder. A link to a file elsewhere stops the build with a message.

## Checking every screen size

`scripts/responsive-check.mjs` opens each template's home page, `/seo-writing-guide/`, a category page and the German home in a real browser, at 320, 360, 375, 390 and 414 pixels (phones), 768 and 1024 (tablets), and 1280, 1440, 1920 and 2560 (desktops, up to a 27 inch iMac). It fails when a page scrolls sideways, when reading text on a phone is under 15 pixels or a menu link is under 40 by 40, and when on a wide screen the reading column passes about 80 characters, the content runs edge to edge or sits off centre, or a picture is drawn larger than its own pixels. It saves a screenshot of each page at each width, so you can look for what a script cannot see.

It needs Node and Playwright. Build the sites, then run the script from a folder where Playwright is installed:

```sh
scripts/pages.sh target/debug/khalid target/pages
cd ~/pw && node ~/khalid/scripts/responsive-check.mjs --dir ~/khalid/target/pages --out ~/shots
```

Add `--templates harbor` to check one template, and `--widths 390,2560@2` to pick widths; `2560@2` is a 5120 pixel iMac screen at device scale 2. For the admin, start `khalid serve`, save the link `khalid staff owner` prints to a file, and pass `--admin http://localhost:8080/blog/admin/ --sign-in-file <that file>`; it checks sign-in, the dashboard, posts, the editor, media and Look. Delete the file afterwards, since the link signs in as the owner.

## Checking that nothing covers anything

`just layout-check` builds the showcase site once with every template, and once more with the contents list in a side column for each template that puts it elsewhere. It opens a home page, a post with a lead picture, a contents list and pros and cons, a page with a folded table, a category, a tag, an author, search and a glossary page, in Chromium, Firefox and WebKit at 390, 768, 1440 and 2000 pixels. Each template wears its sample brand from `fixtures/showcase-brands/<name>/theme/` when it has one, as its demo on GitHub Pages does. It scrolls each page to the end and fails when:

- the letters of one piece of text cover other text or a picture by more than 4 pixels;
- a list marker covers the first letter of its item, or other text;
- the page scrolls sideways;
- a table column is narrower than its longest word, so the word breaks in the middle;
- a picture did not load;
- a person's photo is not drawn square, to within 1 pixel, so its circle turns into an oval;
- at 1440 pixels and wider, the site header, or the main part of the home page or a category or tag page, is narrower than the template's minimum frame, the width its design brief sets, kept as `frame` in `crates/khalid-theme/packs/<name>/pack.toml`;
- at 1440 pixels and wider, a post's body text is smaller than 18 pixels, or a full line of it holds fewer than 60 or more than 75 characters;
- a stylesheet the home page links is 20 KB or more once compressed with Brotli.

Parts that stay on screen as you scroll, such as the floating contents button and the side contents column, are checked against what passes under them. Each failure names the template, the page, the browser, the width, the two elements and where on the page they meet. The Demo check runs it in CI. To run it yourself, install the tools and the browsers once:

```sh
npm install --prefix scripts/demo-check
npx --prefix scripts/demo-check playwright install --with-deps chromium firefox webkit
just layout-check --browsers chromium,firefox --packs keystone
```

`--packs` picks templates and `--widths` picks widths. A screenshot of each failing page goes to `target/layout-check/shots`, and every failure to `target/layout-check/report.json`. `--frames <file>` reads the minimum frames from another file, to try a stricter one. `just layout-selftest` proves the frame, reading and stylesheet checks on two small sites with a planted fault each.

A template whose demo has a ready publication of its own, in `fixtures/showcase-publications/<name>/`, is checked a second time with that publication in place of the showcase's posts, on the pages its `check-pages.txt` names. The showcase pass stays as it is for every template, so its long tables, glossary and long lists remain the floor. `just publication-selftest` proves the publication step and the second pass on a small sample publication, with no browser. `fixtures/showcase-publications/README.md` sets out what a publication holds.

## Pictures for review

A template's design is approved from pictures, not from the checks alone. `just demo-pictures --out <folder>` builds the showcase with each template twice, in its sample brand and with the template's own defaults and no theme, and takes full-page pictures of the home page, the post `/content-benchmark/` and `/category/content-strategy/` at 2000, 1440 and 390 pixels. Each template gets a sheet, `sheet.html` and `sheet.png`, with the branded demo, the default and a reference picture side by side for each page and width. Add `--refs <folder>` to place stored reference pictures: one folder per template, each file named such as `<source>-<page>-<width>.png`, where the page is `home`, `post` or `list`. Keep `--out` outside the repository, since the pictures are never committed. A template with a demo publication, ready or not, is pictured with it, on the home page and the first post and category its `check-pages.txt` names.
