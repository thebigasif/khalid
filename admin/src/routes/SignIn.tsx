import { blogName } from "../auth/blog";
import { EmailSignIn } from "../components/EmailSignIn";
import { PasskeySignIn } from "../components/PasskeySignIn";
import { SignInCard } from "../components/SignInCard";
import { TokenSignIn } from "../components/TokenSignIn";
import { canUsePasskeys } from "../passkeys/webauthn";
import { en } from "../strings/en";

/** Whose admin this is, then the ways in: a passkey, an email, or a staff token. */
export function SignIn() {
  const words = en.signIn;
  const passkeys = canUsePasskeys();
  return (
    <SignInCard title={words.title(blogName() ?? words.thisBlog)}>
      {passkeys ? <PasskeySignIn /> : null}
      {passkeys ? <p className="sign-or">{words.or}</p> : null}
      <EmailSignIn primary={!passkeys} />
      <TokenSignIn />
      <p className="sign-new">
        {words.newHere} {words.owners} <code>{words.ownerCommand}</code>.
      </p>
    </SignInCard>
  );
}
