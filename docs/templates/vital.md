---
title: "Vital"
order: 88
---

# Vital

Vital is a template for a health and wellness site that shares programmes and routines. It shows each routine's time at a glance, opens every routine with a short session panel, and draws its steps as a track. It feels calm and practical, like a good class handout.

```toml
[template]
name = "vital"
```

## Who it suits

- A movement coach or a small studio that shares routines people can follow at home, such as a ten-minute stretch.
- A group that runs programmes over several weeks, such as a beginner walking plan.
- A wellbeing writer who mixes routines with short notes, such as a new timetable.

It suits a news site or a company blog less well. For those, Wire or Keystone read better.

## Keep it to general wellbeing

Vital is made for general wellbeing ideas. Set `care_note` to a plain safety line, and it shows under every routine and on the front page:

```toml
[template.options]
care_note = ["General wellbeing ideas, not medical advice. Go gently, stop if anything hurts, and ask your doctor first if you have a health condition."]
```

Say the same on your about page.

## Routines are posts with facts

Vital needs nothing new in your posts. A post with `facts` is a routine. A post without facts is a note. Put the facts in this order, since Vital reads them by their place, not their label:

```yaml
---
title: "A ten-minute morning mobility routine"
description: "Six gentle moves to wake up your hips, back and shoulders."
facts:
  - label: Time
    value: "10 min"
  - label: Level
    value: "Gentle"
  - label: How often
    value: "Daily"
  - label: You need
    value: "A mat"
---
```

1. **How long a session takes.** It sits in the session ring. Keep it short, such as `10 min`: a longer value turns the ring into a pill.
2. **The level,** such as Gentle, Steady or Strong.
3. **How often, or for how long,** such as Daily or 4 weeks.
4. **What you need,** such as a mat or a chair.

Your labels show on the post, so write what fits. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Write a routine's moves as a numbered list. Vital draws it as the routine track: each step's number in a ring, joined by a soft line. A note's numbered list stays a plain list.

Categories are your programmes, such as Move well or Breathe and rest. Give each an intro in `categories.toml`, and it shows at the top of that programme's page. Tags are what a routine helps with, such as posture or sleep.

## What the front page shows

1. Your site's name, large, and the `tagline` under it.
2. Up to four large numbers from `hero_stats`, such as routines, programmes and minutes a day, each on a soft panel. Nothing shows until you write them.
3. **Programmes:** every category as a rounded chip. A category whose posts on the page are all notes is left out, so a notes category never shows as a programme.
4. **Routines:** your newest routines as picture cards. Each shows its programme, its title, its time in the session ring, and its level and how often beside it.
5. **Notes:** the newest posts without facts as a dated feed, with the care note beside it on wide screens. `notes_count` sets how many.
6. Numbered page links, your call to action as a band, and the newsletter form.

## What a post shows

1. The trail from the home page to the post's programme.
2. The programme as a rounded label, the title, and the post's description under it.
3. The coach with their mark and role, and the date.
4. **The session panel:** the time in a large ring, then the other facts beside it.
5. The care note, when you set one.
6. The lead picture, with rounded corners, then the post in one column about 37rem wide, with a folded contents list for long posts.
7. The tags, the share links and the author box.
8. Related posts as picture cards.

A note has no session panel and no track.

## Other pages

- **Lists** (a programme, a tag, the archive): the list's name and intro on a soft panel, then the posts as picture cards.
- **Coaches** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A coach's page:** their photo or mark, role, experience and bio, then their routines and notes as picture cards, and the other coaches.

## Your brand

Vital brings its own colours: a sage green accent (`#2d6649`) on a warm off-white page, with a pale sage surface for the session panel and the lists' headers. Shapes are round: pill buttons and chips, cards with soft corners. Your colours, fonts and logo in `theme/tokens.toml` replace them. A soft serif for headings and a friendly sans for the text suit it best.

Vital's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Vital's default |
|---|---|
| `[fonts] heading` | Iowan Old Style, then Palatino and Georgia |
| `[fonts] body` | The system's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.125rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.9375rem` |
| `[type] line_height` | `1.65` |
| `[type] heading_weight` | `400` |
| `[shape] radius` | `1rem`, soft corners |
| `[shape] measure` | `40rem` |
| `[space] section` | `4.5rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the site's name on the front page. |
| `care_note` | none | One safety line under every routine's session panel and beside the notes on the front page. |
| `hero_stats` | none | Up to four numbers under the site's name, each written `number \| what it counts`. |
| `notes_count` | `5` | How many notes show on the front page, from `0` to `10`. |
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
| `share_networks` | `["whatsapp", "facebook", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
