# 0000. The planner's days show on the calendar, and the owner takes them

**Number:** next free. **Status:** Proposed, 2026-11-10. GEO phase, task M4 (KH-EDT-4, KH-OPS-2). Gap audit 3, backlog row 18.

## Context

ADR "publishing planner suggests only" built `khalid plan` and left the Admin API route and the admin screen for later. Gap audit 3 found the planner CLI-only: no route, no suggested days on the calendar, and no way to apply them. That ADR's rule still holds: the planner never moves a date or publishes by itself (decision D4), and publishing has one door (decision of 2026-10-27).

## Decision

- `GET /api/admin/v1/calendar/plan/` answers the same plan as `khalid plan`, from the same input: the publish door reads every post through the new `PlanPosts` trait and adds each draft's schedule. `per_week` from 1 to 14 sets the pace; left out, the site's own pace holds. It needs `posts:write`, like the calendar, and writes nothing.
- The plan hangs on the publish door (`PublishDoor::with_planner`), which the calendar route already holds, rather than on a new Admin API desk.
- No apply route. The admin takes a suggestion by editing the post to `scheduled`, the same edit a draft dropped on a calendar day makes, at noon in the writer's time zone. One edit per post keeps the one door, the linter, the webhooks and the audit log exactly as for any schedule, and an apply route would only repeat it. A client of the API does the same.
- The calendar shows each suggested day as a dashed chip, says the pace and where it came from, lists the warnings, and has one button that schedules every suggestion shown, after saying what will happen. The pace is the site's own on the screen; the API takes another.

## Consequences

- The owner can see and take the plan without the command line, and still nothing changes until they press the button.
- `khalid plan` and the route share `PlannedPost::plan_post`, so they cannot disagree about which draft is ready, blocked or scheduled.
- `khalid plan` still has no `--apply`; the admin and the post edit cover it. A later task can add it on the same edit.
- The plan is made on request, judging every post with the lint gate, as the posts list does. A site with thousands of drafts pays for that on each calendar visit; caching it is a later choice if a budget shows it.
