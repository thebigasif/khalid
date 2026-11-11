# Content analysis: the keyword and readability checks

Khalid checks every post the way the Yoast SEO plugin does, and tells you what to change in plain words. This page lists the checks, the settings behind them and the numbers the admin shows. Every check is a lint rule: you can change its level or switch it off in `lint.toml`, and you can change its numbers in the `[thresholds]` table of the same file.

A check has one of four levels:

- `block`: the post cannot be published until you fix it.
- `warn`: you see the message and lose 5 points of the score; the post can still go live.
- `info`: a tip. It costs no points and never stops a publish.
- `off`: the check does not run.

None of the checks on this page blocks a post. To change a level, name the rule in `lint.toml`:

```toml
[rules]
passive-voice = "info"
focus-not-at-title-start = "off"

[thresholds]
paragraph-max-words = 225
```

## The focus keyword

The focus keyword is the phrase you want a post to be found for, set with `focus_keyword:` in the header. Khalid finds it in any word form: "internal links" counts for "internal link" and "Interne Links" for "interner Link". This works in English, German, French, Spanish, Italian, Portuguese and Dutch. Other languages compare the words as written.

Synonyms count as the keyword. List them in the header:

```yaml
focus_keyword: internal links
keyword_synonyms: [site links, links between posts]
```

| Rule | Level | What it checks | Setting |
| --- | --- | --- | --- |
| `focus-too-long` | warn | The keyword holds more than four words that carry meaning. Small words such as "to" do not count. | `focus-max-words`, 4 |
| `focus-only-filler` | warn | The keyword holds only small words, such as "how to". | |
| `focus-not-at-title-start` | info | The search title holds the keyword, but only after other words that carry meaning. "Why internal links matter" still starts with it. | |
| `focus-distribution` | warn | In a text of 300 words or more, more than half of its ten equal parts never use the keyword. | |
| `focus-subheading-share` | warn | Once a post has three or more `##` and `###` subheadings, under 30 or over 75 percent of them hold the keyword. | `focus-subheading-min-percent`, 30; `focus-subheading-max-percent`, 75 |
| `duplicate-focus-keyword` | warn | Another, older post in the same language uses the same keyword. A dashboard check over the whole site. | |

The checks that were there before, such as `focus-not-in-title` and `focus-too-rare`, now also match word forms and synonyms.

## Secondary keywords

Each entry of `secondary_keywords:` gets the main focus checks too, as tips:

| Rule | Level | What it checks |
| --- | --- | --- |
| `secondary-not-in-title` | info | The search title does not hold it. |
| `secondary-not-in-first-paragraph` | info | The first paragraph does not hold it. |
| `secondary-not-in-subheading` | info | No subheading holds it. |
| `secondary-too-rare` | info | It is used, but less often than the focus keyword's floor, `focus-min-per-thousand-words`. |
| `secondary-too-often` | info | It is used more often than the focus keyword's ceiling, `focus-max-per-thousand-words`. |

## The page

| Rule | Level | What it checks | Setting |
| --- | --- | --- | --- |
| `title-too-wide` | warn | The title search results show is wider than Google shows, measured in pixels of Arial at 20 pixels. | `title-max-pixels`, 600 |
| `long-title` | warn | The character count, now only for a title with characters the pixel table lacks, such as Japanese. | `title-max-chars`, 60 |
| `no-image` | warn | No image in the text and no `image:` in the header. | |
| `all-outbound-nofollow` | warn | Every link to another site carries `nofollow`. | |
| `pillar-min-words` | warn | A pillar post, the main post of a topic group, is shorter than the floor. | `pillar-min-words`, 900 |
| `stale-pillar` | warn | A pillar post has not been updated for months. A dashboard check. | `pillar-stale-months`, 6 |

## Readability

The readability checks read only your own prose. Code, quotes, tables and the parts Khalid writes itself, such as the table of contents or a bookmark card, do not count. The checks that measure a share of sentences or words wait until a post has 200 words of prose.

| Rule | Level | What it checks | Setting |
| --- | --- | --- | --- |
| `long-sentences` | warn | Over 25 percent of the sentences run past 20 words. | `sentence-max-words`, 20; `long-sentences-max-percent`, 25 |
| `long-section` | warn | Over 300 words without a `##` or `###` subheading. | `section-max-words`, 300 |
| `long-paragraph` | warn | A paragraph runs past 150 words. Before Khalid 1.3 the default was 225; a site that set its own number keeps it. | `paragraph-max-words`, 150 |
| `repeated-openers` | warn | Three or more sentences in a row in one paragraph start with the same word. | `repeated-openers`, 3 |
| `few-transitions` | warn | Under 30 percent of the sentences hold a linking word such as "because", "for example" or "zum Beispiel". | `transition-min-percent`, 30 |
| `passive-voice` | warn | Over 10 percent of the sentences are passive, such as "was written" or "wird gelesen". | `passive-max-percent`, 10 |
| `complex-words` | warn | Over 10 percent of the words are long and rare: in English over seven letters and not a name, in German over ten letters, and not an everyday word. | `complex-words-max-percent`, 10 |
| `non-inclusive-term` | info | A word that can leave out or hurt readers, with the words to use instead. The list is in `crates/khalid-lint/catalogue.toml`, under the rule's entry. | |
| `hard-to-read` | warn | The reading index, LIX, is above the limit. It now reads only your prose too. | `hard-reading-index`, 60 |

The passive voice, linking words and complex words are checked in English and German. In other languages those three checks stay quiet.

## Two scores and the numbers beside them

The admin shows two scores out of 100, as Yoast does:

- The **SEO score** counts every rule except the readability ones above.
- The **readability score** counts only the readability rules.

Beside them it shows the word count, the reading index (LIX) and up to ten prominent words: the words the post uses most, word forms counted together and small words left out. The lint result of one post carries these as `analysis` in its JSON:

```json
{
  "blocks": false,
  "findings": [],
  "score": { "value": 95, "lost": [{ "rule": "no-image", "points": 5 }] },
  "analysis": {
    "word_count": 640,
    "reading_index": 38,
    "readability": { "value": 100, "lost": [] },
    "prominent_words": [{ "word": "links", "uses": 9 }]
  }
}
```

## For the settings screen

Every number on this page is a field of `khalid_lint::Thresholds`, with its default and its reason in the field's documentation. The struct reads and writes the same kebab-case keys as `lint.toml`, so the SEO settings screen can show each value and save it back.
