---
title: "Newsletters"
order: 80
---

# Newsletters

Readers sign up with a plain form, confirm with one click in an email, and leave with one click in any email after that. Sending needs mail settings, see "Sending email" in `docs/deploy.md`.

## Several newsletters

Every site has one newsletter called `main`. To add another, make a folder for it in `content/newsletters/`, named in lowercase letters, digits and hyphens:

```text
content/
  newsletters/
    weekly/
```

Then name it in the sign-up block:

```markdown
::newsletter{label="Your email address" button="Get the weekly links" list="weekly"}
```

Leave `list` out and the form signs readers up for `main`. A form that names a newsletter with no folder saves nothing and tells the reader. Only the main newsletter's page and past issues become pages, see "The newsletter page" below. One reader can get several newsletters, and unsubscribing stops all of them.

## Labels and segments

A label is a word you give a subscriber, such as `paid` or `from-launch`, in lowercase letters, digits and hyphens. A segment is one newsletter's confirmed subscribers who carry every label you name.

```sh
khalid --site <folder> newsletter label grace@example.com paid
khalid --site <folder> newsletter label grace@example.com paid --remove
khalid --site <folder> newsletter segment --list weekly --label paid
```

`segment` prints how many subscribers are in the segment. It never lists who they are. Add `--json` to either command for a program to read.

## Sending a post

A post is emailed only when you ask, and only once:

```sh
khalid --site <folder> newsletter send what-a-sitemap-does
khalid --site <folder> newsletter send what-a-sitemap-does --list weekly --label paid
```

The post must be published. Each confirmed subscriber in the newsletter, or in the segment the labels name, gets one copy: the title, a link to read it on the site, and its text. Forms, galleries and other blocks that only work on a page are left out. Every copy ends with that reader's own one-click unsubscribe link and the site's postal address.

Khalid sends nothing until `postal_address` is set under `[mail]` in `site.toml`, because anti-spam laws ask every newsletter email to show one. A post that went out before is never sent again, even after an edit. A copy the mail server refuses is counted and not sent again.

## Welcome emails

A newsletter can greet each reader the moment they confirm. Write the email in `content/newsletters/<name>/welcome.md`, such as `content/newsletters/weekly/welcome.md`, or `content/newsletters/main/welcome.md` for `main`. The first line is a `# ` heading that becomes the subject, and the rest is the text:

```markdown
# Welcome to the weekly links

Every Friday you get five links worth your time, with a line on why each one matters.
```

A reader who already confirmed and signs up for one more newsletter gets that newsletter's welcome the moment they join it, with no second confirmation.

Like every newsletter email, a welcome ends with the reader's one-click unsubscribe link and the postal address, so it goes out only once `postal_address` is set under `[mail]`. `khalid serve` reads the files each time a reader is due one, so an edit reaches the next reader without a restart.

### A welcome sequence

To send more emails in the days after someone joins, add one file per email beside `welcome.md`, named by the day it goes out: `welcome-day-3.md` goes out three days after the reader joined, `welcome-day-10.md` ten days after. Each is written like `welcome.md`. Days run from 1 to 365.

```text
content/
  newsletters/
    weekly/
      welcome.md
      welcome-day-3.md
      welcome-day-10.md
```

`khalid serve` looks once an hour, so each email arrives within the hour of its day. Each reader gets each email once. If the server was stopped on an email's day, it goes out when the server is back, up to a week late; after that it is skipped, so a reader who joined long before you wrote a new step does not get it out of the blue.

## The newsletter page

Readers decide to sign up when they can see what they will get. Write `content/newsletters/main/page.md`: a `# ` heading with the newsletter's name, then what readers get and how often, in markdown. Any block works, such as a summary callout:

```markdown
# The Larkspur Letter

One short email every other Tuesday, five minutes to read.

:::callout{kind=summary}
- **One lesson from client work**, with the before and after.
- **Three links worth your time**, each with a line on why.
:::
```

Then keep each issue you send as a file in `content/newsletters/main/issues/`, named by the day it went out and a slug, such as `2026-10-27-ai-search-quotes.md`. Its first line is a `# ` heading with the subject, and the rest is the text.

With `page.md` in place, the blog has a page at `/newsletter/` in its main language: your text, the sign-up form, the newest issue's opening as a sample, and every past issue, newest first. Each issue gets its own page at `/newsletter/<slug>/`, with the day it went out, the form and a link back. An issue dated after today waits until that day. A static blog shows the same pages without the form, since nothing could take the sign-up. Add the page to your menu or footer with `href = "/blog/newsletter/"`.

## A sign-up form on your main website

To let readers sign up from your main website, print the form and paste it into any page there:

```sh
khalid --site <folder> newsletter embed --list weekly
```

It is plain HTML that posts to the blog's full address, so it needs no script, no frame and nothing loaded from the blog. Style it with the main site's CSS through the `khalid-subscribe` class. After signing up, the reader lands on the blog's "Check your inbox" page.

## Numbers

```sh
khalid --site <folder> newsletter numbers
```

lists each email that went out, newest first, with its totals: copies sent, copies the mail server refused, readers who unsubscribed with a link from that email, and clicks on its links. Numbers are totals per email, never per person. Khalid does not count opens: that needs a hidden image in every email, which tells you who read what, so it is off.

Clicks are counted only when you ask, with `count_clicks = true` under `[mail]` in `site.toml`. Each link in an email you send after that leads through your blog first, such as `https://www.example.com/blog/subscribe/click?send=4&link=2`, which counts the click and sends the reader on. Every reader's copy has the same links, so a click tells you which link was popular, never who clicked. Some mail programs open links to check them for safety, so a few clicks may come from them. Welcome emails are not counted.

## RSS to email

```sh
khalid --site <folder> newsletter digest --list weekly
```

sends one email that lists every post published since the last digest, each with its title and link. The first digest looks back seven days. With nothing new it sends nothing, so it is safe to run from a timer, such as a systemd timer or a cron job once a week. `--label` narrows it to a segment and `--language` picks the posts' language.

## In the admin and the Admin API

The admin's Subscribers screen does all of the above except the digest while `khalid serve` runs: it lists your readers, gives and takes away labels, forgets a reader who asks, shows each newsletter's readers and sign-up form, sends a published post, and shows the numbers. Only the owner and admins see it, because addresses are personal data.

The same routes are in the Admin API, for a key with the `subscribers:write` scope:

| Route | What it does |
|---|---|
| `GET /api/admin/v1/subscribers/` | Lists readers, newest first. Filter with `status` (`pending`, `confirmed` or `unsubscribed`), `list`, `label` and `q` (text in the address), page with `page` and `limit`. |
| `PUT /api/admin/v1/subscribers/{id}/labels/{label}/` | Gives a reader a label. A reader who unsubscribed gets none. |
| `DELETE /api/admin/v1/subscribers/{id}/labels/{label}/` | Takes a label away. |
| `DELETE /api/admin/v1/subscribers/{id}/` | Forgets a reader: their address, newsletters, labels and links are removed. The totals of emails sent stay, as they name no one. |
| `GET /api/admin/v1/newsletters/` | Each newsletter with its confirmed and waiting readers and its sign-up form, whether the site can send now (`ready`, `no_mail` or `no_postal_address`), and the numbers of every email sent. |
| `POST /api/admin/v1/newsletters/{name}/sends/` | Sends a published post, `{"post": "en.what-a-sitemap-does", "labels": ["paid"]}`, once. It answers `202` with the send's key as the copies start to leave. |

No route adds a subscriber. Readers join only through the form and the link in its email, so every address on the list asked to be there.
