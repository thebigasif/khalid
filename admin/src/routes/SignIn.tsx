import { blogName } from "../auth/blog";
import { EmailSignIn } from "../components/EmailSignIn";
import { SignInCard } from "../components/SignInCard";
import { TokenSignIn } from "../components/TokenSignIn";
import { en } from "../strings/en";

/** Whose admin this is, then the ways in: an email, or a staff token. */
export function SignIn() {
  const words = en.signIn;
  return (
    <SignInCard title={words.title(blogName() ?? words.thisBlog)}>
      <EmailSignIn primary={true} />
      <TokenSignIn />
      <p className="sign-new">
        {words.newHere} {words.owners} <code>{words.ownerCommand}</code>.
      </p>
    </SignInCard>
  );
}
