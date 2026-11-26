// The words of the two schema reports under the editor's "Your own schema"
// field. They sit in their own file so en.ts stays short.
export const schemaCheckWords = {
  schemaOrg: "Is it correct schema.org?",
  google: "Can Google show it as a rich result?",
  nothing: "Nothing to fix.",
  suggested: "Schema your post could carry",
  suggestedHint:
    "Khalid read the post and found things search engines understand better as schema.",
  sure: (confidence: number) => `${confidence} percent sure`,
  error: "Fix",
  warning: "Check",
  where: (path: string) => `At ${path}`,
  checking: "Checking your schema",
  failed: "Khalid could not check this schema. It is checked again when it saves.",
  stale:
    "The schema changed since this fix was offered, so it was not made. Leave the field to check it again.",
};
