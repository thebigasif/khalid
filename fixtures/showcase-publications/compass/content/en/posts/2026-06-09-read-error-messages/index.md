---
title: "How to read error messages without panic"
slug: read-error-messages
date: 2026-06-09
updated: 2026-09-30
updated_note: We added the Firefox wording of each error, after students using Firefox found different text from ours.
author: imke-sato
reviewed_by: hanna-ostrow
category: start-here
description: How to read error messages in four parts, the type, the words, the file and the line, with the five errors beginners meet most often.
focus_keyword: error messages
tags: [beginners, errors, javascript]
image:
  src: lead.jpg
  alt: A black-handled magnifying glass resting on a pale wooden desk beside a steel ruler
---

To read error messages, look for four things in this order: the type of error, the words after it, the file it names and the line number. Go to that line, compare it with what the message says the computer expected, and change one thing at a time until the message goes away or changes.

Every programmer, at every level, reads error messages all day. The difference between a beginner and an experienced developer is not that one gets fewer errors. It is that the experienced one reads them slowly and believes what they say.

::toc

:::callout{kind=summary}
- **An error is information, not a verdict.** It tells you where the computer stopped and why.
- **Read it in four parts:** type, words, file, line.
- **Fix the first error first.** Later errors are often caused by the first one.
- **Change one thing, then run again.** Two changes at once hide which one worked.
:::

## Why error messages look frightening

An error message arrives in red, often with a block of text that looks like a crash. Adults learning to code tell us it feels like being marked wrong in front of a class. It is not. The computer cannot be cross with you. It is following its rules exactly, and it has stopped at the first instruction it could not follow, then written down where that was.

Reading that note is a skill you can learn, like reading a bus timetable. It feels slow at first and automatic after a month.

## The four parts of error messages

Here is a short program with a mistake in it. Save it as `greet.js`, or paste it into the browser console:

```javascript
let visitor = "Priya";
console.log("Welcome back, " + vistor);
```

The browser answers with something like this:

```text
Uncaught ReferenceError: vistor is not defined
    at greet.js:2:32
```

Take it apart:

| Part | In this example | What it tells you |
|---|---|---|
| Type | `ReferenceError` | The kind of problem. A reference error means a name the computer does not know. |
| Words | `vistor is not defined` | The detail. It names the exact thing it could not find. |
| File | `greet.js` | Which file to open. In the console it may say `<anonymous>` or `VM123` instead. |
| Line and column | `2:32` | Line 2, about 32 characters along. Go there first. |

On line 2 the variable is spelled `vistor`, but on line 1 it was made as `visitor`. To a computer those are two different names. Fix the spelling and the error disappears.

:::figure{src="puzzle-piece.jpg" alt="A red jigsaw piece lying across a scatter of green jigsaw pieces on a wooden table"}
An error message points at the one piece that does not fit. You rarely have to rebuild the whole puzzle.
:::

## The five error messages you will meet first

Most beginners' errors are one of these five. The wording of error messages varies a little between browsers, so we show both. The [JavaScript error reference on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors) explains every message in detail.

| Error | Chrome wording | Firefox wording | Usual cause |
|---|---|---|---|
| Reference error | `x is not defined` | `x is not defined` | A typo in a name, or using a name before you made it |
| Syntax error | `Unexpected token` | `expected expression, got ...` | A missing bracket, quote or comma |
| Type error | `Cannot read properties of undefined` | `x is undefined` | Using something that does not hold a value yet |
| Type error | `x is not a function` | `x is not a function` | Brackets after a name that is not a function, or a typo in a function name |
| Missing file | `404 (Not Found)` in the console | `404` in the Network panel | A file path or file name that is wrong |

A syntax error is a grammar mistake: the computer could not even read your code. The other errors happen while your code runs.

## A routine for any of your error messages

When red text appears, follow these steps, in this order, every time. After a few weeks you will do them without thinking.

::::howto{name="Read an error message and fix it" time="0:10:00"}
:::step{name="Stop and read the first line"}
Do not scroll or change anything yet. Read the type and the words out loud, slowly.
:::
:::step{name="Go to the line it names"}
Open the file and find the line number. Look at that line and the line just above it, because a missing bracket is often reported one line late.
:::
:::step{name="Say what the computer expected"}
Put the message into your own words: "It does not know a name called vistor." Often that sentence is the answer.
:::
:::step{name="Change one thing"}
Make one change that matches your sentence, save, and run again.
:::
:::step{name="Read the new message"}
If the error changed, you made progress, even if there is still red. If it is the same, undo your change and try a different idea.
:::
::::

:::callout{kind=tip}
When you are stuck, copy the type and the words of the error, but not your file names, into a search engine. Thousands of people have met the same message, and the first few answers usually explain it.
:::

## When the message points to the wrong place

Sometimes the line number seems wrong. The usual reason is a bracket or quote you opened earlier and never closed. The computer keeps reading, thinking it is still inside the bracket, and only notices the problem later. When the named line looks fine, work upwards and count your opening and closing brackets. A [code editor](/blog/code-editor-setup/) helps here, because it colours each pair of brackets.

:::callout{kind=warning}
Never fix an error by deleting the line it points to, unless you know what that line does. The error goes away, but so does the thing your program was meant to do.
:::

## Error messages outside your code

You will also meet error messages in the terminal and in Git. They follow the same pattern. `command not found` means the terminal does not know the word you typed, almost always a typo. `fatal: not a git repository` means you are in the wrong folder. The [terminal guide](/blog/terminal-basics/) and the [Git guide](/blog/git-first-steps/) list the common ones.

The next stop on the path is a habit that keeps you going: [how much to practise each week](/blog/practice-schedule/).
