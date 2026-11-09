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

Like every newsletter email, a welcome ends with the reader's one-click unsubscribe link and the postal address, so it goes out only once `postal_address` is set under `[mail]`. `khalid serve` reads the files when it starts.

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

lists each email that went out, newest first, with three totals: copies sent, copies the mail server refused, and readers who unsubscribed with a link from that email. Numbers are totals per email, never per person. Khalid does not count opens: that needs a hidden image in every email, which tells you who read what, so it is off.

## RSS to email

```sh
khalid --site <folder> newsletter digest --list weekly
```

sends one email that lists every post published since the last digest, each with its title and link. The first digest looks back seven days. With nothing new it sends nothing, so it is safe to run from a timer, such as a systemd timer or a cron job once a week. `--label` narrows it to a segment and `--language` picks the posts' language.
