---
title: "What the linter checks"
order: 65
---

# What the linter checks

Every message from `khalid lint`, the editor and the publish check links one
section of this page. Each section lists the rules it explains, with the
three sentences the writer sees: what is wrong, why it matters and what to
do. A site turns a rule up, down or off in `lint.toml`.

This page is made from `crates/khalid-lint/catalogue.toml`. Change a message
there, and the lint tests rewrite this page.

## Answer capsule

- `answer-capsule`: The first paragraph under a question-style heading is not a standalone answer of 40 to 60 words. Answer engines lift the first complete answer under a question, and readers scan for it too. Write the whole answer in one paragraph of 40 to 60 words directly under the heading, before any detail or hedging.

## Author proof

- `no-about-page`: The site has no About or contact page. A reader who cannot find who writes here trusts it less, and an engine citing the site has no author to credit. Add an About or a contact page, with the authors' names and how to reach them.

## Citing sources

- `all-outbound-nofollow`: Every link from this post to another website tells Google not to follow it. A post that vouches for none of its sources looks less trustworthy to readers and Google. Remove `nofollow` from the links to sources you trust, and keep it only for paid or untrusted links.
- `no-outbound-link`: This post links to no other website. Links to good sources show readers and Google where your facts come from. Add a link to a trusted source that backs up a claim in the post.

## Collections

- `case-study-incomplete`: This case study leaves out its client, its outcome or its date, as named below. A study without them reads as a story, so readers and search engines take it as weaker proof. Add the missing `client:`, `outcome:` or `date:` line to the study's front matter.
- `changelog-entry-undated`: This changelog entry has no date. The changelog lists entries by month, so an undated entry drops to the end under no month. Add a `date:` line to the entry's front matter, such as `date: 2026-11-10`.
- `overlapping-collections`: Two collections, the folders that group your posts, could give two posts the same address. Only one post can live at an address, so the other could vanish from your site and from Google. Change the `permalink` of one collection in `routes.toml` so their addresses differ.

## Comparison pages

- `comparison-answer-late`: The opening paragraph does not name both things the title compares. Readers and AI search tools look for the verdict first, and a page that hides it loses them. Write an opening paragraph that names both choices and says which one suits which reader.
- `comparison-without-table`: This post weighs choices against each other but has no table that sets them side by side. Readers scan a table faster than paragraphs, and AI search tools quote its rows as they stand. Add a table with one column per choice and one row for each point you compare.
- `fewer-picks-than-title`: The title promises more picks than the post gives a heading of their own. Readers who count fewer picks than the title promised stop trusting the page. Add a heading for each missing pick, or change the number in the title.
- `review-without-rating`: This review has no review block with your rating. Without the block, search results cannot show your score, and readers must hunt for your verdict. Add a review block with the item, your score and a short verdict.

## Descriptions

- `duplicate-description`: Another post on this site has the same description. A shared description makes two posts look the same in search results. Rewrite the description so it sums up this post alone.
- `long-description`: This post's description is longer than search results can show. Google cuts long descriptions short, so the end of yours may never be read. Shorten the description to one or two short sentences.
- `no-description`: This post has no description. Search results show a description under the title, so without one Google picks any sentence. Add `description:` to the header with one sentence that sums up the post.
- `short-description`: This post's description is shorter than this site sets. A very short description leaves Google to pick other words from the post for search results. Write one or two sentences that say what the reader gets from the post.

## Duplicate content

- `near-duplicate`: This post shares most of its text with another post on your site. Google shows only one of two near copies, so the two posts compete and both lose. Merge the two posts into one, or rewrite this one to cover a different angle.

## Fact registry

- `fact-checked-in-future`: This post uses a fact from `facts.toml` whose `checked` date has not come yet. A check that has not happened hides the fact from the reminder to check it again. Change the fact's `checked` date in `facts.toml` to the day you last checked it.
- `fact-not-checked`: This post uses a fact from `facts.toml` that has not been checked for a long time. A price, count or date that no longer holds misleads readers and the engines that quote your post. Update the fact in `facts.toml` after checking it, and set its `checked` date to today.
- `fact-old-value`: This post gives a value that the site's fact list says has changed. Two different values for one fact make readers and search engines trust the whole site less. Replace the old value with the current one from `facts.toml`.
- `fact-unknown`: This post shows a `[[fact:...]]` that `facts.toml` does not have. The page would show a gap where readers expect the value. Add the fact to `facts.toml`, or change the key to one the file has.

## Focus keyword

