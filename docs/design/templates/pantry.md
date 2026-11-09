# Pantry: design brief

Pantry is an original Khalid template for a baking and recipe blog: a home baker, a small bakery that shares its recipes, or a writer who tests cakes, breads and biscuits. It should feel like a tidy recipe box on a floury worktop: index cards, clear weights, and the ingredients before the story.

## Who it is for

- A home baker who writes tested recipes with weights, times and oven settings.
- A small bakery or cookery school that shares its recipes and baking notes.
- A recipe writer who wants readers to find the ingredients at once, not after a long story.

## The content model

Pantry adds no front matter. It reads what Khalid already has:

- **A recipe is a post with facts,** read by position: 1 the total time, 2 what it makes (the yield), 3 the oven setting, 4 the level. The labels are the writer's own.
- **A baking note is a post without facts,** such as a method or a kitchen tip. Notes show in their own list, not among the recipes.
- **Sections of the recipe box are categories,** such as Bread, Cakes, Biscuits and Pastry. A category of notes only is left out of the recipe box.
- **Ingredients and method are the post's own lists.** Every bullet list before the first numbered list is an ingredients list; the first numbered list is the method. A recipe writes them in that order, with headings such as "Ingredients" and "Method".
- Bakers are authors, with their role.

## The front page

1. **Hero:** the site's name and its `tagline`, with an optional figure strip from `hero_stats`.
2. **The recipe box:** each category that holds recipes as a divider tab in a row along a ruled line, like the tabs in a card box.
3. **Fresh from the oven:** the newest recipe as one wide index card: its picture beside its section, title, description and its full ticket of four facts.
4. **Recipes:** the next recipes as index cards in a grid. Each card has its picture, section, title and a tear-off ticket with the time and the yield.
5. **Baking notes:** the newest notes as a dated list. `notes_count` sets how many.
6. Page links, the call to action band and the newsletter form.

## The post page

1. The trail, the section label, the title, the dek, the baker and the date, beside the lead picture on wide screens.
2. **The ticket:** the four facts in ruled cells, each a small label over a large value, under a perforated line.
3. **Ingredients first:** the first lists of the post are drawn as an ingredients card on the kraft surface, each line ruled like an index card, in two columns on wide screens. Group headings such as "For the dough" sit inside it.
4. **The method:** the numbered list as steps, each number large in the heading face in a square, with a rule between steps.
5. The rest of the post in one column of 60 to 70 characters, with the contents beside it on wide screens.
6. Tags, share links, the author box, then related recipes as index cards.

A baking note has no ticket and no ingredients card; its lists stay plain.

## Other pages

- **Lists** (a section, a tag, the archive): the name and intro on a ruled header, then the posts as index cards.
- **Bakers** and **a baker's page:** the shared pages, with Pantry's cards.

## Type pair

- Headings: **Alegreya** 500 to 800, a warm, slightly calligraphic serif with the feel of an old cookery book.
- Text: **Alegreya Sans** 400 and 700, its humanist sans partner, clear at 19 px and good with figures such as 450 g.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files and self-hosted in the sample brand. The pack's own default uses the system's serif and sans.

## Palette

Flour, kraft paper and cherry:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f1` | flour |
| surface | `#f2eadb` | kraft, for the ingredients card and the tabs |
| text | `#2b211a` | 14.8:1 on background, 13.2:1 on surface |
| muted | `#5c4f44` | 7.5:1 on background, 6.6:1 on surface |
| link | `#9b2c2c` | 7.1:1 on background, 6.3:1 on surface |
| accent | `#9b2c2c` | white text on it 7.5:1 |
| border | `#e2d7c3` | |

Shapes are square, with no rounded corners and no shadows. Rules are thin, and the index card's header line is the accent.

## Signature device

**The tear-off ticket.** Every recipe carries its facts on a ticket: a dashed perforation line, then ruled cells, each a small capital label over a bold value. A card's ticket holds the time and the yield. The post's ticket holds all four facts, and the ingredients card under it repeats the index card's ruled lines. A reader sees how long a bake takes and what it makes before reading a word of the story.

## How it differs from the existing templates

- **Harbor** is a busy cooking blog: an indigo, rounded look, a newsletter in the masthead, a featured post with a column beside it, and facts as small pill tags with icons. Pantry is square, warm and quiet: facts on a ruled ticket, sections as recipe box tabs, and a post that opens on its ingredients.
- **Vital** puts a routine's time in a round ring and draws its steps on a soft track. Pantry has no rings: its facts sit in ruled cells and its method steps in square numbers.
- **Compass** opens a guide with key points and a dashed trail. Pantry opens a recipe with its ingredients card.
