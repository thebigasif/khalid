// The Subscribers screen's words, kept beside en.ts so that file stays short.
import type { SendReadiness, SubscriberStatus } from "../api/subscribers";

const plural = (count: number, one: string, many: string) =>
  `${count.toLocaleString("en")} ${count === 1 ? one : many}`;

export const subscriberWords = {
  nav: "Subscribers",
  title: "Subscribers",
  lede: "The readers who signed up for your newsletters. Readers join only through the sign-up form and the link in its email, so everyone here asked to be here. Only the owner and admins see this screen.",
  loading: "Loading your newsletters…",
  lists: {
    title: "Your newsletters",
    readers: (count: number) => plural(count, "reader gets it", "readers get it"),
    waiting: (count: number) =>
      count === 0
        ? "Everyone who signed up has confirmed."
        : `${plural(count, "more has", "more have")} not opened their confirmation email yet.`,
    form: "Sign-up form for your main website",
    formHint:
      'Paste this into any page of your main site. It works with scripts off, and readers land on the blog\'s "Check your inbox" page.',
    copy: "Copy the form",
    copied: "The form is copied. Paste it into a page of your main site.",
  },
  send: {
    title: "Send a post",
    lede: "A published post goes to one newsletter once, with a link to read it on the blog and each reader's own unsubscribe link. An edit never sends it again.",
    post: "Post",
    pick: "Pick a published post",
    noPosts: "No post is published yet. Publish one, then send it here.",
    list: "Newsletter",
    listOption: (name: string, readers: number) =>
      `${name} (${plural(readers, "reader", "readers")})`,
    labels: "Only readers with these labels",
    labelsHint: "Leave empty to send to everyone on the newsletter. Separate labels with commas.",
    submit: (readers: number) => `Send to ${plural(readers, "reader", "readers")}`,
    submitLabelled: "Send to the labelled readers",
    sending: "Sending…",
    started: (title: string) =>
      `${title} is on its way. Its numbers show under Sent emails once the copies are out.`,
    notReady: {
      no_mail: {
        what: "This site cannot send email yet.",
        why: "A newsletter goes out the way [mail] in site.toml names, and there is none.",
        fix: 'Add a [mail] table to site.toml, as docs/deploy.md shows under "Sending email".',
      },
      no_postal_address: {
        what: "Khalid sends no newsletter until the site has a postal address.",
        why: "Anti-spam laws ask every newsletter email to show where the sender can be reached by post.",
        fix: 'Add postal_address = "..." under [mail] in site.toml.',
      },
    } satisfies Record<Exclude<SendReadiness, "ready">, { what: string; why: string; fix: string }>,
  },
  sent: {
    title: "Sent emails",
    none: "No email has gone out yet.",
    post: "Post",
    list: "Newsletter",
    when: "Sent",
    copies: "Copies sent",
    refused: "Refused",
    left: "Unsubscribed with its link",
    totals: "Numbers are totals per email, never per person. Khalid does not count opens.",
  },
  readers: {
    title: "Readers",
    which: "Which readers",
    statuses: {
      all: "Everyone",
      confirmed: "Getting emails",
      pending: "Not confirmed yet",
      unsubscribed: "Left",
    } satisfies Record<SubscriberStatus | "all", string>,
    find: "Find an address",
    list: "Newsletter",
    anyList: "Any",
    address: "Address",
    stands: "Stands",
    newsletters: "Newsletters",
    labels: "Labels",
    joined: "Signed up",
    actions: "Actions",
    none: "No reader matches.",
    removeLabel: "Remove",
    removeLabelName: (label: string, email: string) => `Take the label ${label} away from ${email}`,
    addLabel: "Add a label",
    label: "Label",
    labelHint: "Lowercase letters, digits and hyphens, such as paid.",
    add: "Add",
    forget: "Forget",
    forgetName: (email: string) => `Forget ${email}`,
    forgetSure: (email: string) =>
      `Forget ${email}? Their address, newsletters and labels are removed for good.`,
    forgetYes: "Forget them",
    keep: "Keep",
    forgotten: (email: string) => `${email} is forgotten.`,
    labelled: (email: string) => `The labels of ${email} are saved.`,
    shown: (from: number, to: number, total: number) =>
      `${from.toLocaleString("en")} to ${to.toLocaleString("en")} of ${plural(total, "reader", "readers")}`,
    previous: "Previous 50",
    next: "Next 50",
    forgetNote:
      "Forget removes a reader's address, newsletters and labels for good, for a reader who asks. A reader who wants fewer emails can unsubscribe with the link in any of them.",
  },
};