- `duplicate-focus-keyword`: An older post on your site already uses this focus keyword. Two posts aimed at the same search compete, and Google may show neither of them well. Change `focus_keyword:` to a different angle, or merge this post into the older one and redirect it.
- `focus-not-at-title-start`: Your focus keyword sits late in the title search results show. Searchers scan the first words of each result, and a long title may be cut before the keyword. Move the words of `focus_keyword:` toward the start of `seo_title:`, or of `title:` when there is none.
- `focus-not-in-address`: The web address of this post does not hold your focus keyword. Searchers see the address in results, and a clear one tells them what the page is about. Change `slug:` to the words of `focus_keyword:` before you publish, because a later change needs a redirect.
- `focus-not-in-description`: The description does not hold your focus keyword. Google shows the description under the title and marks the words a searcher typed in bold. Rewrite the description so it holds the words of `focus_keyword:` once.
- `focus-not-in-first-paragraph`: The first paragraph does not hold your focus keyword. Readers decide in the first lines whether the post answers their search, and so does Google. Use the words of `focus_keyword:` in the first paragraph, where it reads naturally.
- `focus-not-in-h1`: The main headline on the page does not hold your focus keyword. A reader who arrives from search looks for the words they typed in the headline before reading on. Rewrite `title:` so it holds the words of `focus_keyword:`, as your search title does.
- `focus-not-in-image-alt`: None of the image descriptions holds your focus keyword. Google Images reads the descriptions to know what an image shows and which searches it fits. Describe one image in words that include `focus_keyword:` if the image really shows it.
- `focus-not-in-subheading`: None of the subheadings holds your focus keyword. Subheadings show readers and Google how the post is built around its topic. Change one subheading so it holds the words of `focus_keyword:`.
- `focus-not-in-title`: The title search results show does not hold your focus keyword. The title is the headline in search results, and Google reads it first to learn the topic. Rewrite `seo_title:`, or `title:` when there is none, so it holds the words of `focus_keyword:`.
- `focus-only-filler`: Your focus keyword holds only small words, such as "how to", that name no topic. Khalid and Google cannot tell what the post is about from words that appear in every text. Add the words that name your topic to `focus_keyword:`, such as "how to bake bread".
- `focus-subheading-share`: Too few or too many of your subheadings hold your focus keyword. Readers and Google use subheadings to see the topic, and the same words in all of them read as repetitive. Use the focus keyword or a synonym in about a third to three quarters of the subheadings.
- `focus-too-long`: Your focus keyword holds more than four words that carry meaning. Few people search for a long phrase in exactly that order, so the post is checked against a search almost nobody makes. Shorten `focus_keyword:` to the two to four words a reader would type, and move the rest to `secondary_keywords:`.
- `no-focus-keyword`: This post has no focus keyword, the phrase you want it to be found for. Without one, Khalid cannot check that the title, description and text are about one topic. Add `focus_keyword:` to the header with the words a reader would search for.
- `secondary-keyword-unused`: One of your secondary keywords never appears in the text. A close phrase only helps the post when readers and Google can find it in the text. Use the words this message names at least once, or remove them from `secondary_keywords:`.
- `secondary-not-in-first-paragraph`: The first paragraph does not hold this secondary keyword. Readers who searched for the close phrase look for it in the opening lines. Use the named words in the first paragraph if they read naturally there.
- `secondary-not-in-subheading`: None of the subheadings holds this secondary keyword. A subheading with the close phrase shows readers where the post covers it. Rewrite one subheading so it holds the named words, if a section is about them.
- `secondary-not-in-title`: The title search results show does not hold this secondary keyword. A close phrase in the title helps the post show up for that search too. Use the named words in `seo_title:` or `title:` if they fit, or leave the title as it is.

## Forms

- `form-without-destination`: A form in this post sends its answers nowhere. Readers who fill it in never hear back, so the visits search brings you are lost. Give this form an email address or a service such as Zapier to send its answers to.

## Glossary

- `glossary-term-not-linked`: This post uses a term your glossary explains, without linking to it. A link to the definition helps new readers and shows Google how your pages relate. Link the first use of the term to its glossary entry.

## Headings

- `heading-order`: This heading skips a level, such as a `####` right after a `##`. Screen readers and Google follow headings like a table of contents. Change this heading to one level below the heading before it.

## Headings promise

- `heading-agrees`: Two headings in the post say near the same thing. A repeated heading leaves a reader unable to tell the sections apart, and an engine unsure which to lift. Give each heading its own words, or merge the two sections.
- `heading-shape`: A heading promises kinds or a comparison, but no list or table follows it. The heading sets a promise the body must keep; a list or a table is the shape a reader can scan. Turn what follows into a list for kinds, or a table for differences.

