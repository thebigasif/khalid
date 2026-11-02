// The words of the sign-in screens. The admin's other words are in en.ts.

export const signInWords = {
  title: (blog: string) => `Sign in to ${blog}`,
  thisBlog: "your blog",
  passkey: {
    submit: "Sign in with a passkey",
    checking: "Waiting for this device",
    hint: "A passkey signs you in with your fingerprint, your face or your device's PIN.",
  },
  or: "or use your email",
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
  welcome: {
    title: "You're signed in",
    lede: "Add a passkey so you can sign in next time without a link.",
    noPasskeys:
      "This browser cannot make passkeys. Choose a password so you can sign in next time.",
    add: "Add a passkey",
    adding: "Waiting for this device",
    passkeyName: "First passkey",
    password: "Choose a password instead",
    skip: "Skip for now and open the admin",
    addedTitle: "Your passkey is ready",
    added: "Next time, press Sign in with a passkey. You can rename this one under Passkeys.",
    open: "Open the admin",
  },
  problems: {
    noPasskey: {
      what: "This device did not sign you in.",
      why: "It was cancelled, it timed out, or this device has no passkey for this blog.",
      fix: "Press Sign in with a passkey again, or sign in with your email and password.",
    },
    noPasskeys: {
      what: "Nobody has added a passkey to this blog yet.",
      why: "A passkey is added after a first sign-in, under Your account.",
      fix: "Sign in with your email and password, or with a sign-in link.",
    },
  },
} as const;
