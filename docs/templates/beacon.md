---
title: "Beacon"
order: 86
---

# Beacon

Beacon is a template for a charity or a cause. It shows your campaigns and how far each one is towards its goal, what your work has achieved, and one clear way to give. It reads like a charity's own website, not a blog.

```toml
[template]
name = "beacon"
```

## Who it suits

- A small or mid-sized charity that runs appeals with a money goal, such as a river trust, a food bank or an animal rescue.
- A cause or community group that raises money for one project at a time and posts news from its work.
- A foundation that shows its impact in a few large numbers and wants readers to give or volunteer.

It suits a magazine or a company blog less well. For those, Atlas or Keystone read better.

## Beacon takes no payments

The donate button is your call to action, `[template.cta]` in `site.toml`. It links to a page you own, such as a donate page with your bank details or a link to your payment provider. Beacon never shows a payment form.

```toml
[template.cta]
label = "Donate"
href = "/blog/donate/"
note = "Every £10 plants two trees on a bare riverbank. Give today and help the river run free."
```

The button shows in the header, beside the charity's name on the front page and in each campaign's panel. The note and the button show as a band at the end of the front page and of each post.

## Campaigns are posts with facts

Beacon needs nothing new in your posts. A post with `facts` is a campaign. A post without facts is news. Put the facts in this order, since Beacon reads them by their place, not their label:

```yaml
---
title: "Hob Mill weir removal: help us free the river"
featured: true
facts:
  - label: Raised
    value: "£61,850"
  - label: Goal
    value: "£90,000"
  - label: Supporters
    value: "1,412"
  - label: Closes
    value: "31 March 2027"
---
```

1. **The amount raised.**
2. **The goal.**
3. **and 4.** Any two short facts, such as supporters and the closing date.

When the first two hold a number, a goal meter shows how far the campaign has come, such as "68% of the goal". Beacon reads only the digits, so write whole amounts like "£61,850", not "£1.2m". Your labels show on the page, so "Given" and "Needed" work as well. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Mark one campaign `featured: true` to make it the lead appeal on the front page. Without one, the newest campaign leads. Categories are your areas of work, and tags are a post's topics.

## What the front page shows

1. Your charity's name, large, the `tagline` under it and the donate button.
2. **Appeal now** beside it: the lead appeal as a card with its picture, title and goal meter.
3. **Our impact:** up to four large numbers from `hero_stats` in a band across the page. Nothing shows until you write them.
4. **Campaigns:** your other newest campaigns as cards, each with its picture, area of work, title and goal meter. `campaign_count` sets how many.
5. **News** beside **Our work** on wide screens: the newest posts without facts, each with its date and category, and every area of work as a list of links. `news_count` sets how many news posts show.
6. Numbered page links, the donate band and, when you take sign-ups, the newsletter form.

## What a post shows

1. The trail from the home page to the post's area of work.
2. The area of work as a small label, the title and the post's description.
3. The author with their role, who reviewed it, and the date.
4. **The campaign panel** beside the lead picture on wide screens, under it on phones: the amount raised in large figures, the goal meter, facts 3 and 4, and the donate button.
5. The post in one column about 35rem wide, with a folded contents list for long posts.
6. The tags, the share links and the author box, then the donate band.
7. Related posts as cards, a campaign's with its goal meter.

A news post has no campaign panel.

## Other pages

- **Lists** (a category, a tag, the archive): the list's name and intro, then its posts as cards.
- **Team** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A person's page:** their photo or mark, role, experience and bio, then their posts as cards, and the rest of the team.

## Your brand

Beacon brings its own colours: an ember accent (`#a13d16`) on white, with a warm surface for the impact band and the campaign panel. Your colours, fonts and logo in `theme/tokens.toml` replace them. A sturdy heading font with clear figures suits it best, since the amounts raised and the impact numbers are set in the heading font.

Beacon's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Beacon's default |
|---|---|
| `[fonts] heading` | Rockwell, then other slab serifs and Georgia |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `2rem` |
| `[type] h3` | `1.1875rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0.125rem`, nearly square |
| `[shape] measure` | `40rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the charity's name on the front page. |
| `hero_stats` | none | Up to four impact numbers in a band on the front page, each written `number \| what it counts`. |
| `campaign_count` | `6` | How many campaigns show as cards on the front page after the lead appeal, from `0` to `9`. |
| `news_count` | `4` | How many news posts show on the front page, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"both"` | Where the donate band shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
