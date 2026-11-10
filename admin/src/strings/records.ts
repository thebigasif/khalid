// The words of the Trash and Audit log screens and of changing many posts
// at once (brief G52). The admin's other words are in en.ts.

/** How a deleted post's old status reads. */
const was = { draft: "Draft", published: "Live", scheduled: "Scheduled" } as const;

/** What each part of the Admin API holds, by the first part of a route after its base. */
const things: Record<string, string> = {
  posts: "a post",
  trash: "a post in the trash",
  redirects: "a redirect",
  "not-found": "an address that found nothing",
  comments: "a comment",
  webhooks: "a webhook",
  media: "a media file",
  staff: "a staff account",
};

/** How each action of the audit log starts its sentence. */
const doing: Record<string, string> = { added: "Added", changed: "Changed", removed: "Removed" };

/** What one change in the audit log did, in words, from its action and route. */
function happened(action: string, route: string): string {
  if (route.endsWith("/restore/")) {
    return "Put a post back from the trash";
  }
  if (route.endsWith("/preview-link/")) {
    return "Made a preview link for a post";
  }
  const part = route.replace("/api/admin/v1/", "").split("/")[0] ?? "";
  const thing = things[part] ?? "a setting";
  return `${doing[action] ?? "Changed"} ${thing}`;
}

export const records = {
  trash: {
    title: "Trash",
    lede: "Posts you deleted wait here until you put them back or delete them for good. Readers never see them.",
    loading: "Reading the trash",
    none: "The trash is empty. A post you delete waits here until you decide what to do with it.",
    count: (n: number) => (n === 1 ? "1 post is in the trash." : `${n} posts are in the trash.`),
    titleColumn: "Title",
    was: "Was",
    status: was,
    deleted: "Deleted",
    action: "What to do",
    restoreText: "Put it back",
    restore: (title: string) => `Put back ${title}`,
    restored: (title: string) => `${title} is back where it was.`,
    purgeText: "Delete for good",
    purge: (title: string) => `Delete ${title} for good`,
    sureText: "Yes, delete it for good",
    sure: (title: string) => `Yes, delete ${title} for good`,
    asking: (title: string) => `Delete ${title} for good? This cannot be undone.`,
    purged: (title: string) => `${title} is deleted for good.`,
    beforeTitle: "Before you delete for good",
    before:
      "Putting a post back returns it to where it was. A live post goes live again at the same address. Deleting for good removes the file, and it cannot be undone.",
  },
  audit: {
    title: "Audit log",
    lede: "Every change made to the blog through the admin, the API or an agent: who made it, what it touched and when. Only owners and admins see this page.",
    who: "Who",
    whoMeans:
      "The person or the key that made the change, by the name their account or key was given. A key that was revoked since shows its short code instead.",
    loading: "Reading the audit log",
    none: "Nothing has been changed through the admin or the API yet.",
    when: "When",
    what: "What happened",
    where: "Where",
    happened,
    older: "Show older changes",
    newest: "Back to the newest",
  },
  bulk: {
    label: "Change the posts you tick",
    tickColumn: "Tick",
    tick: (title: string) => `Tick ${title}`,
    ticked: (n: number) => (n === 1 ? "1 post ticked." : `${n} posts ticked.`),
    careful:
      "Each post is changed on its own. A post someone else changed meanwhile is left as it is, and you are told which.",
    action: "Change",
    author: "Set the author",
    trash: "Move to the trash",
    newAuthor: "New author",
    newAuthorHint: "The authors of the posts on this page.",
    submit: (n: number) => (n === 1 ? "Change 1 post" : `Change ${n} posts`),
    working: "Changing the posts",
    done: (n: number) => (n === 1 ? "Changed 1 post." : `Changed ${n} posts.`),
    left: (title: string, why: string) => `${title} was left as it is. ${why}`,
    farewell: "What should the old addresses of live posts do?",
    farewellHint:
      "Readers and search engines may still follow links to these posts. Tell them what happened.",
    gone: "Say the page is gone for good",
    goneHint: "Search engines drop the address quickly. This is a 410 answer.",
    moved: "Send readers to another post",
    movedHint:
      "Search engines move the old address's standing to the new post. This is a 301 redirect.",
    target: "Post to send readers to",
    targetHint: "Pick the live post closest to what the old ones covered.",
    noTarget: "There is no other live post to send readers to.",
    noFarewell: (title: string, from: string, why: string) =>
      `${title} is in the trash, but ${from} was not redirected. ${why} Add it under Redirects.`,
  },
} as const;