## Image descriptions

- `missing-alt`: This image has no description. People who cannot see the image, and Google, rely on the description to know what it shows. Add a short description of what the image shows.
- `no-image`: This post has no image. A picture helps readers understand and share a post, and it can bring visitors from image search. Add an image to the text or set `image:` in the header, with a short description.

## Inclusive language

- `non-inclusive-term`: This post uses a word that can leave out or hurt some readers. Readers who feel left out stop reading, and many sites now use other words. Replace the word with one of the suggested words if it fits what you mean.

## Internal links

- `broken-internal-link`: A link in this post opens an address on this site that has no page. Readers who follow it land on a page that does not exist, and search engines count it as a dead end. Change the link to the address of an existing page, or add a redirect from this address.
- `no-internal-link`: This post links to none of your other posts. Internal links are how readers and Google find the rest of your site. Add one link to a related post on your site.
- `nofollow-internal-link`: A link to another page of your own site tells Google not to follow it. Links between your pages help Google find them and judge their value, and this link passes nothing. Remove `nofollow` from the link, or add its host to `internal_domains` under `[seo.links]` in `site.toml`.
- `one-inbound-link`: Only one other post links to this post. A post with few links pointing to it looks less important to Google. Link to this post from two or three more related posts.
- `orphan-post`: No other post on your site links to this post. Readers and Google find posts by following links, so this one is hard to find. Add a link to this post from one or two related posts.
- `same-target-twice`: This post links to the same page more than once. Google counts only the first link to a page, so the others add nothing. Remove the extra link, or point it at another useful page.
- `too-many-links`: This post holds more links than your site's limit. Google shares a page's value across its links, so each one counts for less, and readers skim past them. Cut the links that do not help the reader, or split the post into two.

## Keyword density

- `focus-distribution`: Your focus keyword appears in only a few parts of the text. A post that names its topic only at the start or the end reads as being about it only in places. Use the words of `focus_keyword:` or a synonym in the parts of the text that lack it, where they fit.
- `focus-too-often`: Your focus keyword appears so often that the post reads as if written for machines. Google treats heavy repetition as spam and may show the post less. Replace some repeats of `focus_keyword:` with other words, such as a close phrase or a pronoun.
- `focus-too-rare`: Your focus keyword appears too rarely in the text. A post that names its topic only once or twice reads to Google as being about something else. Use the words of `focus_keyword:` a few more times where they fit, about once every 200 words.
- `secondary-too-often`: This secondary keyword appears so often that the text reads as if written for machines. Google treats heavy repetition as spam, whichever phrase is repeated. Replace some repeats of the named words with other words or a pronoun.
- `secondary-too-rare`: This secondary keyword appears only rarely in the text. A close phrase used once in a long post says little about what the post covers. Use the named words a few more times where they fit, about once every 200 words.

## Keyword stuffing

- `keyword-stuffing`: One phrase appears so often that the post reads as if written for machines. Google treats heavy repetition as spam and may show the post less. Rewrite some of those sentences with other words, or cut the repeats.

## Link text

- `empty-link-text`: This link has no words a reader can see or hear. People using screen readers and Google cannot tell where it goes. Add words to the link, or describe the image inside it.
- `link-text-is-own-focus`: These link words hold this post's own focus keyword, but they open another page. Link words tell Google which page answers a phrase, so these send your phrase elsewhere. Rewrite the link to name the other page's topic, and keep your focus keyword for this post.
- `link-text-misses-target`: These link words share no word with the title of the page they open. Google reads link words as a label for the page, and this label does not match it. Rewrite the link with words from the title of the page it opens.
- `repeated-link-text`: Too many links to this page use these exact words. Varied link words tell Google more about what the page covers. Rewrite these link words to fit what this post says about the page.
- `reused-link-text`: These link words already point to a different page on your site. Google reads link words as a label, and one label should mean one page. Change the link words, or point the link at the same page as before.
- `vague-link-text`: This link uses vague words such as "click here" or "read more". Readers and Google use the link words to know where a link goes. Rewrite the link so its words name the page it opens.

## Page or section

- `post-fits-section`: This short post covers what a section of another post already covers. Two pages on one subject compete for the same searches, and the short one rarely wins. Move what is new here into that section, then redirect this post's address to that post.
- `section-repeats-post`: A section covers the whole subject of another post on the site. Two pages on one subject compete for the same searches, and neither does as well as one would. Shorten the section to a summary and link to the post this message names.
- `section-wants-own-page`: A long section never names the post's subject, so it reads as a second topic. A page on one subject is easier to find and to quote, and the side topic gets no page of its own. Move the section to its own post and link to it, or tie it to the post's subject in its heading and text.

