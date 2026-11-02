// The admin's line icons, drawn for it on a 24 unit grid with one stroke.
// Every icon sits beside words that name it, so screen readers skip it.
const PATHS = {
  home: "M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z",
  posts: "M14 3H6a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V8zM14 3v5h5M9 13h6M9 17h6",
  media: "M4 4h16v16H4zM4 16l5-5 4 4 2-2 5 5M16 8.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0",
  comments: "M4 5h16v11H9l-5 4z",
  analytics: "M5 20v-9M12 20V4M19 20v-6",
  search: "M11 18a7 7 0 1 1 0-14 7 7 0 0 1 0 14zM16 16l5 5",
  link: "M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1",
  look: "M12 3a9 9 0 1 0 0 18c.9 0 1.5-.7 1.5-1.5 0-.9-.7-1.3-.7-2.2 0-.9.7-1.3 1.7-1.3H17a4 4 0 0 0 4-4c0-5-4-9-9-9zM7.5 11.5h.01M10.5 7.5h.01M15.5 8.5h.01",
  person: "M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM4 21a8 8 0 0 1 16 0",
  redirect: "M5 12h14M13 6l6 6-6 6",
  braces:
    "M8 4H7a2 2 0 0 0-2 2v4l-2 2 2 2v4a2 2 0 0 0 2 2h1M16 4h1a2 2 0 0 1 2 2v4l2 2-2 2v4a2 2 0 0 1-2 2h-1",
  sliders: "M4 7h16M4 17h16M9 4v6M15 14v6",
  team: "M9 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM2 21a7 7 0 0 1 14 0M16 3.5a4 4 0 0 1 0 7M18 14a7 7 0 0 1 4 7",
  key: "M8 21a5 5 0 1 1 3.5-8.5L20 4M16 8l3 3M13.5 10.5l2 2",
  bolt: "M13 2 4 14h7l-1 8 9-12h-7z",
  shield: "M12 3l8 3v6c0 5-3.5 8-8 9-4.5-1-8-4-8-9V6z",
  list: "M9 6h11M9 12h11M9 18h11M4 6h.01M4 12h.01M4 18h.01",
  trash: "M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3",
  account:
    "M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM12 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM6.5 18.5a6 6 0 0 1 11 0",
  plus: "M12 5v14M5 12h14",
  menu: "M4 6h16M4 12h16M4 18h16",
  close: "M6 6l12 12M18 6 6 18",
  chevron: "M6 9l6 6 6-6",
  external: "M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5",
  signOut: "M15 4h4a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1h-4M10 17l5-5-5-5M15 12H3",
  alert:
    "M12 9v4M12 17h.01M10.3 3.9 2.4 18a2 2 0 0 0 1.7 3h15.8a2 2 0 0 0 1.7-3L13.7 3.9a2 2 0 0 0-3.4 0z",
  clock: "M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM12 7v5l3 2",
} as const;

/** The name of one of the admin's icons. */
export type IconName = keyof typeof PATHS;

/** One line icon, 16 px by default, in the colour of the text around it. */
export function Icon({ name }: { name: IconName }) {
  return (
    <svg className="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false">
      <path d={PATHS[name]} />
    </svg>
  );
}
