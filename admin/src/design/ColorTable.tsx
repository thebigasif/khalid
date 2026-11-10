import type { ContrastPair } from "../api/design";
import { COLOR_TOKENS, type ColorToken, designWords } from "../strings/designWords";
import { channels } from "./contrast";
import { type Readable, readable, scoreWords, type Values } from "./designChange";

type Props = {
  values: Values;
  /** The colours as saved, with the server's contrast and nearest shades. */
  saved: { tokens: Record<string, string>; contrast: ContrastPair[] };
  onColor: (token: ColorToken, value: string) => void;
  onKeep: (token: ColorToken, keep: boolean) => void;
};

/**
 * A ref that sets CSS custom properties through the element's style
 * object. The admin's content policy refuses a style attribute, not this.
 */
export function painted(properties: Record<string, string>) {
  return (node: HTMLElement | null) => {
    for (const [name, value] of Object.entries(properties)) {
      node?.style.setProperty(name, value);
    }
  };
}

/** `value` when it is a colour, else no colour at all. */
export function colorOr(value: string | undefined): string {
  return value !== undefined && channels(value) !== null ? value : "transparent";
}

/** The pill's look for a score: green when it passes, amber when not. */
function pillOf(score: Readable): string {
  if (score.kind === "score") {
    return score.passes ? "pill ok" : "pill warn";
  }
  return "pill quiet";
}

/** The server's nearest passing shade for `token`, while the colour is as saved. */
function nearest(token: ColorToken, props: Props): string | null {
  if (props.values.tokens[token] !== props.saved.tokens[token]) {
    return null;
  }
  const pair = props.saved.contrast.find((each) => each.token === token && !each.passes);
  return pair?.nearest ?? null;
}

/** Every colour with its swatch, its contrast beside it, and whether a sync keeps it. */
export function ColorTable(props: Props) {
  const words = designWords;
  const { values, onColor, onKeep } = props;
  return (
    <div className="scroll">
      <table className="design-colors">
        <thead>
          <tr>
            <th scope="col">{words.columns.usedFor}</th>
            <th scope="col">
              <span className="visually-hidden">{words.columns.swatch}</span>
            </th>
            <th scope="col">{words.columns.color}</th>
            <th scope="col">{words.columns.readable}</th>
            <th scope="col">{words.columns.keep}</th>
          </tr>
        </thead>
        <tbody>
          {COLOR_TOKENS.map((token) => {
            const usedFor = words.usedFor[token];
            const value = values.tokens[token] ?? "";
            const score = readable(token, values.tokens);
            const shade = nearest(token, props);
            const id = `color-${token.replace("colors.", "")}`;
            return (
              <tr key={token}>
                <th scope="row">{usedFor}</th>
                <td>
                  <span
                    className="swatch"
                    aria-hidden="true"
                    ref={painted({ "--swatch": colorOr(value) })}
                  />
                </td>
                <td data-label={words.columns.color}>
                  <label className="visually-hidden" htmlFor={id}>
                    {words.colorLabel(usedFor)}
                  </label>
                  <input
                    id={id}
                    className="hex"
                    value={value}
                    spellCheck={false}
                    onChange={(event) => onColor(token, event.target.value)}
                  />
                </td>
                <td data-label={words.columns.readable}>
                  <span className={pillOf(score)}>{scoreWords(score)}</span>
                  {shade === null ? null : (
                    <button
                      type="button"
                      className="btn nearest"
                      onClick={() => onColor(token, shade)}
                    >
                      {words.useNearest(shade)}
                    </button>
                  )}
                </td>
                <td data-label={words.columns.keep}>
                  <label className="choice">
                    <input
                      type="checkbox"
                      checked={values.locked.includes(token)}
                      onChange={(event) => onKeep(token, event.target.checked)}
                    />
                    <span className="visually-hidden">{words.keepLabel(usedFor)}</span>
                  </label>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
