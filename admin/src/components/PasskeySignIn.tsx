import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { passkeySignInOptions, signInWithPasskey } from "../api/account";
import { isData } from "../api/call";
import { problemMessage } from "../api/posts";
import { saveCsrf } from "../auth/session";
import { choosePasskey } from "../passkeys/webauthn";
import { en, type UserMessage } from "../strings/en";
import { Problem } from "./Problem";

/** The main way in: one of this blog's passkeys, on this device. */
export function PasskeySignIn() {
  const words = en.signIn;
  const navigate = useNavigate();
  const [busy, setBusy] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);

  // Each step stops at the first problem: the options, the device, the blog.
  const signIn = async (): Promise<UserMessage | null> => {
    const options = await passkeySignInOptions();
    if (!isData(options)) {
      return options.status === 404
        ? words.problems.noPasskeys
        : problemMessage(options.problem, en.problems.unreachable);
    }
    const used = await choosePasskey(options.data);
    if (used === null) {
      return words.problems.noPasskey;
    }
    const reply = await signInWithPasskey(used);
    if (!isData(reply)) {
      return problemMessage(reply.problem, words.problems.noPasskey);
    }
    saveCsrf(reply.data.csrf);
    return null;
  };

  const press = async () => {
    setBusy(true);
    setProblem(null);
    const found = await signIn();
    setBusy(false);
    if (found === null) {
      await navigate({ to: "/" });
      return;
    }
    setProblem(found);
  };

  return (
    <div className="sign-way">
      {problem === null ? null : <Problem message={problem} />}
      <button type="button" className="btn primary" disabled={busy} onClick={press}>
        {busy ? words.passkey.checking : words.passkey.submit}
      </button>
      <p className="hint">{words.passkey.hint}</p>
    </div>
  );
}
