# The privacy page

Every Khalid site has a privacy page at `<mount>privacy/`, in each of its languages. Khalid writes it from the site's settings and the features it has on, so it is right from the first day and changes when they do. It is a starting point, not legal advice.

## What it says

- Who runs the site, and how to reach them.
- What the site collects and why: comments, the newsletter and forms, but only the ones the site has on, and the visitor log when `[visitor_log]` is on.
- Every cookie the site may set, with its name, what it does and how long it lasts.
- Other companies: the code that waits for a reader's consent, by category, the video and audio players the site's pages embed, and the scripts that set no cookie and track no one.
- How long each kind of data is kept.
- A reader's rights, how to use them, and how to withdraw consent.
- How the site answers Global Privacy Control and Do Not Track.
- Where to complain, and the day the page last changed.

The day moves only when what the page says changes. Khalid keeps it in `data/privacy-facts`, which is not content: delete the file and the next build dates the page today.

## Contact details

Fill in your name and email on the admin Privacy screen, under Contact. Until you do, `khalid doctor` and `khalid build` remind you. Readers never see that reminder.

## Writing your own page

Write `content/<language>/pages/privacy.md` with the slug `privacy`, and it replaces the generated page. To keep the generated facts in it, write `::privacy-facts` on a line of its own where they belong:

```markdown
---
title: How we treat your data
slug: privacy
---

We keep as little about you as we can.

::privacy-facts

Write to us any time.
```

Your words are shown as written. Khalid never reads them as a template, so `{{` and `}}` show as they are. On any other page `::privacy-facts` shows nothing.
