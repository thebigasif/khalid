# Cookie consent: what Khalid does for you

Privacy law in the EU, the UK and several US states says a website must ask a reader before it tracks them or stores data on their device. Khalid asks for you, on every site, with no setup. This guide says what it does by default, what each setting changes, and what is left to you.

This guide and the privacy page Khalid writes are not legal advice. They follow the rules regulators publish, but only a lawyer can tell you what your own site needs.

## What happens with no setup

- **Nothing optional loads before a reader agrees.** Your own scripts wait, and so do videos and other pages your posts embed. A reader who never answers loads nothing from anyone but your site.
- **No banner when nothing waits.** Khalid's own visitor counts, fonts, search and share images set no cookie and come from your site, so a site with no scripts of its own shows no banner at all.
- **Embeds ask for themselves.** A YouTube, Vimeo, Spotify, SoundCloud, CodePen or Loom embed, or a page from a domain you allow under `[media] embed_domains`, shows a card instead. The card names who runs it and says that loading it shares the reader's address with them. "Load media" loads that one embed, and "Always load external media" allows every embed on the site.
- **One click to say no.** The banner offers Accept all and Reject all, the same size and style, and Choose, which opens the choices page.
- **Readers can change their mind.** Every page links "Your privacy choices" in its footer. The choices page lists every category with a switch, and saving there replaces the old choice.
- **Global Privacy Control is honoured.** Global Privacy Control, GPC for short, is a signal some browsers send to say "do not sell or share my data". Khalid keeps Marketing off for those readers and tells them the signal was honoured.
- **A choice lasts 6 months.** Then the reader is asked again. They are also asked again when you add a script in a new category, name a new vendor or change the lifetime. Changing only the banner's words does not ask again.
- **A privacy page is written for you** at `privacy/`, from your real settings. See [the privacy page](privacy-page.md).

## The five categories

A category is a group of things a reader can allow together. There are five, and you cannot add more.

| Category | What it covers | Can a reader turn it off? |
|---|---|---|
| Necessary | What the site needs to work, such as remembering the reader's own consent choice. | No, it is always on. |
| Preferences | Code that remembers a reader's settings. | Yes, and it starts off. |
| Analytics | Code that counts and measures visits. | Yes, and it starts off. |
| Marketing | Ads and tracking across sites. Code with no category counts here. | Yes, and it starts off. |
| External media | Videos, sound and pages from other sites. | Yes, and it starts off. |

## Tagging your own scripts

Your own code lives in `site.toml` as `[[code]]` entries, one per script. Give each one the category it belongs to, who runs it, and their privacy policy. You can do the same on the admin's Privacy screen, under Scripts, where you can also add a script, change its code for the head or the end of the page, and remove it. The code for the end of the page may hold only `script` elements. Khalid uses a changed script after it restarts.

```toml
[[code]]
head = "<script async src='https://stats.example.net/a.js'></script>"
category = "analytics"
vendor = "Example Stats"
policy = "https://stats.example.net/privacy"
```

The category is one of `preferences`, `analytics`, `marketing` or `no-tracking`. A script with no category counts as Marketing, the strictest.

Use `no-tracking` only for code that sets no cookie and sends nothing about the reader anywhere. Such code runs for every reader without asking, and the privacy page names it, so readers can check.

## The settings

Each setting is on the admin's Privacy screen, in the `[privacy]` table of `site.toml`, and in the settings API, so the CLI and MCP tools reach it too. Khalid uses a change after it restarts.

| Setting | Default | What it changes |
|---|---|---|
| Months a choice lasts, `lifetime_months` | 6 | How long a reader's answer stands, from 1 to 13 months. 6 is what the French regulator, the CNIL, suggests. |
| Global Privacy Control, `gpc` | `marketing` | `marketing` keeps Marketing off, as US law reads the signal. `reject-all` keeps every optional category off. |
| Count readers' choices, `counts` | on | Daily totals per category, with no address and no reader ID, as proof of consent. A site built as plain files cannot count. |
| Certified consent tool, `external_tool` | none | The name of a paid consent tool you run instead, such as Cookiebot. See below. |
| Banner words, `[privacy.text.<language>]` | Khalid's own | Your own words for the banner and each category, in English and German. |
| Contact, `[privacy.contact]` | none | Your name, email and postal address, for the privacy page. |
| Imprint, `[privacy.imprint]` | none | Adds the imprint German law asks for. See [the imprint](imprint.md). On the admin's Privacy screen it is the Imprint tab. |

Until the contact details are filled in, `khalid doctor` and `khalid build` remind you. Readers never see that reminder.

## Proof of consent

A regulator may ask what your readers were shown and what they chose. Open the admin's Privacy screen, then Proof. It shows each week's totals per category, every version of the banner with the days it was in force, and a CSV download. Khalid keeps the versions for as long as the site exists and the daily totals for 25 months. It never stores a reader's address or anything that names them.

## A site built as plain files

A site with `mode = "static"` is plain files on a host such as GitHub Pages, so no Khalid server sees the reader. A script of under 2 KB, only on sites that need it, shows the banner, keeps the reader's choice and loads what they allowed. A reader with scripts off gets nothing optional: your scripts never run, and each embed is a plain link. Choices are not counted.

## When you use a certified consent tool

Sites that run programmatic ads must use a consent tool that Google certifies, which Khalid is not. Name that tool on the Privacy screen. Khalid then writes your scripts as they are, shows no banner, and names the tool on the privacy page. Your tool must ask before your scripts run. Embeds still show their cards.

## What Khalid checks before each release

Every template's demo is opened in a browser with no consent given, with scripts on and with scripts off, and the check fails if any page asks another site for anything (`scripts/demo-check/network.mjs`). The same check runs on the served demo site.