## Plain writing

- `long-paragraph`: A paragraph runs past the word limit, 150 words unless this site sets another, so it holds more than one idea. Readers lose the thread in a wall of text, and the paragraph's main point is hard for you or an engine to name. Split it at each change of idea, into shorter paragraphs of one idea each.
- `plain-answer`: An answer paragraph uses hedge words, vague openers or a chain of trailing which clauses. Answer engines lift the clearest sentence they can find, and hedged text reads as unsure. Write the claim outright in the answer paragraph and drop the hedge.

## Post authors

- `author-without-profile`: This post names an author who has no profile on the site. The byline links to the author's page, and without a profile that page does not exist. Add `content/<language>/authors/<author>.md` with `name:` and `slug:` in its header.
- `no-author`: This post has no author. People and search engines trust content with a named writer. Add `author:` to the header and give that author a profile.
- `no-reviewer`: This post is in a category that needs a reviewer, and names none. Readers and search engines trust health or money advice more when an expert checked it. Add `reviewed_by:` to the header with the short name of the author who checked the post.

## Post briefs

- `brief-link-missing`: The post's brief lists a page that the post does not link to. The brief planned this link so readers and search engines reach the related page from here. Link to the page where the post mentions its subject, or remove it from the brief.
- `brief-link-unknown`: The post's brief plans a link to a post your site does not have. A link to a missing post takes readers to a page that does not exist. Change the address in the brief to the post's current one, or write that post first.
- `brief-question-unanswered`: The post's brief lists a question that no heading in the post asks. A reader or an answer engine looking for that question cannot find where the post answers it. Add a heading that asks the question, with the answer right under it, or remove it from the brief.
- `brief-term-missing`: The post's brief lists a term that the post never uses. The brief named the things a full answer covers, and this one is left out. Use the term where the post explains it, or remove it from the brief.

## Post dates

- `no-date`: This post has no date. Google and readers use dates to judge freshness. Add `date:` to the header.

## Post length

- `pillar-min-words`: This pillar post, the main post of its topic group, is shorter than the minimum this site sets for pillar posts. The main post should cover the whole topic, so readers and Google treat it as the best page on it. Add the parts of the topic the group's other posts cover, with a link to each of them.
- `thin-post`: This post is shorter than the minimum this site sets. Short posts rarely answer a question fully, so Google seldom shows them. Add the details, examples or steps a reader needs to act on the topic.

## Post titles

- `duplicate-title`: Another post on this site has the same title. Google may show only one of the two, and readers cannot tell them apart. Change the title so it says what makes this post different.
- `long-title`: This post's title is longer than search results can show. Google cuts long titles short, so readers may not see the end of yours. Shorten the title and keep the words that matter most at the start.
- `many-headlines`: This post has more than one main headline. A page needs exactly one headline, so readers and Google know its topic. Change each extra `#` headline in the text to a `##` subheading.
- `no-title`: This post has no title. A page needs exactly one headline, and Google shows it in search results. Add `title:` to the header.
- `short-title`: This post's title is shorter than this site sets. A very short title says little about the post, so fewer people choose it in search results. Add the words that tell a reader what they will learn.
- `title-too-wide`: The title search results show is wider than Google shows on a desktop result. Google cuts a title at about 600 pixels, so readers may not see the end of yours. Shorten `seo_title:`, or `title:` when there is none, and keep the words that matter most at the start.

## Readability

- `complex-words`: More than one in ten words in this post are long words readers rarely meet. Rare long words slow every reader down, and many readers skip what they cannot follow. Replace the long words you can with short everyday ones, such as "use" for "utilise".
- `few-transitions`: Fewer than three in ten sentences use a linking word such as "because", "so" or "for example". Linking words show how one idea leads to the next, so readers follow the thread. Add linking words where one sentence explains, contrasts or follows from the one before.
- `hard-to-read`: This post is hard to read, because its sentences are long and many of its words are long. Readers who struggle leave early, and Google favours pages that answer people quickly. Split long sentences, and use shorter everyday words where you can.
- `long-section`: A stretch of this post runs past 300 words without a subheading. Readers scan subheadings to find their part, and a long stretch hides what it covers. Add a `##` subheading where the topic of this stretch changes.
- `long-sentences`: More than a quarter of this post's sentences run past 20 words. Readers lose track of long sentences, most of all on a phone. Split the longest sentences in two, each with one idea.
- `passive-voice`: More than one in ten sentences are in the passive voice, such as "the post was written". Passive sentences hide who does what, and they are longer and harder to read. Rewrite these sentences so the one who acts comes first, such as "we wrote the post".
- `repeated-openers`: Three or more sentences in a row start with the same word. Sentences that all start alike read as monotonous, and readers start to skim. Rewrite one or two of these sentences so they start with another word.

