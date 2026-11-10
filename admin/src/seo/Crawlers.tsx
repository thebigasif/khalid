import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, flagOf, lines, listOf, partOf, textOf } from "./parts";

const words = seoWords.crawlers;

/** The crawlers that copy posts to train AI models, as they name themselves in robots.txt. */
export const TRAINING_CRAWLERS = Object.keys(words.trainingBotsAbout);

/** The crawlers that fetch posts to quote and link them in AI answers. */
export const ANSWER_CRAWLERS = Object.keys(words.answerBotsAbout);

/** Every AI crawler the screen has a box for. */
export const AI_CRAWLERS = [...TRAINING_CRAWLERS, ...ANSWER_CRAWLERS];

/**
 * Which crawlers may read the blog: the AI training choice first, then a box per
 * AI crawler, other names and paths, and the AI info page.
 */
export function Crawlers({ site, onSave }: SectionProps) {
  const crawlers = partOf(site.seo, "crawlers");
  const page = partOf(site.seo, "ai_page");
  const blocked = listOf(crawlers, "block");
  const [training, setTraining] = useState(textOf(crawlers, "ai_training") === "block");
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
    const ai_training = training ? "block" : undefined;
    const allow = lines(texts.allow);
    const disallow = lines(texts.disallow);
    const off = published ? undefined : true;
    const { contact, covers, cite } = texts;
    return onSave({
      crawlers: cleaned({ ...crawlers, ai_training, block, allow, disallow }),
      ai_page: cleaned({ ...page, off, contact, covers, cite }),
    });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <TrainingChoice blocked={training} onChange={setTraining} />
      <h3>{words.oneByOne}</h3>
      <p className="hint">{words.oneByOneHint}</p>
      <BotBoxes
        legend={words.trainingBots}
        about={words.trainingBotsAbout}
        ticked={(bot) => training || bots.includes(bot)}
        locked={training}
        onTurn={turn}
      />
      <BotBoxes
        legend={words.answerBots}
        hint={words.answerHint}
        about={words.answerBotsAbout}
        ticked={(bot) => bots.includes(bot)}
        locked={false}
        onTurn={turn}
      />
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

type ChoiceProps = { blocked: boolean; onChange: (blocked: boolean) => void };

/** The screen's focus point: what AI training crawlers do, then the two choices and what each means. */
function TrainingChoice({ blocked, onChange }: ChoiceProps) {
  const choices = [
    {
      id: "seo-training-allow",
      block: false,
      label: words.trainingAllow,
      hint: words.trainingAllowHint,
    },
    {
      id: "seo-training-block",
      block: true,
      label: words.trainingBlock,
      hint: words.trainingBlockHint,
    },
  ];
  return (
    <fieldset className="training" aria-describedby="seo-training-context">
      <legend>{words.training}</legend>
      <p className="hint" id="seo-training-context">
        {words.trainingContext}
      </p>
      {choices.map((choice) => (
        <div className="choice" key={choice.id}>
          <input
            id={choice.id}
            type="radio"
            name="seo-training"
            checked={blocked === choice.block}
            aria-describedby={`${choice.id}-hint`}
            onChange={() => onChange(choice.block)}
          />
          <div>
            <label htmlFor={choice.id}>{choice.label}</label>
            <span className="hint" id={`${choice.id}-hint`}>
              {choice.hint}
            </span>
          </div>
        </div>
      ))}
    </fieldset>
  );
}

type BoxesProps = {
  legend: string;
  hint?: string;
  about: Record<string, string>;
  ticked: (bot: string) => boolean;
  /** Every box is ticked by the training choice and cannot be changed here. */
  locked: boolean;
  onTurn: (bot: string, on: boolean) => void;
};

/** A box per crawler in one group, each with who runs it and what for. */
function BotBoxes({ legend, hint, about, ticked, locked, onTurn }: BoxesProps) {
  return (
    <fieldset>
      <legend>{legend}</legend>
      {hint === undefined ? null : <span className="hint">{hint}</span>}
      <div className="bots">
        {Object.entries(about).map(([bot, what]) => (
          <Check
            key={bot}
            id={`seo-bot-${bot}`}
            label={words.block(bot)}
            hint={locked ? `${what}. ${words.bySwitch}` : what}
            checked={ticked(bot)}
            disabled={locked}
            onChange={(on) => onTurn(bot, on)}
          />
        ))}
      </div>
    </fieldset>
  );
}
