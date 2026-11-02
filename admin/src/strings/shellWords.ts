// The words of the frame around every signed-in screen: the side menu's
// groups, the top bar and the account menu.
export const shellWords = {
  groups: {
    write: "Write",
    grow: "Grow",
    site: "Site",
    settings: "Settings",
  },
  openMenu: "Open the menu",
  closeMenu: "Close the menu",
  newPost: "New post",
  accountMenu: "Your account menu",
  byEmail: "You signed in with your email.",
  byToken: "You signed in with a staff token.",
  search: {
    open: "Search",
    label: "Search posts and screens",
    screens: "Screens",
    posts: "Posts",
    latest: "Latest posts",
    nothing: "Nothing matches those words. Try fewer words, or part of a title.",
  },
} as const;
