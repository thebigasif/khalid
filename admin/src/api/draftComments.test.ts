import { describe, expect, it } from "vitest";
import { adminBlog, noContent } from "../../test/blog";
import { json, signedIn } from "../../test/render";
import { isData } from "./call";
import {
  addDraftComment,
  type DraftComment,
  listDraftComments,
  resolveThread,
} from "./draftComments";

const FIRST: DraftComment = {
  id: 4,
  parent: null,
  block: 2,
  author: "maria",
  text: "Can we name the tool here?",
  resolved: false,
  created_at: "2026-11-10T10:42:00Z",
};

const NONE_KEPT = {
  type: "https://khalid.dev/problems/invalid-body",
  title: "This site keeps no comments on drafts.",
  status: 422,
  detail: "Comments on drafts are kept by the site's server, in its database.",
  fix: "Send the request to the running khalid serve.",
};

describe("the draft comment routes", () => {
  it("lists a post's comments with the signed-in key", async () => {
    signedIn();
    const sent = adminBlog({
      "GET /posts/01JB/draft-comments/": () => json(200, { draft_comments: [FIRST] }),
    });
    expect(await listDraftComments("01JB")).toEqual({ data: [FIRST] });
    expect(sent[0]?.headers.get("Authorization")).toMatch(/^Khalid /);
  });

  it("sends a new thread as the one comment the API expects", async () => {
    signedIn();
    const sent = adminBlog({
      "POST /posts/01JB/draft-comments/": () => json(201, { draft_comments: [FIRST] }),
    });
    const added = await addDraftComment("01JB", { text: FIRST.text, block: 2 });
    expect(added).toEqual({ data: [FIRST] });
    expect(await sent[0]?.json()).toEqual({ draft_comments: [{ text: FIRST.text, block: 2 }] });
  });

  it("sends an answer with its thread and no block", async () => {
    signedIn();
    const sent = adminBlog({
      "POST /posts/01JB/draft-comments/": () => json(201, { draft_comments: [] }),
    });
    await addDraftComment("01JB", { text: "Yes.", parent: 4 });
    expect(await sent[0]?.json()).toEqual({ draft_comments: [{ text: "Yes.", parent: 4 }] });
  });

  it("marks a thread done or open again, with nothing to read back", async () => {
    signedIn();
    const sent = adminBlog({ "PUT /posts/01JB/draft-comments/4/": noContent });
    expect(isData(await resolveThread("01JB", 4, true))).toBe(true);
    expect(await sent[0]?.json()).toEqual({ draft_comments: [{ resolved: true }] });
  });

  it("gives back the problem when the site keeps no comments", async () => {
    signedIn();
    adminBlog({ "POST /posts/01JB/draft-comments/": () => json(422, NONE_KEPT) });
    const added = await addDraftComment("01JB", { text: "Hello" });
    expect(added).toEqual({ problem: NONE_KEPT, status: 422 });
  });

  it("counts an answer with no list of comments as a failure", async () => {
    signedIn();
    adminBlog({ "GET /posts/01JB/draft-comments/": () => json(200, { comments: [] }) });
    expect(isData(await listDraftComments("01JB"))).toBe(false);
  });
});
