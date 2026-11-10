// The Forms screen's words, kept beside en.ts so that file stays short.
export const formWords = {
  title: "Forms",
  lede: "What visitors sent through the forms in your posts, newest first. Answers hold people's email addresses, so only owners and admins can read them.",
  terms: [
    {
      term: "Form",
      meaning:
        "A form you add to a post with a form block, such as a contact form. Each form has a short name, such as contact. Answers a robot sent are left out.",
    },
  ],
  loading: "Loading the answers",
  none: "No one has sent a form yet. Add a form block to a post, and what visitors send shows here.",
  which: "Form",
  option: (id: string, answers: number) =>
    answers === 1 ? `${id}, 1 answer` : `${id}, ${answers} answers`,
  count: (answers: number, id: string, newest: string) =>
    `${answers === 1 ? "1 answer" : `${answers} answers`} to ${id}. The newest came on ${newest}.`,
  sentOn: "on",
  page: (page: number, pages: number) => `Page ${page} of ${pages}`,
  newer: "Newer answers",
  older: "Older answers",
};
