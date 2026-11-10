// The honest update switch in the editor (F2): the one way a live post's
// "Updated" date moves.

/** The longest what-changed note the API takes, in characters. */
export const MAX_NOTE = 160;

export const realUpdateWords = {
  title: "Real update",
  hint: 'Readers and search engines see an "Updated" date only when you say the post really changed. Leave this off for a typo or a fixed link.',
  last: (day: string, note: string) => `Last real update: ${day}. ${note}`,
  none: "No real update yet, so readers see the day it went live.",
  on: "This change is a real update",
  note: "What changed",
  noteHint: (length: number) =>
    `One short sentence readers see beside the new date. ${length} of ${MAX_NOTE} characters. The date becomes today with your next change to the text.`,
  empty: "Say what changed, or the date stays as it is.",
  tooLong: `Keep it to ${MAX_NOTE} characters, or the date stays as it is.`,
} as const;
