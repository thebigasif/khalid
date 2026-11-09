import { isPage, type PostRow, useRows } from "../api/rows";
import { en } from "../strings/en";
import type { Farewell } from "./bulk";

type Props = {
  /** The live posts going to the trash, which no farewell may send readers to. */
  leaving: PostRow[];
  farewell: Farewell;
  onChange: (farewell: Farewell) => void;
};

/** Asks what the old addresses of live posts going to the trash should do: 410 or 301. */
export function FarewellChoice({ leaving, farewell, onChange }: Props) {
  const words = en.bulk;
  const live = useRows({ filter: "status:published", order: "title asc", limit: 100 });
  const going = new Set(leaving.map((row) => row.id));
  const targets = isPage(live)
    ? live.rows.filter((row) => !going.has(row.id) && row.path !== undefined && row.path !== null)
    : [];
  const first = targets[0]?.path ?? null;
  const moved = farewell.kind === "moved";
  return (
    <fieldset>
      <legend>{words.farewell}</legend>
      <p className="hint">{words.farewellHint}</p>
      <label className="choice">
        <input
          type="radio"
          name="bulk-farewell"
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
          name="bulk-farewell"
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
          <label htmlFor="bulk-target">{words.target}</label>
          <span className="hint" id="bulk-target-hint">
            {words.targetHint}
          </span>
          <select
            id="bulk-target"
            aria-describedby="bulk-target-hint"
            value={farewell.to}
            onChange={(event) => onChange({ kind: "moved", to: event.target.value })}
          >
            {targets.map((row) => (
              <option key={row.id} value={row.path ?? ""}>
                {row.title}
              </option>
            ))}
          </select>
        </div>
      ) : null}
    </fieldset>
  );
}
