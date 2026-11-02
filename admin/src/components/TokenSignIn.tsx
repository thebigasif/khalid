import { useNavigate } from "@tanstack/react-router";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { checkKey, type KeyCheck } from "../auth/checkKey";
import { saveKey } from "../auth/session";
import { en, type UserMessage } from "../strings/en";
import { Problem } from "./Problem";

const MESSAGES: Record<Exclude<KeyCheck, "accepted">, UserMessage> = {
  rejected: en.problems.rejected,
  "lacks-scope": en.problems.lacksScope,
  unreachable: en.problems.unreachable,
};

/** The staff token field, behind a small link: few people need it. */
export function TokenSignIn() {
  const words = en.signIn.token;
  const navigate = useNavigate();
  const [open, setOpen] = useState(false);
  const [key, setKey] = useState("");
  const [checking, setChecking] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const field = useRef<HTMLInputElement>(null);

  // Opening the field puts the cursor in it, so a keyboard user can paste at once.
  useEffect(() => {
    if (open) {
      field.current?.focus();
    }
  }, [open]);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setChecking(true);
    const answer = await checkKey(key.trim());
    setChecking(false);
    if (answer === "accepted") {
      saveKey(key.trim());
      await navigate({ to: "/" });
      return;
    }
    setProblem(MESSAGES[answer]);
  };

  return (
    <div className="sign-token">
      <button
        type="button"
        className="quiet-link"
        aria-expanded={open}
        onClick={() => setOpen(!open)}
      >
        {words.open}
      </button>
      {open ? (
        <form onSubmit={submit}>
          {problem === null ? null : <Problem message={problem} />}
          <div className="field">
            <label htmlFor="key">{words.label}</label>
            <span className="hint" id="key-hint">
              {words.hint}
            </span>
            <input
              id="key"
              ref={field}
              type="password"
              autoComplete="off"
              spellCheck={false}
              required
              aria-describedby="key-hint"
              value={key}
              onChange={(event) => setKey(event.target.value)}
            />
          </div>
          <button type="submit" className="btn" disabled={checking}>
            {checking ? words.checking : words.submit}
          </button>
        </form>
      ) : null}
    </div>
  );
}
