---
title: "Tidewater Ferries: ferry timetables people can read at the pier"
slug: tidewater-ferry-timetables
date: 2026-04-14
reviewed: 2026-10-28
author: tobias-reyne
reviewed_by: dev-hallam
category: projects
description: How we redesigned the Tidewater Ferries timetables in 2026 so a passenger can find the next boat in seconds, at the pier, on paper and on a phone.
focus_keyword: ferry timetables
featured: true
tags: [wayfinding, information design, case study]
image:
  src: lead.jpg
  alt: The open upper deck of a white passenger ferry on a misty grey morning, its green railings running towards the bow and the shore lost in fog
---

We redesigned the Tidewater Ferries timetables in 2026 so that a passenger at the pier can find the next boat in a few seconds. The new ferry timetables show one route per panel, the next departures first, and the same layout on the pier boards, the printed leaflets and the website.

| Client | Year | Discipline |
|---|---|---|
| Tidewater Ferries, Eastbridge | 2026 | Information design, signage |

## The problem with the old ferry timetables

Tidewater runs three routes from Eastbridge: to Gull Island, to Saltby across the estuary and a summer service to the lighthouse. The old timetable put all three routes in one large grid, with summer and winter times side by side and a dozen footnotes in small type.

Regular passengers had learned to read it. Visitors had not. The crew told us the question they heard most was "is this the boat to Saltby?", asked by people standing in front of a sign that answered it.

[Information design](https://en.wikipedia.org/wiki/Information_design) is the work of making facts easy to find and use. Here the facts were simple: where, when and from which berth. The design had hidden them.

## What we changed

We sat at the pier for three mornings and two evenings, watching people read the old board. Most read it from the top left, gave up on the grid after a few seconds and asked someone. We built the new ferry timetables around four decisions.

| Decision | Before | After |
|---|---|---|
| One route per panel | Three routes in one grid | One panel per route, each with its own colour |
| Today first | Summer and winter side by side | Only the current season on the pier; the other season online and in the leaflet |
| Next boats first | Every departure of the day, same size | The next three departures large, the rest of the day below |
| No footnotes | Twelve footnotes with symbols | Each exception written in words on the line it affects |

### A colour for each route

Each route has a colour that appears on its timetable panel, its berth sign and the band painted on its boat. Gull Island is green, Saltby is orange and the lighthouse service is blue. A passenger who has seen the colour on the boat can match it to the board without reading a word.

We tested the three colours with the crew and with passengers who have colour blindness. The panels also carry the route name in large letters, so colour is never the only clue.

### Words in place of symbols

The old timetable used a dagger for "not on Sundays", a star for "school days only" and a small anchor for "calls at Saltby Point on request". Nobody remembered which was which. Now each line says what it means: "Not on Sundays". It takes more space and saves far more time.

:::figure{src="rope.jpg" alt="A teal mooring rope coiled into a tight flat spiral on weathered pale wooden deck boards, the kind of detail we matched our ferry timetables colours to"}
The Gull Island green was matched to the mooring ropes the crew already use on that route.
:::

## The same layout everywhere

A timetable is only useful if it agrees with itself. The pier boards, the leaflets in the ticket office and the website now come from one set of data, kept by the Tidewater office in a spreadsheet. Dev built a small page on their website that reads the spreadsheet, so a change made in the office appears online the same day. When the season changes, the print files for the pier boards are made from the same sheet.

## Timeline

| Stage | Weeks | What happened |
|---|---|---|
| Watching | 1 to 2 | Three mornings and two evenings at the pier, crew interviews |
| Design | 3 to 6 | Panel layout, route colours, leaflet and web page |
| Testing | 7 | Paper boards at the pier for a week, with a note asking for comments |
| Making | 8 to 10 | Enamel panels for the pier, printed leaflets, the web page live |

:::callout{kind=note}
The paper test turned up one change we had not expected. Passengers waiting at the far end of the pier could not see the board, so a second, smaller board now stands at the gangway.
:::

## Results after one season

The crew say "is this the boat to Saltby?" has almost stopped. Tidewater printed 40 percent fewer leaflets in the first season, because people now check times on their phones and find the same layout there. The ferry timetables also gave us a method we use again, as in [the library signs at Calder Street](/blog/calder-street-library-wayfinding/): watch first, then remove everything people do not need. We describe how we try a design like this before it is made in [testing signs at full size](/blog/test-signs-on-site/).
