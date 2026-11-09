---
title: "Write your first program in ten minutes"
slug: first-program
date: 2026-05-26
author: hanna-ostrow
reviewed_by: desmond-vale
category: start-here
description: Write your first program in ten minutes in the browser you already have. No install, four short JavaScript lines, and what each one does.
focus_keyword: first program
tags: [beginners, javascript]
image:
  src: lead.jpg
  alt: A laptop, a separate white keyboard and a grey mug on a warm wooden desk, seen from above at an angle
---

You can write your first program in ten minutes, today, in the web browser you are using to read this. Every desktop browser has a small window called the console where you can type JavaScript and see the answer at once, so you need nothing to install.

A program is a list of instructions a computer follows in order. Your first program will have four lines. It will say hello, do a sum, remember a name and make a small decision.

## Open the console

The console is part of your browser's developer tools. Open it with the keys for your computer.

| Browser and system | Keys to open the console |
|---|---|
| Chrome, Edge or Brave on Windows | Ctrl + Shift + J |
| Chrome, Edge or Brave on a Mac | Cmd + Option + J |
| Firefox on Windows | Ctrl + Shift + K |
| Firefox on a Mac | Cmd + Option + K |
| Safari on a Mac | First turn on "Show features for web developers" in Settings, then Cmd + Option + C |

A panel opens at the side or bottom of the window, with a blinking cursor after a `>` sign. That is where you type.

:::callout{kind=note}
Some browsers show a warning the first time you paste into the console, because people are sometimes tricked into pasting harmful code there. Type the examples in this guide yourself. It is better practice anyway, and you only run code you can read.
:::

## Write your first program, line by line

Type each line, then press Enter. The browser runs it straight away.

::::howto{name="Write your first program in the browser console" time="0:10:00"}
:::step{name="Say hello"}
Type `console.log("Hello, Northbound!");` and press Enter. The console prints your message. `console.log` means "write this in the console", and the text in quotes is called a string.
:::
:::step{name="Do a sum"}
Type `12 * 7` and press Enter. The console answers `84`. The star means multiply.
:::
:::step{name="Remember a name"}
Type `let learner = "Sam";` and press Enter. You have made a variable: a labelled box that holds a value. Now type `console.log("Hello, " + learner);` to use it.
:::
:::step{name="Make a decision"}
Type the `if` example below on one line and press Enter. The program checks a condition and picks what to say.
:::
::::

Here are all four lines together, so you can check your typing:

```javascript
console.log("Hello, Northbound!");
12 * 7;
let learner = "Sam";
if (learner.length > 2) { console.log(learner + " is a long name"); } else { console.log(learner + " is short"); }
```

The last line prints `Sam is a long name`, because "Sam" has three letters and three is more than two. Change the name to `"Al"`, run the `if` line again and watch the answer change. If the console says `learner` has already been declared, write `learner = "Al";` without `let`.

## What your first program did

You wrote a program. It was short, but it has the same parts as any program you will ever write:

- **Output:** `console.log` showed you a result.
- **Calculation:** the computer worked out a sum.
- **Memory:** a variable kept a value so you could use it later.
- **A decision:** `if` and `else` chose between two paths.

Professional programs are made of these same pieces, arranged in larger numbers. The [console reference on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/API/console/log_static) lists everything `console.log` can do.

:::callout{kind=tip}
Press the up arrow key in the console to bring back the line you typed before. It saves a lot of retyping while you experiment.
:::

## Make a mistake on purpose

Now give your first program a mistake. Type a line with an error in it. Leave out the closing quote:

```javascript
console.log("Hello);
```

The console shows a red message, something like `SyntaxError: Invalid or unexpected token` or `SyntaxError: "" string literal contains an unescaped line break`. The exact words depend on your browser. That is an error message, and it is not a failure. It is the computer telling you, as precisely as it can, what it did not understand. Learning to read these messages calmly is the most useful skill on this whole path.

:::callout{kind=warning}
The console forgets your first program when you close or reload the page. That is fine for experiments. When you want to keep your work, you will save it in a file, which the [first HTML page guide](/blog/first-html-page/) shows you.
:::

## Try three changes on your own

1. Print your own name with `console.log`.
2. Work out how many minutes there are in a week: `60 * 24 * 7`.
3. Make a variable called `age`, then write an `if` that prints one message when it is over 40 and another when it is not.

If one of them goes red, good: your first program has taught you something. Your next guide is [how to read error messages without panic](/blog/read-error-messages/).

Later, [add a little JavaScript to a web page](/blog/first-javascript/) puts these same four kinds of line into a real page.
