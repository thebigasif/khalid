# Lattice: design brief

Lattice is an original Khalid template for a science explainer site. Each post explains one idea, such as why the sky is blue, with numbered figures, footnotes for the sources and links to a glossary of terms. It should read like a well-made science magazine for curious readers, not a research lab and not a news site.

## Who it is for

- A science writer, a museum, a school or a science centre that explains how the world works in plain words.
- A team that wants every claim to carry a source, and every hard word to have a plain definition.
- Readers who arrive from search with one question and should leave with the answer, the numbers behind it and the words to go further.

## The content model

Lattice adds no front matter. It reads what Khalid already has:

- **An explainer is a post.** Its dek is the one-line answer. Its lead picture is the first figure.
- **Key numbers are facts.** Up to four `facts` show as a row of large numbers under the title, such as "Wavelength of blue light: 450 nm". The label says what the number is.
- **Figures are the `:::figure` block** (ADR 0012): a picture with its caption. Lattice numbers every figure in a post, 01, 02 and on, with no script.
- **Footnotes are markdown footnotes** (`[^1]`), which Khalid already renders. Lattice sets each reference as a small numbered tab and the notes as a ruled panel at the end of the text.
- **The glossary is Khalid's glossary** (`content/<lang>/glossary/`, docs/discovery.md). A `[[glossary:slug]]` link in a post is marked as a defined term with a dotted underline, and the glossary page and each term's page get the Lattice look. The site's menu links to the glossary.
- **Topics are categories.** Tags are the post's subjects.

## The front page

1. **Hero.** The site's name large and its tagline. The owner's numbers from `hero_stats` show under it when set.
2. **The lead explainer.** The newest featured post, or the newest post: its picture on the lattice mat, its topic, title and dek, and its key numbers.
3. **Topics.** Every category as a row of ruled tiles.
4. **Latest explainers.** The other posts as a grid of numbered figure plates: picture, topic, title, excerpt.
5. The numbered page links, the call to action band and the sign-up form.

## The post page

1. The topic above the title, then the title, the dek as the answer in one line, the byline, the date and the reading time.
2. **The lead figure** on the lattice mat.
3. **Key numbers:** the facts as a row of large numbers.
4. The text in one column of 60 to 75 characters a line, with numbered figures, footnote tabs and marked glossary terms. The contents list follows the `contents` option.
5. The notes panel, the subjects (tags), the share links, the author box, then related explainers as plates.

## Other pages

- **Lists** (topic, tag, archive): the topic's name and intro, then the posts as figure plates.
- **The glossary** (`/glossary/`): the letters as a row of square mono tabs, then each term as a dictionary entry on a hairline, not a card.
- **A glossary term**: the way back to the glossary, the term and its definition in the reading column, then related terms and the explainers that use it.

## Type pair

- Headings: **Hanken Grotesk** 700, a clean grotesque that reads well in large titles and small labels.
- Text: **Gelasio** 400 and 600, a sturdy serif made for screens, set at 19 px for long reading.
- Figure numbers, key numbers and labels: **DM Mono** 500.

All three are SIL OFL fonts with no reserved font name, downloaded from the Google Fonts repository on GitHub with their OFL files, subset to Latin and self-hosted in the sample brand.

## Palette

The pack's own default is ink on white with a cool grey surface for the mats, a blue link and a burnt orange accent for numbers:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#f3f5f7` | |
| text | `#15191e` | 17.7:1 on white, 16.2:1 on surface |
| muted | `#4f5862` | 7.2:1 on white, 6.6:1 on surface |
| link | `#1d4fb8` | 7.3:1 on white, 6.7:1 on surface |
| accent | `#b5400f` | 5.7:1 on white, 5.2:1 on surface, white text on it 5.7:1 |
| border | `#d5dbe1` | |

The demo's sample brand, Wavelength, uses indigo (`#3b3fb6`, 8.2:1 on white) for links and the accent, on white with a pale lilac surface (`#f1f1fa`).

## Signature device

**The figure plate.** Every picture sits on a lattice mat: a fine square grid drawn from the border colour, like graph paper, with a small mono number tab in its corner, 01, 02 and on. The lead picture, every `:::figure` in the text and every card on the front page and the lists are plates, so the whole site reads like a set of numbered figures. Footnote references use the same small mono tab, so a reader learns one mark for "there is more here".

## How it differs from the existing templates

- **Research** is a lab's site: papers in a numbered reference register with venue, DOI and a cite box. Lattice has no papers. It explains ideas to the public, with pictures as numbered plates, key numbers, footnotes and the glossary.
- **Compass** is a teaching blog of steps, with key points at the top of each guide. Lattice explains why something happens, not how to do a task, and leads with figures and numbers.
- **Almanac** is a resource library of long list guides. Lattice posts are single explainers with a picture-led front page.
- **Ledger** is text-led with no pictures in its lists. Lattice's lists are all pictures, each on its mat.

Lattice reuses the hero and the figure strip from the shared section parts (`crates/khalid-render/templates/sections/`). It adds no new shared part: the figure plate and the notes panel are styles on Khalid's own figure and footnote markup.
