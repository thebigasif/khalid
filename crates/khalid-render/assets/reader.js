// Polish on top of pages that read completely without it (RULES.md BL-WEB-2):
// fill the reading progress bar, copy a heading's address, open an
// enlarged image in a dialog, open search in a dialog from the header or
// with / or Ctrl+K, show results while typing a search, mark the section
// being read in the contents, open the contents as a sheet on a phone,
// and preview a linked post.
// The words it shows come from the script tag's data attributes.
(() => {
  "use strict";
  const words = document.currentScript ? document.currentScript.dataset : {};

  // Says something to screen readers and shows it briefly.
  const status = document.createElement("p");
  status.className = "reader-status";
  status.setAttribute("role", "status");
  document.body.append(status);
  const say = (text) => {
    status.textContent = text;
    setTimeout(() => {
      status.textContent = "";
    }, 3000);
  };

  // The progress bar fills with the scroll where CSS alone cannot drive it.
  const bar = document.querySelector(".reading-progress");
  if (bar && !(window.CSS && CSS.supports("animation-timeline: scroll()"))) {
    let queued = false;
    const fill = () => {
      queued = false;
      const room = document.documentElement.scrollHeight - innerHeight;
      bar.style.setProperty("--read", room > 0 ? Math.min(1, scrollY / room) : 1);
    };
    const queue = () => {
      if (!queued) requestAnimationFrame(fill);
      queued = true;
    };
    addEventListener("scroll", queue, { passive: true });
    addEventListener("resize", queue);
    fill();
  }

  // Each heading's # link becomes a button that copies the section's
  // address and says so. Where the page cannot copy, the link stays.
  if (navigator.clipboard) {
    for (const link of document.querySelectorAll("a.heading-link")) {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "heading-link copy-link";
      button.textContent = "#";
      button.dataset.href = link.getAttribute("href");
      button.dataset.tip = words.copy || "";
      button.setAttribute("aria-label", words.copy || "");
      link.replaceWith(button);
    }
    document.addEventListener("click", (event) => {
      const button = event.target.closest("button.copy-link");
      if (!button) return;
      const hash = button.dataset.href;
      navigator.clipboard.writeText(new URL(hash, location.href).href).then(() => {
        history.replaceState(null, "", hash);
        button.dataset.tip = words.copied || "";
        button.dataset.done = "";
        say(words.copied || "");
        setTimeout(() => {
          button.dataset.tip = words.copy || "";
          delete button.dataset.done;
        }, 2000);
      });
    });
  }

  // A reduced image opens at full size in a dialog that Escape closes.
  // The browser returns focus to the image's link when it closes.
  document.addEventListener("click", (event) => {
    const link = event.target.closest("a.enlarge");
    if (!link || typeof HTMLDialogElement !== "function") return;
    event.preventDefault();
    const dialog = document.createElement("dialog");
    dialog.className = "lightbox";
    const figure = document.createElement("figure");
    const image = document.createElement("img");
    image.src = link.href;
    image.alt = link.querySelector("img")?.alt || "";
    figure.append(image);
    const caption = link.closest("figure")?.querySelector("figcaption");
    if (caption) {
      const text = document.createElement("figcaption");
      text.textContent = caption.textContent.trim();
      figure.append(text);
    }
    const close = document.createElement("button");
    close.type = "button";
    close.textContent = words.close || "";
    close.addEventListener("click", () => dialog.close());
    dialog.append(close, figure);
    dialog.addEventListener("click", (click) => {
      if (click.target === dialog) dialog.close();
    });
    dialog.addEventListener("close", () => dialog.remove());
    document.body.append(dialog);
    dialog.showModal();
  });

  // / or Ctrl+K or Cmd+K opens search from any page, unless the reader is typing.
  document.addEventListener("keydown", (event) => {
    const typing = event.target.closest("input, textarea, select, [contenteditable]");
    const slash = event.key === "/" && !typing && !event.ctrlKey && !event.metaKey;
    const command = event.key.toLowerCase() === "k" && (event.ctrlKey || event.metaKey);
    if (!slash && !command) return;
    const box = document.querySelector("main input[type=search]");
    const page = document.querySelector("a.search-link");
    if (!box && !page) return;
    event.preventDefault();
    if (box) box.focus();
    else if (page.hasAttribute("data-dialog") && openSearch) openSearch();
    else location.href = page.href;
  });

  // Results while typing in a search form. The server answers the form's
  // own address with `typing=1` as JSON, and the list is built from text,
  // never from markup. Without the script the form submits as before.
  const live = (form, id) => {
    const query = form.querySelector("input[type=search]");
    let timer = 0;
    let asked = 0;
    const show = (found) => {
      const section = document.createElement("section");
      section.setAttribute("aria-labelledby", id);
      const heading = document.createElement("h2");
      heading.id = id;
      heading.textContent = found.heading;
      section.append(heading);
      if (found.results.length === 0) {
        const nothing = document.createElement("p");
        nothing.textContent = found.nothing;
        section.append(nothing);
      } else {
        const list = document.createElement("ol");
        list.className = "results";
        for (const result of found.results) {
          const title = document.createElement("h3");
          const link = document.createElement("a");
          link.href = result.href;
          link.textContent = result.title;
          title.append(link);
          const text = document.createElement("p");
          for (const part of result.snippet) {
            const run = document.createElement(part.kind === "match" ? "mark" : "span");
            run.textContent = part.text;
            text.append(run);
          }
          const item = document.createElement("li");
          item.append(title, text);
          list.append(item);
        }
        section.append(list);
      }
      document.querySelector(`section[aria-labelledby=${id}]`)?.remove();
      form.after(section);
    };
    query.addEventListener("input", () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        const number = ++asked;
        const typed = query.value.trim();
        if (!typed) return;
        const address = new URL(form.action, location.href);
        address.search = new URLSearchParams({ q: typed, typing: "1" }).toString();
        fetch(address)
          .then((response) => (response.ok ? response.json() : null))
          .then((found) => {
            if (found && number === asked) show(found);
          })
          .catch(() => {});
      }, 250);
    });
  };
  const pageForm = document.querySelector("main form.search");
  if (pageForm && window.fetch) live(pageForm, "search-results");

  // The header's search link opens a search dialog with results while
  // typing. It is modal, so focus stays inside and Escape closes it.
  // Without the script the link opens the search page.
  const searchLink = document.querySelector("a.search-link[data-dialog]");
  let searchDialog = null;
  const openSearch =
    searchLink && window.fetch && typeof HTMLDialogElement === "function"
      ? () => {
          if (!searchDialog) {
            searchDialog = document.createElement("dialog");
            searchDialog.className = "search-dialog";
            searchDialog.setAttribute("aria-label", words.search || "");
            const close = document.createElement("button");
            close.type = "button";
            close.className = "search-close";
            close.textContent = words.close || "";
            close.addEventListener("click", () => searchDialog.close());
            const form = document.createElement("form");
            form.className = "search";
            form.setAttribute("role", "search");
            form.action = searchLink.href;
            form.method = "get";
            const label = document.createElement("label");
            label.htmlFor = "search-dialog-query";
            label.textContent = words.search || "";
            const input = document.createElement("input");
            input.id = "search-dialog-query";
            input.type = "search";
            input.name = "q";
            input.autocomplete = "off";
            const find = document.createElement("button");
            find.type = "submit";
            find.textContent = words.find || "";
            form.append(label, input, find);
            searchDialog.append(close, form);
            searchDialog.addEventListener("click", (click) => {
              if (click.target === searchDialog) searchDialog.close();
            });
            document.body.append(searchDialog);
            live(form, "search-dialog-results");
          }
          searchDialog.showModal();
          searchDialog.querySelector("input").focus();
        }
      : null;
  searchLink?.addEventListener("click", (event) => {
    if (!openSearch) return;
    event.preventDefault();
    openSearch();
  });

  // Every contents list marks the section on screen: the column, the box
  // and the sheet below.
  const contents = document.querySelector(".toc-side, .toc-box, .toc-inline nav");
  let reading = "";
  const mark = () => {
    for (const link of document.querySelectorAll(".toc-side a, .toc-box a, .toc-inline a, .toc-sheet a")) {
      if (reading && link.getAttribute("href") === `#${reading}`) {
        link.setAttribute("aria-current", "location");
      } else link.removeAttribute("aria-current");
    }
  };
  if (contents) {
    // The section being read is the last one whose heading has passed the
    // top third of the window.
    const headings = [...contents.querySelectorAll("a[href^='#']")]
      .map((link) => document.getElementById(link.getAttribute("href").slice(1)))
      .filter(Boolean);
    let queued = false;
    const find = () => {
      queued = false;
      const line = innerHeight / 3;
      const passed = headings.filter((heading) => heading.getBoundingClientRect().top < line);
      const now = passed.length ? passed[passed.length - 1].id : "";
      if (now !== reading) {
        reading = now;
        mark();
      }
    };
    addEventListener(
      "scroll",
      () => {
        if (!queued) requestAnimationFrame(find);
        queued = true;
      },
      { passive: true },
    );
    find();
  }

  // On a narrow screen a floating button opens the contents as a sheet: a
  // modal dialog, so focus stays inside and Escape closes it. Picking a
  // section closes it and goes there.
  if (contents && typeof HTMLDialogElement === "function") {
    const open = document.createElement("button");
    open.type = "button";
    open.className = "toc-button";
    open.textContent = words.contents || "";
    open.setAttribute("aria-haspopup", "dialog");
    const sheet = document.createElement("dialog");
    sheet.className = "toc-sheet";
    sheet.setAttribute("aria-label", words.contents || "");
    const top = document.createElement("div");
    top.className = "toc-sheet-top";
    const title = document.createElement("p");
    title.textContent = words.contents || "";
    const close = document.createElement("button");
    close.type = "button";
    close.textContent = words.close || "";
    close.addEventListener("click", () => sheet.close());
    top.append(title, close);
    sheet.append(top, contents.querySelector("ol").cloneNode(true));
    sheet.addEventListener("click", (event) => {
      if (event.target === sheet || event.target.closest("a")) sheet.close();
    });
    open.addEventListener("click", () => {
      mark();
      sheet.showModal();
      sheet.querySelector("[aria-current]")?.scrollIntoView({ block: "center" });
    });
    document.body.append(open, sheet);
  }

  // A link in a post's body to another page of the site shows that page's
  // summary while pointed at or focused, read from the page's own
  // description the first time. Escape hides it, and the pointer may move
  // onto it without it going away.
  if ("previews" in words) {
    const summaries = new Map();
    const summary = (link) => {
      if (!summaries.has(link.pathname)) {
        const read = fetch(link.pathname)
          .then((response) => (response.ok ? response.text() : ""))
          .then((html) => new DOMParser().parseFromString(html, "text/html"))
          .then((page) => page.querySelector("meta[name=description]")?.content || "")
          .catch(() => "");
        summaries.set(link.pathname, read);
      }
      return summaries.get(link.pathname);
    };
    const internal = (target) => {
      const link = target.closest?.(".post-body a[href]");
      const other = link && link.origin === location.origin && link.pathname !== location.pathname;
      return other ? link : null;
    };
    let preview = null;
    let previewed = null;
    let pending = null;
    const hidePreview = () => {
      preview?.remove();
      previewed?.removeAttribute("aria-describedby");
      preview = null;
      previewed = null;
    };
    const showPreview = (link, text) => {
      if (!text || link !== pending || link === previewed) return;
      hidePreview();
      preview = document.createElement("span");
      preview.className = "link-preview";
      preview.id = "link-preview";
      preview.setAttribute("role", "tooltip");
      preview.textContent = text;
      link.setAttribute("aria-describedby", preview.id);
      link.after(preview);
      previewed = link;
    };
    for (const kind of ["mouseover", "focusin"]) {
      document.addEventListener(kind, (event) => {
        const link = internal(event.target);
        if (!link) return;
        pending = link;
        summary(link).then((text) => showPreview(link, text));
      });
    }
    for (const kind of ["mouseout", "focusout"]) {
      document.addEventListener(kind, (event) => {
        const to = event.relatedTarget;
        if (to && (preview?.contains(to) || previewed?.contains(to))) return;
        if (internal(event.target) || event.target.closest?.(".link-preview")) {
          pending = null;
          hidePreview();
        }
      });
    }
    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape") hidePreview();
    });
  }
})();