## Redirects

- `redirecting-link`: A link in this post opens an old address that now sends readers on to another page. Each detour slows readers down, and Google trusts a direct link to the current address more. Change the link to the post's current address.

## Refreshing posts

- `stale-pillar`: This pillar post, the main post of its topic group, has not been updated for longer than this site allows. The main post should be the best and most current page on its topic, for readers and Google. Refresh this post: add what the group's newer posts cover and set `updated:` to today.
- `stale-post`: This post has not been updated for a long time. Readers and Google trust recent information more, and old facts may now be wrong. Refresh this post: check its facts, update what changed and set `updated:` to today.
- `updated-date-wrong`: This post's `updated:` date is before its publish date or still to come. A date that cannot be true makes readers and search engines doubt the whole page. Change `updated:` to the day you last changed what the post says, or remove it.

## Review queue

- `review-due`: This post has not been reviewed in over six months. A post nobody has looked at lately drifts out of date, and a reader trusts a claim less when it is stale. Update the reviewed date in the front matter after looking the post over; only a real change sets the updated date.

## Secure links

- `insecure-internal-link`: A link to your own site uses an address that starts with `http://`. Google prefers secure pages, and each click takes a slower detour before the secure page opens. Change the address to start with `https://`, or use the path alone, such as `/blog/my-post/`.

## Sister sites

- `cross-site-link`: This post links to a sister site that runs on the same Khalid server. Google may read sister sites that link to each other as a link scheme and trust both less. Remove the link, or keep it only where readers need it and mark it `nofollow`.

## Site structure

- `too-deep`: Readers need too many clicks to reach this post from your home page. Google visits pages near the home page more often, and readers give up on long paths. Link to this post from your home page, a menu or a popular post.

## Sourced statistics

- `sourced-statistics`: A percentage, a sum of money, a figure in millions, a multiple or a study's result has no linked source and no year. A reader, and an engine citing your post, needs to know where a statistic comes from and whether it still holds. Link the statistic to its source, or write the year it held beside it, such as 47 percent (2024).

## Structured data

- `invalid-structured-data`: The search result details on this page miss a property Google needs. Google ignores details it cannot read in full, so the post loses its richer search result. Add the property this message names to the block it belongs to.
- `schema-markup-mistake`: The search result details you wrote by hand have a name, a value or a detail that search engines cannot use. Search engines skip what they cannot read, and Google may ignore markup that describes what readers cannot see. Change what this message names in the post's `schema` field, or press its fix button in the editor.

## Topic clusters

- `cluster-without-pillar`: This post belongs to a topic group but does not link to the group's main post. Links to the main post show Google which page covers the topic best. Add a link to the main post of this topic group.

## Translations

- `missing-translation`: This post has no version in one of your site's languages. Readers of that language cannot find this post, and Google cannot point them to it. Write the translation and give both versions the same `translation:` key.

## Web addresses

- `long-slug`: This post's web address is long. Short addresses are easier to read, share and remember. Shorten `slug:` to the few words that name the topic, before you publish.
- `slug-date`: This post's web address holds a year. A year in the address makes the post look old once that year has passed. Remove the year from `slug:` before you publish.
- `slug-stopwords`: This post's web address holds filler words such as "the" or "of". Filler words make the address longer without telling readers or Google anything. Remove the filler words from `slug:` before you publish.
- `unicode-slug`: This post's web address holds letters outside a to z, but the site keeps plain addresses. Accented letters turn into codes such as %C3%A9 when the address is copied into an email or a chat. Change `slug:` to letters a to z, digits and hyphens, or allow accents with `slugs = "unicode"` in `site.toml`.

## Wikilinks

- `ambiguous-wikilink`: This `[[link]]` names both a post and a glossary term, and it links the post. Readers who expect the term's short definition land on a whole post instead. Write `[[glossary:slug]]` to link the term, or keep `[[slug]]` if you meant the post.
- `broken-wikilink`: This `[[link]]` matches no post, page or glossary term in this language. Readers who follow it land on a page that does not exist. Change it to the address of an existing page, or remove the brackets.
