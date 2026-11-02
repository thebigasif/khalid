import { useState } from "react";
import { blogLogo, blogName } from "../auth/blog";

type Props = { look: "sign" | "side" };

/**
 * The blog's logo; its first letter in a badge when there is none, or it
 * does not load. Its name is always written beside it, so the mark itself
 * is decoration.
 */
export function BlogMark({ look }: Props) {
  const [logo, setLogo] = useState(blogLogo);
  if (logo !== null) {
    return <img className={`${look}-logo`} src={logo} alt="" onError={() => setLogo(null)} />;
  }
  const initial = blogName()?.charAt(0).toUpperCase() ?? "";
  return initial === "" ? null : (
    <span className={`${look}-initial`} aria-hidden="true">
      {initial}
    </span>
  );
}
