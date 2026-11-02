import { Link } from "@tanstack/react-router";
import { en } from "../strings/en";
import { ProfileForm } from "./ProfileForm";

type Props = { author: string | null };

/** The signed-in person's own author profile, ready to change, or how to get one. */
export function YourProfile({ author }: Props) {
  const words = en.account.profile;
  return (
    <section className="max-w-160" aria-labelledby="profile-title">
      <h2 id="profile-title">{words.title}</h2>
      {author === null ? (
        <>
          <p>{words.none}</p>
          <Link to="/authors">{words.authors}</Link>
        </>
      ) : (
        <>
          <p>{words.lede}</p>
          <ProfileForm id={author} />
        </>
      )}
    </section>
  );
}
