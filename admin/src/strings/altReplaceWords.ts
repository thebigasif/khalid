// The words of the Media screen's alt text find-and-replace, kept beside en.ts so that file stays short.
export const altReplaceWords = {
  title: "Change words in alt text",
  lede: "Changes the same words in the alt text of every file at once, such as an old company name. Letter case counts. You see every change before anything is saved.",
  find: "Find",
  replace: "Replace with",
  replaceHint: "Leave it empty to remove the words.",
  show: "Show the changes",
  checking: "Looking through your library",
  none: "No alt text in your library has these words, so nothing would change.",
  found: (count: number) =>
    count === 1
      ? "1 file has these words. Check the change, then save."
      : `${count} files have these words. Check each change, then save.`,
  table: "Alt text changes",
  file: "File",
  before: "Now",
  after: "After",
  consequence:
    "This changes the alt text kept in your library. A post you already wrote keeps the alt text it has. The new words go into a post the next time you add the file from the library.",
  apply: (count: number) => (count === 1 ? "Change 1 file" : `Change ${count} files`),
  saving: "Saving the changes",
  done: (count: number) =>
    count === 1
      ? "Done. 1 file has the new alt text."
      : `Done. ${count} files have the new alt text.`,
  empty: "No alt text",
};
