---
title: "Use the terminal: the ten commands a beginner needs"
slug: terminal-basics
date: 2026-09-15
updated: 2026-10-21
updated_note: We added the Windows PowerShell name for each command after questions from our autumn group.
author: imke-sato
reviewed_by: desmond-vale
category: tools
description: Use the terminal with confidence. Open it, learn where you are, move between folders and make, copy and remove files with ten commands.
focus_keyword: terminal
tags: [terminal, beginners]
image:
  src: lead.jpg
  alt: Close view of the pale grey keys of a slim keyboard, lit from the side so each key casts a small shadow
---

The terminal is a window where you type commands to your computer instead of clicking, and you need only about ten of them to start. With those ten you can see where you are, move between folders, and make, copy, move and remove files, which is all most beginners use it for.

Developers use the terminal because many tools, Git among them, are easiest to run from it. It looks old-fashioned, and it is: a [command-line interface](https://en.wikipedia.org/wiki/Command-line_interface) is older than the mouse. But it is just another way to do what you already do in a file window.

::toc

:::callout{kind=summary}
- **The terminal always works in one folder**, called the current or working directory.
- **`pwd`, `ls` and `cd`** tell you where you are, what is there and how to move.
- **`mkdir`, `touch`, `cp` and `mv`** make, copy and move things.
- **`rm` deletes for good.** There is no bin to rescue files from.
:::

## Open the terminal

| System | How to open it | What it is called |
|---|---|---|
| macOS | Press Cmd + Space, type Terminal, press Enter | Terminal |
| Windows 10 or 11 | Right-click the Start button, choose Terminal or Windows PowerShell | Terminal or PowerShell |
| Linux | Press Ctrl + Alt + T on most systems | Terminal |

A window opens with a line of text ending in `$`, `%` or `>`. That is the prompt: it means the terminal is waiting for you.

:::callout{kind=note}
The commands in this guide work on macOS and Linux, and in Windows PowerShell, which understands the same short names for these ten commands. Where PowerShell needs something different, the cheat sheet says so.
:::

## The ten commands

This is the cheat sheet we hand out in class. Print it and keep it beside your keyboard for the first month.

| Command | What it does | Example | PowerShell |
|---|---|---|---|
| `pwd` | Shows the folder you are in | `pwd` | Same |
| `ls` | Lists what is in this folder | `ls` | Same |
| `cd` | Moves into a folder | `cd Documents` | Same |
| `cd ..` | Moves up one folder | `cd ..` | Same |
| `mkdir` | Makes a new folder | `mkdir my-first-site` | Same |
| `touch` | Makes a new empty file | `touch notes.txt` | `ni notes.txt` |
| `cp` | Copies a file | `cp index.html backup.html` | Same |
| `mv` | Moves or renames a file | `mv old.html new.html` | Same |
| `rm` | Deletes a file | `rm backup.html` | Same |
| `clear` | Clears the screen | `clear` | Same |

## Find your way around

Type each command and press Enter.

```bash
pwd
ls
cd Desktop
ls
cd ..
```

`pwd` prints the path of the folder you are in, such as `/Users/sam` on a Mac or `C:\Users\sam` on Windows. `ls` lists what is inside. `cd Desktop` moves you into your Desktop folder, and `cd ..` moves you back up, because `..` always means "the folder above this one".

:::callout{kind=tip}
Press the Tab key halfway through a folder or file name. The terminal finishes it for you, and it never misspells. If nothing happens, press Tab twice to see every name that starts that way.
:::

## Make a project folder from the terminal

::::howto{name="Make a project folder in the terminal" time="0:15:00"}
:::step{name="Go to your Desktop"}
Type `cd ~/Desktop` and press Enter. The `~` sign means your home folder.
:::
:::step{name="Make a folder"}
Type `mkdir practice-site` and press Enter. Look at your Desktop: the folder is there.
:::
:::step{name="Move into it"}
Type `cd practice-site`, then `pwd` to check where you are.
:::
:::step{name="Make two files"}
Type `touch index.html style.css` on a Mac or Linux, or `ni index.html, style.css` in PowerShell. Then `ls` to see them.
:::
:::step{name="Copy one"}
Type `cp index.html about.html` and `ls` again. You now have three files.
:::
::::

## Reading the terminal's answers

Many commands print nothing when they work. That silence is good news. When something goes wrong, the terminal tells you, and the messages follow a pattern:

| Message | What it means | What to do |
|---|---|---|
| `command not found` | The terminal does not know that word | Check the spelling |
| `No such file or directory` | The name you gave does not exist here | Run `ls` and check you are in the right folder |
| `Permission denied` | Your account may not change that file | Work inside your own home folder |
| `Is a directory` | You gave a folder where a file was expected | Check the name, or use the folder command |

:::callout{kind=warning}
`rm` deletes files immediately and for good. They do not go to the bin. Before you press Enter on any `rm`, run `ls` and read the name twice. Never type `rm` with `-rf` from an example you do not fully understand: it deletes whole folders and everything inside them.
:::

## Why bother with the terminal at all

For a single page, clicking is faster. The terminal pays off when you start using developer tools, because most of them are run by typing. Git, the tool that saves versions of your work, is the first one you will meet on this path, and it is the next stop after you set up a proper editor: read [set up a code editor](/blog/code-editor-setup/) next.
