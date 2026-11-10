// The Snippets screen's words, kept beside en.ts so that file stays short.

export const snippetWords = {
  title: "Snippets",
  lede: "A snippet is a group of blocks you saved once, such as an affiliate disclosure. Every post that places it shows the same blocks. Change a snippet here and every one of those posts changes with it. To make a new one, select blocks in a post and choose Save as a snippet.",
  back: "Back to the list",
  list: {
    title: "Your snippets",
    loading: "Reading your snippets",
    name: "Name",
    language: "Language",
    changed: "Last changed",
    action: "Action",
    edit: "Edit",
    editLabel: (name: string, language: string) => `Edit ${name}, ${language}`,
    remove: "Delete",
    removeLabel: (name: string, language: string) => `Delete ${name}, ${language}`,
  },
  edit: {
    title: (name: string) => `Edit "${name}"`,
    lede: (language: string) =>
      `Every ${language} post that places this snippet shows your change as soon as you save it, live posts included.`,
    name: "Name",
    nameHint: (slug: string) =>
      `As writers see it in the editor's list of snippets. Posts keep finding it, because they place it by its short name ${slug}, which stays.`,
    blocks: "Blocks",
    blocksHint:
      "Written in markdown, the plain text format your posts are saved in. A blank line starts a new paragraph.",
    submit: "Save the snippet",
    done: (name: string) => `Saved the snippet "${name}".`,
  },
  remove: {
    title: (name: string) => `Delete "${name}"`,
    lede: "A post that still places this snippet shows nothing in its place, and the publish check tells that post's writer about the gap. The posts themselves stay.",
    submit: "Delete the snippet",
    done: (name: string) => `Deleted the snippet "${name}".`,
  },
} as const;
