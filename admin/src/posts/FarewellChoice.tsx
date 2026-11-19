import { isPage, type PostRow, useRows } from "../api/rows";
import { en } from "../strings/en";
import type { Farewell } from "./bulk";

/** A live address a farewell can send readers to, with the title the list shows. */
export type Target = { title: string; path: string };

/** The words a farewell choice shows, which bulk delete and page delete each set. */
export type FarewellWords = Record<
  | "farewell"
  | "farewellHint"
  | "gone"
  | "goneHint"
  | "moved"
  | "movedHint"
  | "target"
  | "targetHint"
  | "noTarget",
  string
>;

type Props = {
  /** The live posts going to the trash, which no farewell may send readers to. */
  leaving: Pick<PostRow, "id">[];
  farewell: Farewell;
  onChange: (farewell: Farewell) => void;
  /** More addresses to offer before the live posts, such as the site's other pages. */
  also?: Target[];
  words?: FarewellWords;
  /** What the choice's fields are named by, unique on the screen. */
  name?: string;
};

/** Asks what the old addresses of live posts going to the trash should do: 410 or 301. */
export function FarewellChoice({
  leaving,
  farewell,
  onChange,
  also = [],
  words = en.bulk,
  name = "bulk",
}: Props) {
  const live = useRows({ filter: "status:published", order: "title asc", limit: 100 });
  const going = new Set(leaving.map((row) => row.id));
  const posts: Target[] = isPage(live)
    ? live.rows.flatMap((row) =>
        going.has(row.id) || row.path === undefined || row.path === null
          ? []
          : [{ title: row.title, path: row.path }],
      )
    : [];
  const targets = [...also, ...posts];
  const first = targets[0]?.path ?? null;
  const moved = farewell.kind === "moved";
  return (
    <fieldset>
      <legend>{words.farewell}</legend>
      <p className="hint">{words.farewellHint}</p>
      <label className="choice">
        <input
          type="radio"
          name={`${name}-farewell`}
          checked={!moved}
          onChange={() => onChange({ kind: "gone" })}
        />
        <span>
          <strong>{words.gone}</strong>
          <span className="hint">{words.goneHint}</span>
        </span>
      </label>
      <label className="choice">
        <input
          type="radio"
          name={`${name}-farewell`}
          checked={moved}
          disabled={first === null}
          onChange={() => (first === null ? null : onChange({ kind: "moved", to: first }))}
        />
        <span>
          <strong>{words.moved}</strong>
          <span className="hint">{first === null ? words.noTarget : words.movedHint}</span>
        </span>
      </label>
      {farewell.kind === "moved" ? (
        <div className="field">
          <label htmlFor={`${name}-target`}>{words.target}</label>
          <span className="hint" id={`${name}-target-hint`}>
            {words.targetHint}
          </span>
          <select
            id={`${name}-target`}
            aria-describedby={`${name}-target-hint`}
            value={farewell.to}
            onChange={(event) => onChange({ kind: "moved", to: event.target.value })}
          >
            {targets.map((target) => (
              <option key={target.path} value={target.path}>
                {target.title}
              </option>
            ))}
          </select>
        </div>
      ) : null}
    </fieldset>
  );
}
