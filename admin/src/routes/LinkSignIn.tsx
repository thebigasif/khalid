import { useQuery } from "@tanstack/react-query";
import { Link, useLocation } from "@tanstack/react-router";
import { redeemLink } from "../api/account";
import { isData } from "../api/call";
import { blogName } from "../auth/blog";
import { saveCsrf } from "../auth/session";
import { Problem } from "../components/Problem";
import { SignInCard } from "../components/SignInCard";
import { Welcome } from "../components/Welcome";
import { en } from "../strings/en";

/** Signs in with a link someone was sent, then offers a passkey for next time. */
export function LinkSignIn() {
  const words = en.signIn.link;
  // The token sits after the #, so it never reaches a server log.
  const hash = useLocation({ select: (location) => location.hash });
  const token = new URLSearchParams(hash.replace(/^#/, "")).get("token") ?? "";
  const redeemed = useQuery({
    queryKey: ["sign-in-link", token],
    queryFn: async () => {
      const reply = await redeemLink(token);
      if (isData(reply)) {
        saveCsrf(reply.data.csrf);
      }
      return reply;
    },
    retry: false,
    staleTime: Number.POSITIVE_INFINITY,
  }).data;

  if (isData(redeemed)) {
    // Someone signed in by link may have no passkey or password yet, so they choose one first.
    return <Welcome />;
  }
  if (redeemed === undefined) {
    return (
      <SignInCard title={words.title}>
        <p className="sign-lede" role="status">
          {words.checking}
        </p>
      </SignInCard>
    );
  }
  // Any answer but none at all means the link is spent: used, expired or mistyped.
  const problem = redeemed.status === 0 ? en.problems.unreachable : en.problems.badLink;
  return (
    <SignInCard title={en.signIn.title(blogName() ?? en.signIn.thisBlog)}>
      <Problem message={problem} />
      <Link to="/sign-in" className="btn">
        {words.back}
      </Link>
    </SignInCard>
  );
}
