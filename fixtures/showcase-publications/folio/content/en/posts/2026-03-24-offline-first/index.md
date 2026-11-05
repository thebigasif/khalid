---
title: "Why we built Tern to work offline first"
slug: offline-first
date: 2026-03-24
reviewed: 2026-10-14
author: ada-kerrow
reviewed_by: ines-halloran
category: engineering
description: Why Tern works offline first, keeping every note on your device and syncing in the background, and what that choice costs and buys us.
focus_keyword: offline first
tags: [offline, sync, engineering]
image:
  src: lead.jpg
  alt: A small green plant in a woven basket pot on a white window sill, in front of tall steel-framed windows looking out on brick buildings and a pale sky
---

We built Tern to work offline first: every note lives on your device, every change is saved there before anything else happens, and sync runs quietly in the background when a connection is there. The app never waits for a server to let you write. This note explains why we made that choice in 2022, and what it has cost and bought us since.

Before Tern, I spent five years writing sync for survey software used by field teams. Their tablets often went weeks without a signal. I learned two things there. People will forgive a slow sync, but never a lost note. And software that assumes a connection fails in exactly the places people need it most.

## What offline first means in practice

"Offline first" is a phrase many apps use loosely. For Tern it means four specific promises.

| Promise | What Tern does |
|---|---|
| You can always write | Opening, editing and creating notes never wait for the network |
| Saving is local and instant | A change is on your device's storage before the screen updates |
| Sync is a background job | It runs when it can, retries when it fails and never blocks you |
| Your device has everything | All your notes, not a recent few, are stored on every device you use |

The fourth promise is the expensive one. Many apps keep only recent notes on the device and fetch the rest when asked. That saves space, and it means an old note is out of reach on a plane. Tern keeps everything, which for most people is well under 100 MB.

## How a change travels

When you type, Tern writes the change to a small database on your device within a few milliseconds. It also records the change in a queue of edits waiting to sync. When the device has a connection, Tern sends the queue to our servers, which pass each change on to your other devices.

If two devices change the same note while apart, Tern [merges](https://en.wikipedia.org/wiki/Merge_(version_control)) the changes line by line when they meet again. Where both devices changed the same line, Tern keeps both versions, one under the other, and marks the note so you can choose. In 2025 this happened in about one note in 40,000 edits. We would rather show you both versions than guess and lose one.

:::figure{src="spiral-stairs.jpg" alt="Looking down a wooden spiral staircase, a picture of how an offline first change travels, lit warmly from below, each step fanning out around a curled oak handrail into darkness at the centre"}
Changes travel the same way every time: written locally first, then passed down the line to every other device.
:::

## What it costs

Offline first is not the easy road, and it would be dishonest to say otherwise.

- **More engineering.** Merging changes made apart is hard. It took us longer to build sync than the rest of Tern 1 put together.
- **More storage.** Every device holds every note. On a phone with little space, this matters.
- **Slower features.** Anything that needs all your notes at once, such as search, has to run on your device, not on a powerful server. Search is the clearest example: until now, it was the one part of Tern that needed a connection.

:::callout{kind=warning}
Search in Tern still runs on our servers today. It is the last part of the app that needs a connection, and moving it onto your device is our largest project this year. We have since explained the result in [how offline search works on your device](/blog/how-offline-search-works/).
:::

## What it buys

In return, Tern is fast in a way people notice without being able to name. Opening a note takes the same time on a train in a tunnel as at a desk. People who switch from other apps often say Tern feels "light". Most of that feeling is simply not waiting for a network. The editor we rebuilt for [Tern 2.0 and its linked notes](/blog/tern-2-0-linked-notes/) follows the same offline first rule.

It also means your notes are yours in a practical sense. If our servers went down for a day, you would still have every note on every device and could keep working. Sync would catch up when we came back.

## The principle behind it

The rule we follow is simple: the network is a bonus, never a requirement. Every new feature in Tern is designed first for a device with no connection, then improved for one that has it. That rule has slowed us down more than once. It has never been the wrong call.
