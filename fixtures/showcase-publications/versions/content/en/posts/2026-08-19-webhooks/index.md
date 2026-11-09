---
title: "Webhooks send card changes to your own tools"
slug: webhooks
date: 2026-08-19
author: tomas-ferreira
category: new
description: "Webhooks send a short message to an address you choose each time a card is added, moved or finished, so your own tools can act on it."
focus_keyword: webhooks
tags: [integrations]
facts:
  - label: Version
    value: "3.1.0"
  - label: Platforms
    value: "Web"
  - label: Plans
    value: "Business"
---

Business plans can now use webhooks. A [webhook](https://en.wikipedia.org/wiki/Webhook) is a short message Mossboard sends to an address you choose whenever something happens on a board: a card is added, moved, finished or commented on. Your own tools can then act on it, such as posting a note in your team chat or starting a build.

Messages go out within a few seconds of the change, in the order the changes happened. Your tool should answer within ten seconds; a slow answer counts as a failed delivery and the message is sent again later.

## Setting up webhooks

Open a board, choose **Settings**, then **Connections**, and add the address that should receive the messages. Pick the events you want. Mossboard sends a test message straight away, so you can check it arrives.

Each message is a small JSON document with the board, the card and what changed. It carries a signature made with a secret only you and Mossboard know, so your tool can check the message really came from us and was not changed on the way.

## When a delivery fails

If your address does not answer, Mossboard tries again five times over an hour. The settings page shows the last 50 deliveries with their answers, which helps when something goes wrong. After a day of failures we pause the connection and email the board's owner.

## What they are for

Teams asked for this to connect Mossboard to tools we do not support directly: a build server, a help desk, a chat room. These messages are the building block; we will add ready-made connections on top of them later, starting with the most common chat apps.

## Changes that send a message

The [archived cards](/blog/archived-cards/) in this version send one when a card is archived. Cards moved with the [keyboard shortcuts](/blog/keyboard-shortcuts/) of 3.2 send one too, the same as a card moved with the mouse.
