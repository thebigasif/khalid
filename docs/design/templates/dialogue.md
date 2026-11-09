# Dialogue: design brief

Dialogue is an original Khalid template for an interview show: a podcast, a radio programme or a written interview series where every episode is a conversation with one guest. The guest is the headline. A reader should see who the guest is and hear their voice in one line before they see anything else.

## Who it is for

- A host who talks with one guest an episode, such as makers, founders, scientists or authors.
- A team that publishes each conversation with show notes and a transcript.
- A written interview series that never records sound at all.

## The content model

Dialogue adds no front matter. It reads what Khalid already has:

- **An episode is a post with facts,** read by position: 1 the guest's name, 2 who they are (their work and town), 3 a short fact such as the length, 4 one line the guest said, at most 40 characters. The labels are the writer's own.
- **A note is a post without facts,** such as a season announcement or listener questions. Notes show in a dated feed, not among the conversations.
- **Topics are categories.** Tags are the topics a conversation covers, listed on the episode as "Topics in this conversation".
- **Hosts are authors,** with their role.
- **Pull quotes are the post's blockquotes.** A transcript is plain paragraphs that start with the speaker's name in bold, often inside a toggle; Dialogue sets those names as small labels.
- The sound player is Khalid's own `::audio` block in the post.

## The front page

1. **Hero:** the show's name, its `tagline`, and up to four `listen_links` as plain outlined links, such as the podcast apps and the feed.
2. **This week's guest:** the newest episode, large. Its picture on one side; on the other "In conversation with", the guest's name very large, who they are, their line as the quote line, then the episode title, date, length and topic.
3. **Conversations:** the other episodes as guest rows, one under another. Each row has a small picture, the guest's name and role in a column of their own, then the title, the quote line and the date, length and topic.
4. **Topics:** each category as a ruled link.
5. **Notes:** the newest notes as a dated feed. `notes_count` sets how many.
6. The page links, the call to action band and the newsletter form.

## The post page

1. The topic as a small label, the title and the dek.
2. **The name plate:** two cells side by side, "Host" with the author's mark and name, and "Guest" with the guest's name and who they are, split by a rule. Under it the date and the length.
3. The guest's line as the quote line, then the lead picture.
4. The text in one column: show notes, the player, chapters and the transcript. Blockquotes are pull quotes with the same mark as the quote line. A transcript's bold speaker names are small capitals in the accent.
5. "Topics in this conversation" (the tags), share links, the author box, then related episodes as guest rows.

A note has no name plate and no quote line.

## Other pages

- **Lists** (a topic, a tag, the archive): the name and intro, then the posts as guest rows.
- **Hosts** (`/authors/`): the shared people grid.
- **A host:** their mark, role and bio, then their conversations as guest rows.

## Type pair

- Headings and the quote line: **Petrona**, a lively text serif with a warm italic, 400 to 700 roman and 400 italic.
- Text: **Schibsted Grotesk** 400 to 700, a plain, sturdy sans that reads well at 18 px.

Both are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files and self-hosted in the sample brand. The pack's own default uses Charter or the system's serif, and the system's sans.

## Palette

The pack's default is a brick red on warm paper, like a studio's red "on air" light:

| Token | Value | Check |
|---|---|---|
| background | `#f8f6f1` | |
| surface | `#ece6db` | |
| text | `#1f1a17` | 16.0:1 on background, 13.9:1 on surface |
| muted | `#5a514b` | 7.2:1 on background, 6.2:1 on surface |
| link | `#9c2f1b` | 6.9:1 on background, 6.0:1 on surface |
| accent | `#9c2f1b` | white text on it 7.4:1 |
| border | `#d9d0c3` | |

The demo's sample brand, Open Studio, is a deep teal (`#1d5c55`, 7.2:1 on its background) on a cool paper page (`#f6f7f4`) with a pale green surface (`#e3ebe7`).

## Signature device

**The quote line.** Every conversation is led by its guest's own words: one line in large italic serif, hung behind a big opening quote mark in the accent. It sits under the guest's name on the front page, in every guest row, at the top of each episode and on every pull quote in the text. The second device is **the name plate** on each episode: host and guest as two equal cells, so the page reads as a conversation between two people, not an article by one.

## How it differs from the existing templates

- **Frequency** (built at the same time) is a podcast template led by the episode and its player. Dialogue is led by the person: the guest's name and their words come first, the player sits in the text, and there are no episode numbers or waveforms.
- **Channel** leads with video thumbnails in a grid. Dialogue has no grid of pictures; its conversations are rows led by a name.
- **Folio** is one writer's essays. Dialogue always shows two people, host and guest.
- **Summit** lists speakers in a people grid. Dialogue's guests are not authors: each guest lives in their episode's facts, so a show with a new guest every week needs no author pages for them.

The guest rows are a new shared section part (`sections/guest_rows.html`), since a magazine's interviews, a conference's talks and a research group's seminar series can show a person, their role and one line the same way.
