import { Link } from "@tanstack/react-router";
import { useState } from "react";
import { addThisDevice } from "../passkeys/addThisDevice";
import { canUsePasskeys } from "../passkeys/webauthn";
import { en, type UserMessage } from "../strings/en";
import { Problem } from "./Problem";
import { SignInCard } from "./SignInCard";

/** The step after a sign-in link: a passkey for next time, or a password instead. */
export function Welcome() {
  const words = en.signIn.welcome;
  const passkeys = canUsePasskeys();
  const [busy, setBusy] = useState(false);
  const [added, setAdded] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);

  const add = async () => {
    setBusy(true);
    setProblem(null);
    const found = await addThisDevice(words.passkeyName);
    setBusy(false);
    setAdded(found === null);
    setProblem(found);
  };

  if (added) {
    return (
      <SignInCard title={words.addedTitle}>
        <p className="sign-lede">{words.added}</p>
        <Link to="/" className="btn primary">
          {words.open}
        </Link>
      </SignInCard>
    );
  }
  return (
    <SignInCard title={words.title}>
      <p className="sign-lede">{passkeys ? words.lede : words.noPasskeys}</p>
      {problem === null ? null : <Problem message={problem} />}
      {passkeys ? (
        <button type="button" className="btn primary" disabled={busy} onClick={add}>
          {busy ? words.adding : words.add}
        </button>
      ) : null}
      <Link to="/account" className={passkeys ? "btn" : "btn primary"}>
        {words.password}
      </Link>
      <p className="sign-new">
        <Link to="/">{words.skip}</Link>
      </p>
    </SignInCard>
  );
}
