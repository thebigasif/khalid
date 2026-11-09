---
title: "The case for quiet reminders"
slug: quiet-reminders
date: 2026-10-20
author: ines-halloran
reviewed_by: rafe-morrow
category: essays
description: Why Tern has quiet reminders that arrive once, at a time you chose, and the design rules we follow so a note-taking app never nags.
focus_keyword: quiet reminders
tags: [design, reminders]
image:
  src: lead.jpg
  alt: A white orchid flower with a magenta and orange centre, in sharp focus in front of a softly blurred window with green trees outside
---

Tern has quiet reminders because a note-taking app should help you remember, not compete for your attention. A reminder in Tern arrives once, at the time you chose, without a sound unless you ask for one, and it never repeats itself or invents a reason to bring you back. Tern 2.5, out in early November, makes them quieter still, and this essay explains why.

Most apps on your phone are designed to bring you back as often as possible. Their reminders are really invitations, and their badges are small alarms. We decided early that Tern would not work that way. A note is something you chose to keep. The app's job is to give it back to you at the right moment, then get out of the way.

## What we mean by quiet reminders

Quiet reminders follow five rules. Each one came from a mistake we made or saw elsewhere.

| Rule | What it means in Tern |
|---|---|
| Once | A reminder appears once. If you do not open it, it waits in the note, not on your lock screen |
| On time | It arrives at the minute you set, never "around then" to save battery |
| Silent by default | No sound and no vibration unless you turn them on for that reminder |
| About the note | It shows the note's title and first line, never a message we wrote to tempt you |
| No streaks | Tern never counts days in a row or tells you that you missed one |

The last rule caused the longest discussion. [Streaks](https://en.wikipedia.org/wiki/Gamification) work. They make people open an app every day. But they turn a tool into a chore, and the guilt of a broken streak is a poor reason to write anything down.

## Remind me later, done properly

When a reminder arrives at a bad moment, you can choose **Remind me later**. Older versions of Tern offered fixed times: in an hour, this evening, tomorrow. In interviews, people told us these were almost never right. "This evening" meant six o'clock to one person and ten o'clock to another.

From Tern 2.5, Tern learns the times you choose. If you usually push reminders to half past eight in the evening, that time appears first. Nothing else is learned, and nothing leaves your device. Rafe heard about the old fixed times so often while [answering support email](/blog/everyone-answers-support/) that he kept a tally; it reached 140 before we fixed it.

:::figure{src="window-shadow.jpg" alt="The shadow of a window frame and potted plants falling across a pale linen curtain in soft morning light, as calm as quiet reminders should be"}
A reminder should feel like light moving across a room: noticed when you look, never demanding that you look.
:::

## What we turned down

Several ideas for reminders sounded helpful and were turned down because they broke the rules above.

- **Smart nudges.** Reminding people about notes they had not opened in a while. In testing, people found it unsettling, as if the app were watching them.
- **Daily summaries.** A morning message listing the day's reminders. Most people already had a calendar for that, and two lists disagreed.
- **Repeat until done.** Reminders that came back every hour until dismissed. Useful for medicine, perhaps, but Tern is not a medicine app, and we would rather point people to tools built for that.

:::callout{kind=note}
Quiet does not mean hidden. Every reminder you set is listed in one place, under **Reminders** in the sidebar, with its note and its time. You can see, change or delete all of them at once.
:::

## How quiet reminders changed what people do

In Tern 2.4 (2026), about 30 percent of reminders were dismissed without the note being opened. In two weeks of testing Tern 2.5 with 300 people on our beta, that fell to 18 percent (October 2026). We read that as a good sign: fewer reminders are set out of hope, and more of them arrive at a moment when people can act.

We will keep measuring, and we will keep quiet reminders as the rule. Like the [sketches we draw on paper first](/blog/sketch-on-paper/), they are a small habit with a large effect. A note-taking app earns trust slowly and loses it fast. One nagging reminder too many, and people switch notifications off for the whole app, including the reminders they wanted.
