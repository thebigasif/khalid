import type { SeoScores } from "../api/check";
import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";
import type { Finding, Score } from "./lint";

type Props = {
  findings: Finding[];
  score: Score | null;
  /** The SEO and readability scores, or null before the blog has checked the post. */
  scores?: SeoScores | null;
  /** The words in the post's text, or null before the check. */
  words?: number | null;
};

/** The words on a finding's tag: whether it stops publishing, warns or is a tip. */
const TAGS: Record<Finding["severity"], string> = {
  block: en.editor.blockTag,
  warn: en.editor.warnTag,
  info: en.editor.tipTag,
};

/** The light a score shows: green from 80, amber from 50, red below. */
export function lightOf(value: number): keyof typeof seoEditorWords.state {
  if (value >= 80) {
    return "good";
  }
  return value >= 50 ? "ok" : "bad";
}

/** One score as a light, with its number and what the light means in words. */
function Light({ name, value }: { name: string; value: number }) {
  const light = lightOf(value);
  return (
    <li>
      <span className={`light ${light}`} aria-hidden="true" />
      <strong>{name}</strong> {seoEditorWords.outOf(value)}
      <span className="muted"> · {seoEditorWords.state[light]}</span>
    </li>
  );
}

/** The linter's findings beside the post, each saying what, why and fix, under the scores. */
export function LintPanel({ findings, score, scores = null, words = null }: Props) {
  return (
    <aside className="checks" aria-labelledby="checks-title">
      <h2 id="checks-title">{en.editor.checks}</h2>
      {score === null ? null : (
        <div className="score">
          <b aria-hidden="true">{score.value}</b>
          <p>
            <strong>{en.editor.score(score.value)}</strong> {en.editor.scoreWhy(score.lost)}
          </p>
        </div>
      )}
      {scores === null ? null : (
        <ul className="lights" aria-label={seoEditorWords.lights}>
          <Light name={seoEditorWords.seo} value={scores.seo} />
          <Light name={seoEditorWords.readability} value={scores.readability} />
          {words === null ? null : <li className="muted">{seoEditorWords.words(words)}</li>}
        </ul>
      )}
      {findings.length === 0 ? <p className="hint">{en.editor.noChecks}</p> : null}
      {findings.map((finding) => {
        const tag = TAGS[finding.severity];
        return (
          <section
            key={`${finding.rule} ${finding.subject ?? ""} ${finding.what}`}
            className={`check ${finding.severity}`}
            aria-label={tag}
          >
            <p className="tag">{tag}</p>
            <p>
              <strong>{finding.what}</strong>
              {finding.subject === null ? null : ` ${en.editor.here} "${finding.subject}".`}
            </p>
            <p>{finding.why}</p>
            <p>{finding.fix}</p>
            {finding.guide_url === undefined ? null : (
              <p>
                <a href={finding.guide_url} target="_blank" rel="noopener">
                  {en.editor.guide}
                </a>
              </p>
            )}
          </section>
        );
      })}
    </aside>
  );
}
