import { type ReactNode, useState } from "react";
import { blogLogo, blogName } from "../auth/blog";

type Props = { title: string; children: ReactNode };

/** The blog's logo; its first letter in a badge when there is none, or it does not load. */
function BlogMark() {
  const [logo, setLogo] = useState(blogLogo);
  if (logo !== null) {
    // The heading names the blog, so the logo itself is decoration.
    return <img className="sign-logo" src={logo} alt="" onError={() => setLogo(null)} />;
  }
  const initial = blogName()?.charAt(0).toUpperCase() ?? "";
  return initial === "" ? null : (
    <span className="sign-initial" aria-hidden="true">
      {initial}
    </span>
  );
}

/** The centred card of the screens before the admin opens: whose blog, one heading, one task. */
export function SignInCard({ title, children }: Props) {
  return (
    <main id="main" className="sign-in">
      <div className="sign-card">
        <BlogMark />
        <h1>{title}</h1>
        {children}
      </div>
    </main>
  );
}
