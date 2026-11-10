import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { Calendar, CalendarPost, Plan } from "../api/calendar";
import type { AdminPost, PostEdit } from "../api/posts";
import { DRAGGED } from "../calendar/PostChip";
import { calendarWords } from "../strings/calendarWords";
import { calendarDay } from "../strings/dates";

const LIVE: CalendarPost = {
  id: "en.why-internal-links-matter",
  title: "Why internal links matter",
  status: "published",
  review_state: "ready",
  at: "2026-11-03T09:30:00Z",
};
const NEXT: CalendarPost = {
  id: "en.choosing-anchor-text",
  title: "Choosing anchor text",
  status: "scheduled",
  review_state: "ready",
  at: "2026-11-20T12:00:00Z",
};
const IDEA: CalendarPost = {
  id: "en.planning-a-topic-cluster",
  title: "Planning a topic cluster",
  status: "draft",
  review_state: "ready",
  at: null,
};

const CALENDAR: Calendar = {
  from: "2026-10-26",
  to: "2026-12-06",
  days: [
    { date: "2026-11-03", posts: [LIVE] },
    { date: "2026-11-20", posts: [NEXT] },
  ],
  undated: [IDEA],
};

const PLAN: Plan = {
  per_week: 2,
  pace_from: "history",
  calendar: [
    { day: "2026-11-12", id: IDEA.id, title: IDEA.title, kind: "suggested" },
    { day: "2026-11-20", id: NEXT.id, title: NEXT.title, kind: "scheduled" },
  ],
  waiting: [],
  warnings: [
    {
      what: "2 posts went live in the last four weeks, against 5 in the four weeks before.",
      why: "Search engines read a steady pace as a sign a site keeps covering its subject.",
      fix: "Schedule the suggested drafts on their days to get back to your pace.",
    },
  ],
};

const DRAFT: AdminPost = {
  id: IDEA.id,
  slug: "planning-a-topic-cluster",
  title: IDEA.title,
  status: "draft",
  markdown: "A topic cluster is a group of posts.\n",
  updated_at: "2026-11-09T16:20:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    tags: [],
    share_cards: {},
    aliases: [],
  },
};

const REFUSED = {
  type: "https://khalid.dev/problems/lint-blocked",
  title: "The linter stopped this post.",
  status: 422,
  detail: "The linter found something that stops this post from going live.",
  fix: "Fix each finding marked block, then publish again.",
};

/** A blog with `CALENDAR`, answering a schedule with `answer`; gives the requests and the edits. */
function blog(answer: Response = json(200, { posts: [{ ...DRAFT, status: "scheduled" }] })) {
  const edits: PostEdit[] = [];
  const sent = adminBlog({
    "GET /calendar/": () => json(200, { calendar: CALENDAR }),
    "GET /calendar/plan/": () => json(200, { plan: PLAN }),
    [`GET /posts/${IDEA.id}/`]: () => json(200, { posts: [DRAFT] }),
    [`PUT /posts/${IDEA.id}/`]: async (request) => {
      const body: { posts: PostEdit[] } = await request.json();
      edits.push(...body.posts);
      return answer;
    },
  });
  return { sent, edits };
}

/** The day cell of `day` in the month grid. */
function cell(day: string): HTMLElement {
  const found = document.querySelector<HTMLElement>(`td[data-day="${day}"]`);
  if (found === null) {
    throw new Error(`no cell for ${day}`);
  }
  return found;
}

/** Drops the draft `post` on the day `day`, as a mouse would. */
function drop(post: CalendarPost, day: string) {
  const dataTransfer = { getData: (type: string) => (type === DRAGGED ? post.id : "") };
  fireEvent.dragOver(cell(day), { dataTransfer });
  fireEvent.drop(cell(day), { dataTransfer });
}

