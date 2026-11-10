import { type FormEvent, useState } from "react";
import { useCategories } from "../api/categories";
import type { PostRow } from "../api/rows";
import { en } from "../strings/en";
import { BulkValue, type ValueKind } from "./BulkValue";
import { type BulkAction, type BulkResult, changeAll, type Farewell, liveAddress } from "./bulk";
import { FarewellChoice } from "./FarewellChoice";

type Props = {
  /** The posts ticked in the table. */
  ticked: PostRow[];
  /** The authors of the posts on the page, which "Set the author" offers. */
  authors: string[];
  onDone: (result: BulkResult) => void;
};

/** The changes that take a value, in the order the form offers them. */
const KINDS: ValueKind[] = ["author", "addTag", "removeTag", "category"];

/** The action `kind` and `value` name, with `farewell` for the trash. */
function actionOf(kind: BulkAction["kind"], value: string, farewell: Farewell): BulkAction {
  switch (kind) {
    case "author":
      return { kind, author: value };
    case "addTag":
    case "removeTag":
      return { kind, tag: value };
    case "category":
      return { kind, category: value === "" ? null : value };
    case "trash":
      return { kind, farewell };
  }
}

/** One form that changes every ticked post: author, tags, category, or the trash. */
export function BulkChange({ ticked, authors, onDone }: Props) {
  const words = en.bulk;
  const categories = useCategories();
  const tags = [...new Set(ticked.flatMap((row) => row.tags))];
  const first: Record<ValueKind, string | undefined> = {
    author: authors[0],
    addTag: "",
    removeTag: tags[0],
    category: categories[0]?.slug,
  };
  const offered = KINDS.filter((option) => first[option] !== undefined);
  const [kind, setKind] = useState<BulkAction["kind"]>(offered[0] ?? "trash");
  const [value, setValue] = useState(first[offered[0] ?? "addTag"] ?? "");
  const [farewell, setFarewell] = useState<Farewell>({ kind: "gone" });
  const [working, setWorking] = useState(false);
  const live = ticked.filter((row) => liveAddress(row) !== null);

  const pick = (picked: string) => {
    const kinds: BulkAction["kind"][] = [...offered, "trash"];
    const next = kinds.find((option) => option === picked) ?? "trash";
    setKind(next);
    setValue(next === "trash" ? "" : (first[next] ?? ""));
  };
  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setWorking(true);
    const result = await changeAll(ticked, actionOf(kind, value, farewell));
    setWorking(false);
    onDone(result);
  };

  return (
    <form className="bulk" aria-label={words.label} onSubmit={submit}>
      <p>
        <strong>{words.ticked(ticked.length)}</strong>{" "}
        <span className="muted">{words.careful}</span>
      </p>
      <div className="field">
        <label htmlFor="bulk-action">{words.action}</label>
        <select id="bulk-action" value={kind} onChange={(event) => pick(event.target.value)}>
          {offered.map((option) => (
            <option key={option} value={option}>
              {words[option]}
            </option>
          ))}
          <option value="trash">{words.trash}</option>
        </select>
      </div>
      {kind === "trash" ? null : (
        <BulkValue
          kind={kind}
          value={value}
          onChange={setValue}
          authors={authors}
          tags={tags}
          categories={categories}
        />
      )}
      {kind === "trash" && live.length > 0 ? (
        <FarewellChoice leaving={live} farewell={farewell} onChange={setFarewell} />
      ) : null}
      <button type="submit" className="btn" disabled={working}>
        {working ? words.working : words.submit(ticked.length)}
      </button>
    </form>
  );
}
