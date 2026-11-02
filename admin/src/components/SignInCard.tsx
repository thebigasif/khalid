import type { ReactNode } from "react";
import { BlogMark } from "./BlogMark";

type Props = { title: string; children: ReactNode };

/** The centred card of the screens before the admin opens: whose blog, one heading, one task. */
export function SignInCard({ title, children }: Props) {
  return (
    <main id="main" className="sign-in">
      <div className="sign-card">
        <BlogMark look="sign" />
        <h1>{title}</h1>
        {children}
      </div>
    </main>
  );
}
