// The words of the content calendar: what is planned, scheduled and live,
// by day, how to give a draft its day, and the days the planner suggests.

export const calendarWords = {
  nav: "Calendar",
  title: "Calendar",
  lede: "Every scheduled and live post on the day it goes live, and every draft on the day planned for it. Drag a draft onto a day, or use its Schedule button, to publish it that day at 12:00.",
  loading: "Reading the calendar",
  views: "Show the calendar as",
  legend: "What the colours mean",
  month: "Month",
  list: "List",
  previous: "Previous month",
  next: "Next month",
  thisMonth: "This month",
  today: "Today",
  drafts: "Drafts to schedule",
  draftsHint:
    "Drag one onto a day, or press Schedule and pick the day. Publishing checks the post first, so one the checks block stays a draft.",
  noDrafts: "No drafts wait for a day. A new draft shows here until you schedule it.",
  none: "No posts this month. A post shows here once it is scheduled, live, or a draft given a day.",
  count: (posts: number) => (posts === 1 ? "1 post" : `${posts} posts`),
  open: (title: string) => `Open ${title}`,
  schedule: "Schedule",
  scheduleFor: (title: string) => `Schedule ${title}`,
  day: "Day it goes live",
  dayHint: "It goes live at 12:00 that day, in your time zone.",
  send: "Schedule it",
  cancel: "Cancel",
  scheduled: (title: string, day: string) => `${title} is scheduled for ${day} at 12:00.`,
  dropHere: "Drop here to schedule",
  suggested: "Suggested",
  plan: "Suggested days",
  planHint: (perWeek: number) =>
    `Khalid gives each draft that passes its checks a day, at your usual pace of ${perWeek === 1 ? "1 post" : `${perWeek} posts`} a week, each topic's main post first. Nothing changes until you schedule them.`,
  noSuggestions:
    "No draft passes its checks yet, so none has a day. Finish a draft and it gets one here.",
  applyHint:
    "Each goes live at 12:00 on its day, in your time zone. You can still move any of them.",
  apply: (drafts: number) =>
    drafts === 1 ? "Schedule 1 suggested draft" : `Schedule ${drafts} suggested drafts`,
  applied: (drafts: number) =>
    drafts === 1
      ? "1 draft is scheduled on its suggested day."
      : `${drafts} drafts are scheduled on their suggested days.`,
  past: {
    what: "That day has already passed.",
    why: "A post goes live at the time it is scheduled for, so a day gone by would publish it at once.",
    fix: "Pick today or a day to come.",
  },
};
