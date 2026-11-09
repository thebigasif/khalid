# Inkwell: design brief

Inkwell is an original Khalid template for the archive of a newsletter that has run for years. Every issue has a number. Readers come to find an old issue, to see what a new one holds and to sign up for the next. It should feel like a shelf of bound volumes: one volume a year, each issue numbered in order, its contents listed on the spine.

## Who it is for

- A writer or a small group who has sent a newsletter for years and keeps every issue on the web.
- A society, club or library whose letter goes out a few times a year.
- A weekly or monthly letter with regular sections, such as a lead story, a reading list and letters from readers.

## The content model

Inkwell adds no front matter. It reads what Khalid already has:

- **An issue is a post.** Its title is the issue's title, its date the day it went out.
- **Facts give the number and the contents,** read by position: 1 the issue's number, such as `91`; 2 to 4 the parts of the issue, each with the section as its label and the item as its value, such as `Growing: Saving runner bean seed`. The labels are the writer's own.
- **A post without facts** is an extra, such as a supplement or a note between issues. It shows in the archive without a number.
- **Categories and tags** sort issues as the writer wants, such as issues and supplements, or subjects.
- **Authors** are the editor and the regular writers.

## The front page

1. **The masthead:** the tagline as a small label, the newsletter's name large and its intro, beside the sign-up form, so a new reader can join at once.
2. **The latest issue:** the newest post with a number. Its number set very large, the date, the title, the description, "In this issue" with its parts, a "Read this issue" link and its lead picture.
3. **Every issue:** a row of links to each year on the page, then the issues grouped by year. Each year opens with the year in large figures, a heavy ink rule and how many issues it holds. Each issue is a row: its number, its title and date, and its parts at a glance.
4. The page links, the call to action band and the newsletter form when the masthead has none.

## The post page

- A stamp with the issue's number, then the date and the category.
- The title, the description and the author.
- **In this issue:** the issue's parts in a box ruled down its start edge, each part's section in small capitals over the item.
- The lead picture, then the text at a reading width.
- The tags, the share links and the author box, then the sign-up form, then more issues as the same rows as the archive.

## Other pages

- **Lists** (category, tag, archive, dates): the list's name and intro, then its issues grouped by year as on the front page.
- **People** (`/authors/`): a grid of the editor and writers.
- **A person:** their mark or photo, name, role and bio, then their issues grouped by year.

## Type pair

- Headings and the issue numbers: **Vollkorn** 600, a sturdy book face with old-style figures, so the large numbers read as printed numerals.
- Text and labels: **Mulish** 400 to 700, a plain, open sans that reads well on a screen and in small capitals labels.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own default uses the system's book face and sans, with the same sizes.

## Palette

The pack's own default is a blue-black ink on a pale paper:

| Token | Value | Check |
|---|---|---|
| background | `#fbfaf7` | |
| surface | `#efede6` | |
| text | `#181c27` | 16.3:1 on the background, 14.5:1 on the surface |
| muted | `#4c5263` | 7.5:1 on the background, 6.7:1 on the surface |
| link | `#1d4c9c` | 7.8:1 on the background, 7.0:1 on the surface |
| accent | `#1d4c9c` | white text on it 8.2:1 |
| border | `#d9d5cb` | |

The demo's sample brand, The Seed Tin, is a beetroot red (`#7b2d43`) on a warm paper (`#fcf9f3`), with every text pair above 7:1.

## Signature device

**The bound volume.** Each year of issues opens like the spine of a bound volume: the year in large old-style figures over a heavy ink rule, with how many issues it holds. Under it, every issue keeps its number in a column of its own, so the numbers run down the page in order like folios, and each row lists the issue's parts at a glance. The same number shows as a stamp on the issue itself.

## How it differs from the existing templates

- **Ticker** is a markets newsletter: figures, bar charts and data tables. Inkwell has no figures. It is an archive of words, read by number and year.
- **Chapter** lists writing as a contents page with dotted leaders. Inkwell groups issues into yearly volumes, each with its parts listed, and leads with the sign-up form.
- **Research** groups papers by year as a numbered reference list. Inkwell numbers issues by their own number, not their place in the list, and shows each issue's parts.

Inkwell builds its rows from its own parts file, `parts-inkwell.html`, since no other template on the list groups numbered issues with their parts. It reuses the shared hero and people grid.
