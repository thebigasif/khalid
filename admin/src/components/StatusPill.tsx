import type { PostRow } from "../api/rows";
import { en } from "../strings/en";

/** The colour of each post status: live is green, scheduled the accent, a draft grey. */
const LOOKS: Record<PostRow["status"], string> = {
  published: "pill ok",
  scheduled: "pill info",
  draft: "pill quiet",
};

/** Where a post stands, as a small coloured pill with its name in words. */
export function StatusPill({ status }: { status: PostRow["status"] }) {
  return <span className={LOOKS[status]}>{en.status[status]}</span>;
}
