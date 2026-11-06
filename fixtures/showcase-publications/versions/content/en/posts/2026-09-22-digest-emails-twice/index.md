---
title: "Daily digest emails are sent once again, not twice"
slug: digest-emails-twice
date: 2026-09-22
author: tomas-ferreira
category: fixed
description: "For three days some people got their daily digest emails twice. One email per day goes out again, with a new check that stops a second copy."
focus_keyword: digest emails
tags: [sharing]
facts:
  - label: Version
    value: "3.2.1"
  - label: Platforms
    value: "Email"
  - label: Plans
    value: "All plans"
---

From 17 to 19 September some people got their digest emails twice, a few minutes apart. The two messages were the same. This is fixed, and only one daily digest goes out again.

Nobody got more than two copies on any day, and no other email was sent twice. The other messages Mossboard sends, such as a mention or an invitation, come from a different job that was not moved. Your digest settings were not changed by the bug, so the day and time you chose still apply.

## What went wrong

Digest emails are sent by a job that runs every morning. On 17 September we moved that job to new servers. For three days, the old servers kept running it as well, so two jobs sent the same message. About a third of the people who get a digest were affected, depending on which server reached them first.

## What we changed

We turned off the job on the old servers on 19 September. We also added a check that keeps a short record of each digest sent. A second job that tries to send the same one on the same day now finds the record and stops. Moving a job between servers is common, and the record is a simple guard that makes it safe; the same idea is often called an [idempotent](https://en.wikipedia.org/wiki/Idempotence) operation.

## What you need to do

Nothing. If you turned your digest off because of the extra mail, you can turn it back on under **Settings**, then **Email**. We are sorry for the clutter in your inbox.

## More about digest emails

The digest lists what changed on your boards since the day before: new cards, moved cards and comments that name you. Clients you invite with [guest access](/blog/guest-access/) can get their own digest for the boards they can see. The other fix in 3.2.1 brought back [large attachments](/blog/large-attachments/).
