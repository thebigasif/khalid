---
title: "Dragging cards on a tablet no longer drops them in the wrong list"
slug: dragging-cards-tablets
date: 2026-10-07
author: tomas-ferreira
category: fixed
description: "Dragging cards on some tablets landed them one list to the left of where they were dropped. Cards now land where your finger lifts."
focus_keyword: dragging cards
tags: [mobile apps, boards]
facts:
  - label: Version
    value: "3.3.0"
  - label: Platforms
    value: "iOS, Android"
  - label: Plans
    value: "All plans"
---

On some tablets, dragging cards put them in the wrong place. You lifted your finger over one list, and the card landed in the list to its left. This happened most on large tablets held sideways. It is fixed in the iOS and Android apps.

Phones were not affected, since a card there is narrow. Tablets in portrait were affected less, because their lists are narrower. If you use a stylus, the fix covers it too: the app follows the tip of the pen in the same way it follows a finger, so a card lands under the point you lift from.

## What went wrong

[Drag and drop](https://en.wikipedia.org/wiki/Drag_and_drop) needs to know where you mean to put a thing. The app worked that out from the left edge of the card you were moving, not from your finger. On a phone the card is narrow, so the two were close. On a wide tablet the card is wide, and its left edge could sit over the list before the one you wanted.

## Dragging cards now follows your finger

The app now follows your finger. The list under your fingertip lights up while you are dragging cards, and that is where the card goes. We also made the space between lists a little wider on tablets, so it is easier to aim, and the board now scrolls sideways sooner when you reach its edge.

## Who it affected

People who use Mossboard on a tablet with wide lists turned on saw this most. If a card went missing while you were dragging cards, it is in the list next to where you let go, and the card's history shows the move with the time.

## Thank you

Everyone who sent a screen recording helped; the recordings made the cause easy to see. The [mobile apps area](/blog/tag/mobile-apps/) lists every change to the apps. Two more fixes followed two weeks later in 3.2.1: [digest emails sent twice](/blog/digest-emails-twice/) and [large attachments](/blog/large-attachments/).
