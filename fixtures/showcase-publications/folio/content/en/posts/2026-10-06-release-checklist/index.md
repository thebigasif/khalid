---
title: "Our release checklist, line by line"
slug: release-checklist
date: 2026-10-06
author: greta-voss
reviewed_by: ada-kerrow
category: how-we-work
description: The release checklist the Tern team runs before every version ships, line by line, with why each step is there and the mistake that added it.
focus_keyword: release checklist
tags: [process, releases]
image:
  src: lead.jpg
  alt: Grey steel panels of a building facade meeting at sharp angles, each seam lined with neat rows of small rivets, under flat white light
---

Our release checklist is a single page of 24 short steps that every Tern release passes before it reaches anyone, run by two people over two days. Most steps exist because of one specific mistake, and we keep a note of which. This post walks through the checklist line by line, grouped into its four parts, so you can borrow what is useful.

We are a team of nine shipping to five platforms. We cannot test everything by hand, and we do not try. The release checklist covers the things automated tests miss: the things that only go wrong when a real person updates a real device full of real notes.

::toc

## Who runs the release checklist

Every release has two owners: one engineer and one person who did not build the release. In practice, the second person is often me or Rafe. The second person matters most. They read every step as written, without the knowledge that lets the builder skip ahead.

## Part one: before the build

| Step | What we check | Why it is there |
|---|---|---|
| 1 | Every change in the release has a line in the draft release note | A fix in 2024 shipped silently and confused people who had reported it |
| 2 | Every new screen has been tested with a screen reader | [Tables in Tern 2.2](/blog/tern-2-2-tables/) had eleven screen reader problems in the first build |
| 3 | Every new text has been read aloud by someone outside the team | Names that confuse; see [how we name features](/blog/naming-features/) |
| 4 | The "what we will not do" list from the [one-page spec](/blog/one-page-spec/) still holds | Scope grows quietly in the last week of a [six-week cycle](/blog/six-week-cycles/) |
| 5 | Database changes have been run on a copy of a large account | One migration in 2023 took four minutes on large accounts |
| 6 | Sync between the new version and the old one works in both directions | People update one device at a time |

Step six is the one other teams ask about most. For a few days after every release, many people have the new version on one device and the old one on another. Both must sync with each other without losing anything.

## Part two: the beta

The build goes to our beta group for at least five days.

- **Step 7:** at least half the beta devices have updated and opened the app.
- **Step 8:** the crash rate is no higher than the last release's.
- **Step 9:** sync conflicts stay at their normal, very low rate.
- **Step 10:** every note from the beta is read and labelled by Rafe or the engineer on support.
- **Step 11:** every note marked "data" is answered and closed before release.

:::callout{kind=warning}
Any report that suggests a lost note stops the release, however unlikely it seems. In three years this has stopped four releases. Three were false alarms. The fourth was not, and it is why step 11 exists.
:::

## Part three: release day

- **Step 12:** we release on a Monday, Tuesday or Wednesday, never later in the week.
- **Step 13:** the release goes to a tenth of people first, then everyone after a day.
- **Step 14:** the person [answering support email](/blog/everyone-answers-support/) that day knows what changed.
- **Step 15:** the help pages for any changed feature are updated before the release, not after.
- **Step 16:** the release note is published at the same time as the release.
- **Steps 17 to 20:** each platform's store listing, screenshots and version number are checked by hand.

## Part four: the week after

- **Step 21:** we compare crash and sync rates with the last release, daily, for seven days.
- **Step 22:** we read every support email that mentions the release.
- **Step 23:** we write down anything that surprised us.
- **Step 24:** we update this checklist.

Step 24 is the reason the checklist works. A checklist that never changes slowly stops matching the work, and people stop trusting it. Ours changes after most releases, usually by one line.

## What is not on the checklist

Things a machine can check are not on this release checklist. Automated tests, formatting, builds for each platform and store uploads all run on their own and must pass before step 1. The checklist is only for judgement: the steps that need a person to look, read or decide.

If you want to start your own [checklist](https://en.wikipedia.org/wiki/Checklist) for releases, start with the last mistake you shipped. Write the step that would have caught it. Then do the same after your next release. In a year you will have a checklist that fits your team, which is better than any you could copy.
