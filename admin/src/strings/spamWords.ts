// The words for moving a reader's comment to spam and back, beside en.ts, which is near its size limit.
export const spamWords = {
  spam: "Spam",
  spamLabel: (name: string) => `Mark the comment from ${name} as spam`,
  notSpam: "Not spam",
  notSpamLabel: (name: string) => `The comment from ${name} is not spam`,
  marked: (name: string) =>
    `Marked as spam. The comment from ${name} is hidden from its post and kept under Spam.`,
  back: (name: string) => `Moved back. The comment from ${name} waits for your approval again.`,
};