describe("the content calendar", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(new Date(2026, 10, 10, 9, 0));
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it("shows each post on its day, linked to the editor, in whole weeks", async () => {
    signedIn();
    const { sent } = blog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: calendarWords.nav }));
    const title = { level: 1, name: calendarWords.title };
    expect(await screen.findByRole("heading", title)).toBeVisible();
    const month = await screen.findByRole("table", { name: "November 2026" });
    const live = within(month).getByRole("link", { name: LIVE.title });
    expect(live.getAttribute("href")).toContain(`/posts/${LIVE.id}`);
    expect(within(cell("2026-11-20")).getByRole("link", { name: NEXT.title })).toBeVisible();
    const asked = new URL(sent.find((request) => request.url.includes("/calendar/?"))?.url ?? "");
    expect(asked.searchParams.get("from")).toBe("2026-10-26");
    expect(asked.searchParams.get("to")).toBe("2026-12-06");
    expect(await axeViolations()).toEqual([]);
  });

  it("lists the month's days with posts in the list view", async () => {
    signedIn();
    blog();
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    await userEvent.click(screen.getByRole("button", { name: calendarWords.list }));
    const list = screen.getByRole("list", { name: "November 2026" });
    const days = within(list).getAllByRole("heading", { level: 3 });
    expect(days.map((day) => day.textContent)).toEqual([
      calendarDay("2026-11-03"),
      calendarDay("2026-11-20"),
    ]);
    expect(within(list).getByRole("link", { name: NEXT.title })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("moves a month at a time and back to this month", async () => {
    signedIn();
    blog();
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    await userEvent.click(screen.getByRole("button", { name: calendarWords.next }));
    expect(await screen.findByRole("table", { name: "December 2026" })).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: calendarWords.thisMonth }));
    expect(await screen.findByRole("table", { name: "November 2026" })).toBeVisible();
  });

  it("schedules a draft for the day the writer types, without a mouse", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/calendar");
    const open = await screen.findByRole("button", { name: calendarWords.scheduleFor(IDEA.title) });
    await userEvent.click(open);
    const form = screen.getByRole("form", { name: calendarWords.scheduleFor(IDEA.title) });
    const field = within(form).getByLabelText(calendarWords.day);
    expect(field).toHaveFocus();
    fireEvent.change(field, { target: { value: "2026-11-24" } });
    await userEvent.click(within(form).getByRole("button", { name: calendarWords.send }));
    const said = calendarWords.scheduled(IDEA.title, calendarDay("2026-11-24"));
    expect(await screen.findByText(said)).toBeVisible();
    expect(edits).toEqual([
      {
        updated_at: DRAFT.updated_at,
        status: "scheduled",
        published_at: new Date("2026-11-24T12:00").toISOString(),
      },
    ]);
  });

  it("schedules a draft dropped on a day, and ignores a drop on a day gone by", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    drop(IDEA, "2026-11-09");
    expect(edits).toEqual([]);
    drop(IDEA, "2026-11-27");
    const said = calendarWords.scheduled(IDEA.title, calendarDay("2026-11-27"));
    expect(await screen.findByText(said)).toBeVisible();
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]?.published_at).toBe(new Date("2026-11-27T12:00").toISOString());
  });

  it("shows the planner's suggested days and changes nothing until asked", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    const day = cell("2026-11-12");
    expect(await within(day).findByRole("link", { name: IDEA.title })).toBeVisible();
    expect(within(day).getByText(`${calendarWords.suggested}:`)).toBeInTheDocument();
    const plan = screen.getByRole("region", { name: calendarWords.plan });
    expect(within(plan).getByText(calendarWords.planHint(2))).toBeVisible();
    expect(within(plan).getByText(PLAN.warnings[0]?.what ?? "")).toBeVisible();
    expect(edits).toEqual([]);
    expect(await axeViolations()).toEqual([]);
  });

  it("schedules each suggested draft on its day with one button", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/calendar");
    const plan = await screen.findByRole("region", { name: calendarWords.plan });
    const apply = within(plan).getByRole("button", { name: calendarWords.apply(1) });
    expect(apply).toHaveAccessibleDescription(calendarWords.applyHint);
    await userEvent.click(apply);
    expect(await screen.findByText(calendarWords.applied(1))).toBeVisible();
    expect(edits).toEqual([
      {
        updated_at: DRAFT.updated_at,
        status: "scheduled",
        published_at: new Date("2026-11-12T12:00").toISOString(),
      },
    ]);
  });

  it("keeps the calendar when the server makes no plan", async () => {
    signedIn();
    adminBlog({ "GET /calendar/": () => json(200, { calendar: CALENDAR }) });
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    expect(screen.queryByRole("region", { name: calendarWords.plan })).toBeNull();
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("says why a draft the checks block stays a draft", async () => {
    signedIn();
    blog(json(422, REFUSED));
    renderAt("/calendar");
    await screen.findByRole("table", { name: "November 2026" });
    drop(IDEA, "2026-11-27");
    expect(await screen.findByRole("alert")).toHaveTextContent(REFUSED.title);
  });

  it("says what went wrong when the calendar cannot be read", async () => {
    signedIn();
    adminBlog({ "GET /calendar/": () => json(500, {}) });
    renderAt("/calendar");
    expect(await screen.findByRole("alert")).toBeVisible();
  });
});
