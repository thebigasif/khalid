import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, flagOf, lines, listOf, partOf, textOf } from "./parts";

/** The AI crawlers the screen has a switch for, as they name themselves in robots.txt. */
export const AI_CRAWLERS = Object.keys(seoWords.crawlers.bots);

/** Which crawlers may read the blog, a switch per AI crawler, and the AI info page. */
export function Crawlers({ site, onSave }: SectionProps) {
  const words = seoWords.crawlers;
  const crawlers = partOf(site.seo, "crawlers");
  const page = partOf(site.seo, "ai_page");
  const blocked = listOf(crawlers, "block");
  const [bots, setBots] = useState(() => AI_CRAWLERS.filter((bot) => blocked.includes(bot)));
  const [published, setPublished] = useState(!flagOf(page, "off", false));
  const { texts, set } = useTexts({
    others: blocked.filter((name) => !AI_CRAWLERS.includes(name)).join("\n"),
    allow: listOf(crawlers, "allow").join("\n"),
    disallow: listOf(crawlers, "disallow").join("\n"),
    contact: textOf(page, "contact"),
    covers: textOf(page, "covers"),
    cite: textOf(page, "cite"),
  });
  const turn = (bot: string, on: boolean) =>
    setBots(on ? [...bots, bot] : bots.filter((each) => each !== bot));

  const save = () => {
    const block = [...AI_CRAWLERS.filter((bot) => bots.includes(bot)), ...lines(texts.others)];
    const allow = lines(texts.allow);
    const disallow = lines(texts.disallow);
    const off = published ? undefined : true;
    const { contact, covers, cite } = texts;
    return onSave({
      crawlers: cleaned({ ...crawlers, block, allow, disallow }),
      ai_page: cleaned({ ...page, off, contact, covers, cite }),
    });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <fieldset>
        <legend>{words.ai}</legend>
        <span className="hint">{words.aiHint}</span>
        <button type="button" className="btn" onClick={() => setBots(AI_CRAWLERS)}>
          {words.blockAll}
        </button>
        <div className="bots">
          {AI_CRAWLERS.map((bot) => (
            <Check
              key={bot}
              id={`seo-bot-${bot}`}
              label={words.block(bot)}
              hint={words.bots[bot]}
              checked={bots.includes(bot)}
              onChange={(on) => turn(bot, on)}
            />
          ))}
        </div>
      </fieldset>
      <TextField
        id="seo-others"
        rows={3}
        label={words.others}
        hint={words.othersHint}
        value={texts.others}
        onChange={set("others")}
      />
      <div className="pair">
        <TextField
          id="seo-disallow"
          rows={3}
          label={words.disallow}
          hint={words.disallowHint}
          value={texts.disallow}
          onChange={set("disallow")}
        />
        <TextField
          id="seo-allow"
          rows={3}
          label={words.allow}
          hint={words.allowHint}
          value={texts.allow}
          onChange={set("allow")}
        />
      </div>
      <h3>{words.page}</h3>
      <Check
        id="seo-ai-page"
        label={words.pageOn}
        hint={words.pageOnHint}
        checked={published}
        onChange={setPublished}
      />
      {published
        ? (["contact", "covers", "cite"] as const).map((key) => (
            <TextField
              key={key}
              id={`seo-ai-${key}`}
              label={words[key]}
              hint={words[`${key}Hint` as const]}
              value={texts[key]}
              onChange={set(key)}
            />
          ))
        : null}
    </SectionForm>
  );
}
