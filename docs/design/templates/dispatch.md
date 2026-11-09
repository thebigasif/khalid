# Dispatch: design brief

Dispatch is an original Khalid template for an email newsletter whose website has one first job: to win sign-ups. A visitor who arrives from a shared link should see what the newsletter is, what they get and how to join before anything else. The latest issues come second, as proof that the letter is worth it.

## Who it is for

- A writer or a small team who sends a weekly or monthly newsletter and keeps every issue on the web.
- A club, shop or local group whose website mostly exists to grow a mailing list.
- Anyone who measures their site by new subscribers, not by page views.

## The content model

Dispatch adds no front matter. It reads what Khalid already has:

- **An issue is a post.** Its first fact is its number, such as `Issue` and `178`. Without facts, the issue shows its date in that place.
- **Topics are categories**, such as growing food or reader questions. Tags are smaller themes.
- **Writers are authors.**
- **The sign-up form** is Khalid's own, which a served site with `[mail]` takes. A static site cannot take a sign-up, so Dispatch shows the site's call to action (`[template.cta]`) as the sign-up button there, such as a link to the newsletter's page or to another sign-up service.

## The front page

1. **The sign-up first.** On the left: when the letter goes out (`cadence`), the newsletter's name large, its promise (`tagline`), up to three lines on what each issue holds (`promises`), and the reply slip: the email field and button, or the call to action, with the cadence note and a link to the feed. Under it, optional proof numbers from `hero_stats`, such as readers and issues sent.
2. **The latest issue** beside it: its picture, its stamp, the date, the title, the summary and a link to read it.
3. **Recent issues** as ruled rows: the stamp, the date and topic, the title, the summary and a small picture.
4. **Topics** as a row of plain links.
5. **A second reply slip** at the end of the page, for readers who scrolled through the issues first.
6. Numbered page links.

On a phone everything stacks in that order, so the form is on the first screen.

## The post page

1. The stamp with the issue's number, "Sent on" and the date, and the topic.
2. The title, the dek and the byline. The lead picture under them.
3. The text in one reading column of 60 to 75 characters.
4. **The reply slip** straight after the text: "Get the next issue by email", for the reader who just finished one.
5. Tags, share links, the author box, then more issues as rows.

## Other pages

- **Lists** (a topic, a tag, the archive): the list's name and intro, the issues as rows, then the reply slip.
- **Writers** (`/authors/`): the shared people grid.
- **A writer:** their mark, role and bio, then their issues as rows.

## Type pair

- Headings: **Gloock** 400, a high-contrast display serif with the weight of a printed masthead.
- Text: **Schibsted Grotesk** 400 and 700, a plain, sturdy grotesque that stays readable in long issues and in forms.
- The stamp's label and the micro labels: the text face in small capitals with wide spacing.

Both fonts are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files, and self-hosted in the sample brand. The pack's own defaults are system fonts: a book serif for headings and the system sans for text.

## Palette

The pack's own default is postal red on warm paper:

| Token | Value | Check |
|---|---|---|
| background | `#fbf8f2` | |
| surface | `#f1eadc` | |
| text | `#1e1b17` | 16.2:1 on background |
| muted | `#5d564b` | 6.8:1 on background, 6.1:1 on surface |
| link | `#a3261c` | 7.0:1 on background, 6.2:1 on surface |
| accent | `#b02a1f` | 6.2:1 on background; white text on it 6.6:1 |
| border | `#ddd4c4` | |

The demo's sample brand, The Saturday Gardener, uses a deep leaf green (`#3d6b2f`, 5.7:1; link `#35612a`, 6.6:1) with ink `#1f2a1f` on oatmeal (`#f6f4ec`).

## Signature device

**The stamp and the reply slip.** Every issue carries its number in a small postage stamp: a tinted square with a dashed inner edge, the label in small capitals over the number in the heading face. The same stamp marks the issue in every list, on the latest issue and at the top of the issue itself. The sign-up box is a reply slip: a ruled card with a dashed cut line around it, as on a coupon you would tear off and post back. It sits at the top of the front page, after every issue and at the end of every list, so a reader is never more than one scroll from joining.

## How it differs from the existing templates

- **Ticker** is a markets newsletter built around data: a tape of figures, bar charts and tables. Dispatch has no data and leads with the sign-up, not the numbers.
- **Hub** is a link-in-bio page with a stack of link buttons. Dispatch has one action, the sign-up, and a full archive of issues.
- **Versions** has a small update box beside a changelog. In Dispatch the sign-up is the hero, and it repeats after every issue.
- **Wire** and **Ledger** list posts as headlines. Dispatch lists issues by number, each with its stamp.

Dispatch reuses the shared hero, figure strip and people grid parts. Its stamp, issue rows and reply slip live in its own parts file, since they are its signature.
