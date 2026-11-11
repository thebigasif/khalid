import { en } from "../../strings/en";
import { choicesOf, labelOf } from "./blockForm";
import { SnippetChoice } from "./SnippetChoice";

const choiceNames: Record<string, string> = en.choices;

type Props = {
  /** The directive name of the block the form is for, such as `chart`. */
  block: string;
  field: string;
  value: string;
  onChange: (value: string) => void;
};

/** One labelled detail of a block: a list of choices or snippets, the block's text, or a line. */
export function DetailField({ block, field, value, onChange }: Props) {
  const id = `block-${field}`;
  const choices = choicesOf(block, field);
  let input = <input id={id} value={value} onChange={(event) => onChange(event.target.value)} />;
  if (choices !== undefined) {
    input = (
      <select
        id={id}
        value={value === "" ? choices[0] : value}
        onChange={(event) => onChange(event.target.value)}
      >
        {choices.map((choice) => (
          <option key={choice} value={choice}>
            {choiceNames[choice] ?? choice}
          </option>
        ))}
      </select>
    );
  } else if (field === "slug") {
    input = <SnippetChoice id={id} value={value} onChange={onChange} />;
  } else if (field === "body") {
    input = (
      <textarea
        id={id}
        className="source"
        value={value}
        rows={6}
        onChange={(event) => onChange(event.target.value)}
      />
    );
  }
  return (
    <div className="field">
      <label htmlFor={id}>{labelOf(block, field) ?? field}</label>
      {input}
    </div>
  );
}
