// The words of the sign-in screens. The admin's other words are in en.ts.

export const signInWords = {
  title: (blog: string) => `Sign in to ${blog}`,
  thisBlog: "your blog",
  email: {
    form: "Sign in with your email",
    address: "Email",
    password: "Password",
    submit: "Sign in",
    checking: "Checking your email and password",
  },
  token: {
    open: "Sign in with a staff token",
    label: "Staff token",
    hint: "A long code the blog's owner can make for you. This tab keeps it until you sign out.",
    submit: "Sign in with this token",
    checking: "Checking the token",
  },
  newHere: "No account yet? Ask the blog's owner to add you.",
  owners: "Owners get their first sign-in link from",
  ownerCommand: "khalid staff owner",
  link: {
    title: "Signing you in",
    checking: "Checking your sign-in link",
    back: "Go to the sign-in page",
  },
} as const;
