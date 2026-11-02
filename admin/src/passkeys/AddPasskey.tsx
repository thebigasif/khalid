import { type FormEvent, useState } from "react";
import { en, type UserMessage } from "../strings/en";
import { addThisDevice } from "./addThisDevice";

type Props = {
  /** Called with the new passkey's name once the blog has it, or with what went wrong. */
  onDone: (name: string | null, problem: UserMessage | null) => void;
};

/** The screen's focus: a passkey made by this device and added to the account. */
export function AddPasskey({ onDone }: Props) {
  const words = en.passkeys;
  const [name, setName] = useState("");
  const [busy, setBusy] = useState(false);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const label = name.trim();
    if (busy) {
      return;
    }
    if (label === "") {
      onDone(null, words.problems.blankName);
      return;
    }
    setBusy(true);
    const problem = await addThisDevice(label);
    setBusy(false);
    if (problem === null) {
      setName("");
    }
    onDone(problem === null ? label : null, problem);
  };

  return (
    <section className="focus" aria-labelledby="passkey-add-title">
      <h2 id="passkey-add-title">{words.addTitle}</h2>
      <p>{words.addLede}</p>
      <p role="status">{busy ? words.adding : null}</p>
      <form onSubmit={submit}>
        <div className="field">
          <label htmlFor="passkey-name">{words.name}</label>
          <span className="hint" id="passkey-name-hint">
            {words.nameHint}
          </span>
          <input
            id="passkey-name"
            value={name}
            maxLength={60}
            required
            aria-describedby="passkey-name-hint"
            onChange={(event) => setName(event.target.value)}
          />
        </div>
        <button type="submit" className="btn primary" disabled={busy}>
          {words.addText}
        </button>
      </form>
    </section>
  );
}
