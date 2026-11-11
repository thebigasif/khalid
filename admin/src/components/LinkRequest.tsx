import { type FormEvent, useEffect, useRef, useState } from "react";
import { isData } from "../api/call";
import { problemMessage } from "../api/posts";
import { sendLink } from "../api/sites";
import { en, type UserMessage } from "../strings/en";
import { Problem } from "./Problem";

/** Asks the blog to email a sign-in link, behind a small link like the staff token. */
export function LinkRequest() {
  const words = en.signIn.byLink;
  const [open, setOpen] = useState(false);
  const [email, setEmail] = useState("");
  const [busy, setBusy] = useState(false);
  const [sent, setSent] = useState<string | null>(null);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const field = useRef<HTMLInputElement>(null);

  // Opening the form puts the cursor in it, as the staff token's does.
  useEffect(() => {
    if (open) {
      field.current?.focus();
    }
  }, [open]);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const to = email.trim();
    if (to === "" || busy) {
      return;
    }
    setBusy(true);
    setSent(null);
    const reply = await sendLink(to);
    setBusy(false);
    if (isData(reply)) {
      setProblem(null);
      setSent(to);
      return;
    }
    setProblem(problemMessage(reply.problem, en.problems.unreachable));
  };

  let said: string | null = busy ? words.sending : null;
  if (sent !== null) {
    said = words.sent(sent);
  }
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
        <form aria-label={words.form} onSubmit={submit}>
          {problem === null ? null : <Problem message={problem} />}
          <p role="status">{said}</p>
          <div className="field">
            <label htmlFor="link-email">{words.label}</label>
            <span className="hint" id="link-email-hint">
              {words.hint}
            </span>
            <input
              id="link-email"
              ref={field}
              type="email"
              autoComplete="username"
              required
              aria-describedby="link-email-hint"
              value={email}
              onChange={(event) => setEmail(event.target.value)}
            />
          </div>
          <button type="submit" className="btn" disabled={busy}>
            {words.submit}
          </button>
        </form>
      ) : null}
    </div>
  );
}
