---
title: "Due dates no longer move a day for people in other time zones"
slug: due-dates-time-zones
date: 2026-10-28
author: tomas-ferreira
category: fixed
description: "A due date set late in the day could show as the next day for teammates abroad. Due dates without a time now stay on the same date for everyone."
focus_keyword: due dates
tags: [calendar]
facts:
  - label: Version
    value: "3.4.0"
  - label: Platforms
    value: "Web, iOS, Android"
  - label: Plans
    value: "All plans"
---

Some teams saw due dates move by a day. A card due on Friday showed as due on Saturday for a teammate further east, or on Thursday for one further west. This is now fixed, and dates you set before the fix show on the day you chose.

The fix reaches the web app and both mobile apps at once, so every device shows the same day.

## What went wrong

Due dates without a time were stored as midnight where the person who set it lived. Mossboard then showed that moment in each reader's own [time zone](https://en.wikipedia.org/wiki/Time_zone), which for some people fell on the day before or after. So a card that should mean "Friday" meant "Friday in London".

## What we changed

A date without a time is now stored as a plain day, with no clock attached, so it means the same day everywhere. A deadline with a time, such as "Friday at 15:00", still follows each reader's clock, which is what you want for a call or a hand-in at a set hour.

We moved every existing date to the day its author saw when they set it. You do not need to do anything, and no card lost its date.

## Who it affected

About one team in twenty has people in more than one country, and most of them saw at least one card move. Thank you to the teams who sent screenshots; three of them showed the same Friday card from three cities, which made the cause plain.

## How to check due dates on your cards

Open a board and switch to the [calendar view](/blog/calendar-view/). Every card should sit on the day you meant. If one does not, drag it to the right day and tell us about it, so we can look at what happened. The [card search](/blog/card-search-comments/) can find any card by its title if you are not sure which board it is on.
