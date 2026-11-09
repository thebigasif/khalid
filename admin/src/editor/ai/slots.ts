import { type RefObject, useEffect, useState } from "react";

/**
 * The element named `name` that the AI suggestions keep at the end of
 * `parent`, made when it is not there yet. React leaves an element it did
 * not make alone, and a fresh one is made if the form draws `parent` anew.
 */
export function slot(parent: Element, name: string): HTMLElement {
  const found = parent.querySelector(`:scope > [data-ai="${name}"]`);
  if (found instanceof HTMLElement) {
    return found;
  }
  const host = document.createElement("div");
  host.dataset.ai = name;
  parent.append(host);
  return host;
}

/** Whether two lists of slots hold the same elements, in order. */
function same<T extends { host: HTMLElement }>(a: T[], b: T[]): boolean {
  return a.length === b.length && a.every((one, at) => one.host === b[at]?.host);
}

/**
 * The slots `find` reads in the form under `root`, read again whenever the
 * form's elements change, and none while `ready` is false.
 */
export function useSlots<T extends { host: HTMLElement }>(
  root: RefObject<HTMLElement | null>,
  ready: boolean,
  find: (root: HTMLElement) => T[],
): T[] {
  const [slots, setSlots] = useState<T[]>([]);
  useEffect(() => {
    const element = root.current;
    if (!ready || element === null) {
      return;
    }
    const read = () => {
      const found = find(element);
      setSlots((now) => (same(now, found) ? now : found));
    };
    read();
    const watching = new MutationObserver(read);
    watching.observe(element, { childList: true, subtree: true });
    return () => watching.disconnect();
  }, [root, ready, find]);
  return ready ? slots : [];
}
