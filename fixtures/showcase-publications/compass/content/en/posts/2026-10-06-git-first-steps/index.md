---
title: "Git for beginners: save your work in five commands"
slug: git-first-steps
date: 2026-10-06
author: imke-sato
reviewed_by: ruben-achterberg
category: tools
description: "Git for beginners: set it up once, then save versions of your website with five commands, see what changed and go back when something breaks."
focus_keyword: Git
tags: [git, terminal]
image:
  src: lead.jpg
  alt: A gravel footpath that splits in two beside a hedge, one branch bending left between fields and the other heading straight on under trees
---

Git for beginners comes down to five commands: `git init` once per project, then `git status`, `git add`, `git commit` and `git log` every time you want to save a version of your work. Git is a program that keeps a dated copy of every version you save, with a note on what changed, so you can always go back to one that worked.

Think of it as a save game for your website. You can try anything, break everything and still return to last Tuesday's page in a few seconds.

::toc

:::callout{kind=summary}
- **A repository** is a project folder that Git watches.
- **A commit** is one saved version, with a short message saying what changed.
- **Save in two moves:** `git add` chooses the changes, `git commit` saves them.
- **`git status` is always safe.** Run it whenever you are unsure.
:::

## Install Git and introduce yourself

Check whether Git is already installed. Open a [terminal](/blog/terminal-basics/) and type:

```bash
git --version
```

If you see a version number, such as `git version 2.47.1`, you are ready. If you see `command not found`, install Git from the official Git website for your system, then open a new terminal and try again. On a Mac, the system may offer to install the developer tools for you, which include Git.

Next, tell Git your name and email. It writes them on every commit you make. You only do this once per computer:

```bash
git config --global user.name "Sam Taylor"
git config --global user.email "sam@example.com"
git config --global init.defaultBranch main
```

The third line names your main line of work `main`, which is what most guides and hosting services expect.

## The five commands

| Command | When to use it | What it does |
|---|---|---|
| `git init` | Once, in a new project folder | Makes the folder a repository |
| `git status` | Any time | Shows which files have changed since your last commit |
| `git add .` | Before each commit | Chooses all changed files in this folder for the next commit |
| `git commit -m "..."` | When something works | Saves a version with your message |
| `git log --oneline` | When you want to look back | Lists your commits, newest first, one per line |

## Save your first versions

::::howto{name="Save your website with Git" time="0:20:00"}
:::step{name="Go to your project"}
In the terminal, `cd` into your `my-first-site` folder. Run `ls` and check you can see `index.html`.
:::
:::step{name="Start a repository"}
Type `git init`. Git answers that it has made an empty repository. Nothing in your folder has changed except a hidden `.git` folder where Git keeps its records.
:::
:::step{name="Look at the status"}
Type `git status`. Your files are listed in red as untracked: Git sees them but is not saving them yet.
:::
:::step{name="Choose and save"}
Type `git add .` (the dot means "everything here"), then `git commit -m "First version of my allotment page"`.
:::
:::step{name="Change something and save again"}
Edit a heading in `index.html` and save the file. Run `git status`: the file shows as modified. Run `git add .` and `git commit -m "Change the main heading"`.
:::
:::step{name="Look back"}
Type `git log --oneline`. You see both commits, each with a short code and your message.
:::
::::

The whole session, start to finish, looks like this. The free [Pro Git book on the official Git website](https://git-scm.com/book/en/v2) explains every command in more depth.

```bash
cd ~/Desktop/my-first-site
git init
git status
git add .
git commit -m "First version of my allotment page"
git log --oneline
```

:::figure{src="stone-steps.jpg" alt="Mossy stone steps climbing a woodland path covered in fallen brown leaves"}
Each commit is one step you can stand on. Small, frequent steps are easier to retrace than one giant leap.
:::

## Write messages your future self can use

A commit message says what the version does, in a few words, as if finishing the sentence "This commit will...". Good messages make `git log` a readable diary of your project.

| Weak message | Better message |
|---|---|
| `changes` | `Add the growing tips list` |
| `fix` | `Fix the broken link to the society page` |
| `stuff` | `Make the cards wrap on narrow screens` |

## Go back when something breaks

Say you changed `style.css`, saved it, and now the page looks wrong, but you have not committed yet. This command puts the file back to how it was in your last commit:

```bash
git restore style.css
```

:::callout{kind=warning}
`git restore` throws away the changes you made to that file since your last commit. That is what you want when the changes broke something, but it cannot be undone. Run `git status` first and read which file you are about to restore.
:::

To see an older version without losing anything, run `git log --oneline`, copy the short code of a commit, and use `git show` with that code to see exactly what it changed.

## Branches, briefly

A branch is a separate line of work. You might make one to try a new layout without touching the version that works. Make one with `git switch -c new-layout`, work and commit as usual, then return with `git switch main`. You will not need branches for your first website, but you will hear the word often.

:::figure{src="second-fork.jpg" alt="A grey gravel path dividing into two under bare spring trees, one branch curving away to the right"}
A branch lets you try another route and come back to the main path if it leads nowhere.
:::

:::callout{kind=tip}
Commit whenever something works, even a small thing. Five commits an evening is normal. A commit is free, and each one is a place you can return to.
:::

Your work is now safe. The last tool on this stop is the one already inside your browser: [use the developer tools](/blog/browser-developer-tools/).
