---
title: "Pantry"
order: 52
---

# Pantry

Pantry is a template for a baking and recipe blog. Every recipe shows its time and what it makes on a tear-off ticket, and every recipe page opens on its facts and its ingredients, before the story. It feels like a tidy recipe box on a floury worktop: square index cards, ruled lines and clear weights.

```toml
[template]
name = "pantry"
```

## Who it suits

- A home baker who writes tested recipes for bread, cakes, biscuits and pastry, with weights and oven settings.
- A small bakery or cookery school that shares its recipes and a few baking notes.
- A recipe writer who wants readers to find the ingredients at once.

It suits a busy cooking blog that grows a mailing list less well. For that, Harbor reads better.

## Recipes are posts with facts

Pantry needs nothing new in your posts. A post with `facts` is a recipe. A post without facts is a baking note, such as a kitchen tip. Put the facts in this order, since Pantry reads them by their place, not their label:

```yaml
---
title: "Country sourdough loaf for a first bake"
description: "A country sourdough loaf with a crisp crust, made over two days."
facts:
  - label: Time
    value: "2 days"
  - label: Makes
    value: "1 loaf"
  - label: Oven
    value: "250°C, then 230°C"
  - label: Level
    value: "Steady"
---
```

1. **The total time,** such as `1 hour 15 min`. It shows on every card.
2. **What it makes,** such as `12 buns` or `8 slices`. It shows on every card.
3. **The oven setting,** such as `180°C`.
4. **The level,** such as Easy or Steady.

Your labels show on the ticket, so write what fits. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

## Write the ingredients first

Pantry draws a recipe's own lists. Write them in this order:

1. A heading such as "Ingredients", then the ingredients as a bullet list. Every bullet list before the first numbered list is drawn as the ingredients card: a kraft panel under a cherry line, each ingredient on a ruled line with a small square to tick. For groups, put a small heading such as "For the dough" over each list.
2. A heading such as "Method", then the steps as a numbered list. The first numbered list is drawn as the method: each step's number large in a square, with a rule between steps.

Lists after the method, such as tips, stay plain lists. A baking note has no ticket, and its lists stay plain.

Categories are the sections of your recipe box, such as Bread, Cakes and Biscuits. Give each an intro in `categories.toml`, and it shows at the top of that section's page. Tags are what a recipe is, such as `yeast` or `one bowl`.

## What the front page shows

1. Your site's name, large, and the `tagline` under it.
2. Up to four large numbers from `hero_stats`, such as how many recipes you have. Nothing shows until you write them.
3. **The recipe box:** each category that holds recipes as a divider tab along a cherry line. A category whose posts on the page are all baking notes is left out.
4. **Fresh from the oven:** your newest recipe as one wide index card: its picture beside its section, title, summary and its full ticket of four facts. It shows on the first page only.
5. **Recipes:** your other recipes as square index cards. Each shows its picture, its section, its title and its ticket with the time and what it makes.
6. **Baking notes:** the newest posts without facts as a dated list. `notes_count` sets how many.
7. Numbered page links, your call to action as a band, and the newsletter form.

## What a post shows

1. The trail from the home page to the post's section.
2. The section as a small label, the title, the post's description, the baker with their mark and role, and the date.
3. **The ticket:** the four facts in ruled cells under a dashed line, each label small over its value. On wide screens all of this sits beside the lead picture.
4. The post in one column of about 70 characters, with its ingredients card and its numbered method. On wide screens the contents list sits beside the text, under the picture.
5. The tags, the share links and the author box.
6. Related posts as index cards.

## Other pages

- **Lists** (a section, a tag, the archive): the list's name and intro over a cherry line, then the posts as index cards.
- **Bakers** (`/authors/`) and **a baker's page:** the shared pages, with Pantry's cards and colours.

## Your brand

Pantry brings its own colours: a cherry accent (`#9b2c2c`) on a flour-white page, with a kraft-paper surface for the ingredients card, the tickets and the recipe box tabs. Shapes are square: no rounded corners and no shadows. Your colours, fonts and logo in `theme/tokens.toml` replace them. A warm book serif for headings and a clear sans for the text suit it best, such as Alegreya and Alegreya Sans.

Pantry's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Pantry's default |
|---|---|
| `[fonts] heading` | Charter, then Cambria and Georgia |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen; a post's text is `1.25rem` |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square corners |
| `[shape] measure` | `40rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the site's name on the front page. |
| `notes_count` | `6` | How many baking notes show on the front page, from `0` to `10`. |
| `hero_stats` | none | Up to four numbers under the site's name, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"side-end"` | Where a long post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## Design

Pantry is an original Khalid design. Its brief is in [docs/design/templates/pantry.md](../design/templates/pantry.md). The demo, Oven Spring, is a made-up home baking blog in Alegreya and Alegreya Sans (SIL Open Font License), with photographs from Wikimedia Commons under CC0 or in the public domain.
