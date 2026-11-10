import { waitFor } from "@testing-library/react";

/** The one part of a router the clean-up needs: whether a page is still loading. */
interface Loading {
  readonly state: { readonly status: "pending" | "idle" };
}

// Every router the running test made. A tiny module of its own, so setup.ts
// can wait on them without loading the whole route tree into every test file.
const made: Loading[] = [];

/** Remembers a router so the clean-up after the test waits for its pages. */
export function track(router: Loading) {
  made.push(router);
}

/**
 * Waits until no router the test made is still loading a page, then forgets them.
 * A test that ends mid-navigation, say while the editor's code still loads,
 * would otherwise let the router finish after jsdom is gone, and the run fails
 * with "window is not defined" though every test passed.
 */
export async function settleRouters() {
  const routers = made.splice(0);
  // waitFor yields to the event loop even when it has nothing to wait for.
  // In a test with no router, that pause let an editor's queued work run
  // before the clean-up, so skip it.
  if (routers.length === 0) {
    return;
  }
  await waitFor(() => {
    for (const router of routers) {
      if (router.state.status !== "idle") {
        throw new Error("A page the test opened was still loading when the test ended.");
      }
    }
  });
}
