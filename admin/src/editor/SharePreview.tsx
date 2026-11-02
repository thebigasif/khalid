import { seoEditorWords } from "../strings/seoEditorWords";
import type { SettingsDraft } from "./settings";

type Props = {
  /** The post's title, which a card without its own headline shows. */
  title: string;
  /** The address the post lives at, or empty before the blog has checked it. */
  url: string;
  settings: SettingsDraft;
};

type Card = { title: string; description: string; image: string };

// The first of `choices` with something in it.
function first(...choices: string[]): string {
  return choices.map((choice) => choice.trim()).find((choice) => choice !== "") ?? "";
}

/** The host of `url` as a card names the site, or empty. */
function hostOf(url: string): string {
  try {
    return new URL(url).host;
  } catch {
    return "";
  }
}

/** One network's card: the picture, then the site and the words. */
function SharedCard({ name, card, site }: { name: string; card: Card; site: string }) {
  return (
    <figure className={`share ${name.toLowerCase()}`}>
      <figcaption className="hint">{name}</figcaption>
      {card.image === "" ? (
        <p className="share-none">{seoEditorWords.noImage}</p>
      ) : (
        <img src={card.image} alt="" width={1200} height={630} />
      )}
      <div className="share-words">
        <p className="share-site">{site}</p>
        <p className="share-title">{card.title}</p>
        {card.description === "" ? null : <p className="share-desc">{card.description}</p>}
      </div>
    </figure>
  );
}

/** The post as a link shared on Facebook and on X, from its share settings or its own. */
export function SharePreview({ title, url, settings }: Props) {
  const words = seoEditorWords;
  const facebook: Card = {
    title: first(settings.facebookTitle, title),
    description: first(settings.facebookDescription, settings.excerpt),
    image: first(settings.facebookImage, settings.imageSrc),
  };
  const x: Card = {
    title: first(settings.xTitle, title),
    description: first(settings.xDescription, settings.excerpt),
    image: first(settings.xImage, settings.imageSrc),
  };
  const site = hostOf(url);
  return (
    <details>
      <summary>{words.share}</summary>
      <div>
        <p className="hint">{words.shareLede}</p>
        <SharedCard name={words.facebook} card={facebook} site={site.toUpperCase()} />
        <SharedCard name={words.x} card={x} site={site} />
      </div>
    </details>
  );
}
