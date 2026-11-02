import { Outlet, useNavigate } from "@tanstack/react-router";
import { useEffect, useRef, useState } from "react";
import { endSession } from "../api/account";
import { mountPath } from "../api/mount";
import { forgetKey, readCsrf } from "../auth/session";
import { en } from "../strings/en";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";

/** The side menu, the top bar and the screen inside them, around every signed-in screen. */
export function Shell() {
  const navigate = useNavigate();
  const [open, setOpen] = useState(false);
  const opener = useRef<HTMLButtonElement>(null);
  const closer = useRef<HTMLButtonElement>(null);

  // On a small screen the menu covers the page: it takes the focus when it
  // opens, and Escape gives the focus back to the button that opened it.
  useEffect(() => {
    if (!open) {
      return;
    }
    closer.current?.focus();
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
        opener.current?.focus();
      }
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open]);

  const signOut = async () => {
    // An email sign-in also ends on the server, so its cookie stops working.
    if (readCsrf() !== null) {
      await endSession();
    }
    forgetKey();
    await navigate({ to: "/sign-in" });
  };
  return (
    <>
      <a className="skip" href="#main">
        {en.skip}
      </a>
      <div className="app">
        <Sidebar open={open} close={() => setOpen(false)} closer={closer} />
        <div className="app-main">
          <TopBar open={open} onMenu={() => setOpen(true)} opener={opener}>
            <a className="btn ghost" href={mountPath()}>
              {en.viewBlog}
            </a>
            <button type="button" className="btn" onClick={signOut}>
              {en.signOut}
            </button>
          </TopBar>
          <main id="main" className="page">
            <Outlet />
          </main>
        </div>
      </div>
    </>
  );
}
