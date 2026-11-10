import { useState } from "react";
import type { AdminNewsletter } from "../api/subscribers";
import { subscriberWords } from "../strings/subscriberWords";

type Props = { newsletters: AdminNewsletter[] };

// The clipboard may be missing or refused; the form stays on screen to copy by hand.
async function copy(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    return false;
  }
}

/** Each newsletter: how many readers get it, how many still have to confirm, and its form. */
export function NewsletterCards({ newsletters }: Props) {
  const words = subscriberWords.lists;
  const [copied, setCopied] = useState<string | null>(null);
  return (
    <section aria-labelledby="lists-title">
      <h2 id="lists-title">{words.title}</h2>
      <ul className="looks">
        {newsletters.map((newsletter) => {
          const field = `form-${newsletter.name}`;
          return (
            <li key={newsletter.name} className="card">
              <h3>{newsletter.name}</h3>
              <p className="big">{newsletter.confirmed.toLocaleString("en")}</p>
              <p className="muted">
                {words.readers(newsletter.confirmed)} {words.waiting(newsletter.pending)}
              </p>
              <details>
                <summary>{words.form}</summary>
                <p className="muted">{words.formHint}</p>
                <label htmlFor={field} className="sr-only">
                  {words.form}
                </label>
                <textarea id={field} readOnly rows={4} value={newsletter.form} />
                <button
                  type="button"
                  className="btn"
                  onClick={async () => {
                    if (await copy(newsletter.form)) {
                      setCopied(newsletter.name);
                    }
                  }}
                >
                  {words.copy}
                </button>
                <p role="status">{copied === newsletter.name ? words.copied : null}</p>
              </details>
            </li>
          );
        })}
      </ul>
    </section>
  );
}
