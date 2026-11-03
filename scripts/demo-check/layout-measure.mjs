// The measured layout checks that run inside a page, for layout.mjs.
// Ported from the serpdad.com blog tests. Each function runs in the
// browser, so it uses nothing from outside its own body.

// The walk down the page as a reader scrolls, as a script to run in it:
// a step of most of a window at a time to the very end, so lazy pictures
// load, measuring what moves with the window at each stop, then back to
// the top. One script for the whole walk keeps it to one call per page.
export function walkDownScript() {
  return `(async () => {
    const measure = ${measure};
    const moves = [...document.body.querySelectorAll("*")].some((el) => ["fixed", "sticky"].includes(getComputedStyle(el).position));
    const step = Math.max(200, Math.floor(window.innerHeight * 0.7));
    const end = Math.max(0, document.documentElement.scrollHeight - window.innerHeight);
    const found = [];
    for (let y = Math.min(step, end); end > 0; y = Math.min(y + step, end)) {
      window.scrollTo(0, y);
      await new Promise((done) => requestAnimationFrame(() => setTimeout(done, 30)));
      if (moves) found.push(...measure("moving"));
      if (y >= end) break;
    }
    window.scrollTo(0, 0);
    return found;
  })()`;
}

// Every layout problem a reader can see at the current scroll position.
// `stage` is "page" for the whole page, measured at the top, or "moving"
// for the parts that move with the window, fixed or sticky, against what
// is under them at this scroll position.
// Each problem is { check, a, b, detail }, where a and b name elements.
export function measure(stage) {
  const problems = [];
  const sx = window.scrollX;
  const sy = window.scrollY;
  const vw = window.innerWidth;
  const styles = new Map();
  const style = (el) => {
    if (!styles.has(el)) styles.set(el, getComputedStyle(el));
    return styles.get(el);
  };
  const name = (el) => {
    const id = el.id ? `#${el.id}` : "";
    const cls = typeof el.className === "string" && el.className.trim() ? `.${el.className.trim().split(/\s+/).slice(0, 2).join(".")}` : "";
    const text = (el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40);
    const src = el.tagName === "IMG" ? ` ${(el.currentSrc || el.getAttribute("src") || "").split("/").pop()}` : "";
    return `<${el.tagName.toLowerCase()}${id}${cls}>${src}${text ? ` "${text}"` : ""}`;
  };
  // checkVisibility also rules out content inside a closed <details> and
  // under an ancestor with opacity 0.
  const shown = (el) => (el.checkVisibility ? el.checkVisibility({ opacityProperty: true, visibilityProperty: true }) : style(el).visibility !== "hidden");
  // The part that moves with the window an element is in, fixed or
  // sticky, or null.
  const movingCache = new Map();
  const moving = (el) => {
    if (!el || el === document.documentElement) return null;
    if (!movingCache.has(el)) movingCache.set(el, ["fixed", "sticky"].includes(style(el).position) ? el : moving(el.parentElement));
    return movingCache.get(el);
  };
  // Boxes are in page coordinates: { l, t, r, b }.
  const box = (rect) => ({ l: rect.left + sx, t: rect.top + sy, r: rect.right + sx, b: rect.bottom + sy });
  const meet = (a, b) => ({ l: Math.max(a.l, b.l), t: Math.max(a.t, b.t), r: Math.min(a.r, b.r), b: Math.min(a.b, b.b) });
  const everywhere = { l: -1e9, t: -1e9, r: 1e9, b: 1e9 };
  // The part of the page an element's content can show in: its ancestors
  // that clip, such as a scrolling table or a carousel, cut it.
  const clips = new Map();
  const clipOf = (el) => {
    if (!el || el === document.documentElement || el === document.body) return everywhere;
    if (!clips.has(el)) {
      const s = style(el);
      let area = s.position === "fixed" ? everywhere : clipOf(el.parentElement);
      if (s.overflowX !== "visible" || s.overflowY !== "visible" || s.clipPath !== "none" || s.clip !== "auto") area = meet(area, box(el.getBoundingClientRect()));
      clips.set(el, area);
    }
    return clips.get(el);
  };
  const rtl = (el) => style(el).direction === "rtl";

  // 1. The page never scrolls sideways.
  if (stage === "page") {
    const wider = document.documentElement.scrollWidth - vw;
    if (wider > 0) {
      const out = [...document.body.querySelectorAll("*")].filter((el) => {
        const r = el.getBoundingClientRect();
        return r.width > 0 && r.right + sx > vw + 1 && clipOf(el.parentElement).r > vw + 1 && style(el).position !== "fixed";
      });
      let named = out.filter((el) => !out.includes(el.parentElement));
      // What sticks out may be a ::before or ::after, which is no element:
      // then the innermost elements that grew wider than the window.
      if (named.length === 0) {
        const wide = [...document.body.querySelectorAll("*")].filter((el) => el.scrollWidth > vw + 1);
        named = wide.filter((el) => !wide.some((inner) => inner !== el && el.contains(inner)));
      }
      problems.push({ check: "sideways-scroll", a: named.slice(0, 3).map(name).join(", ") || "<html>", b: "", detail: `the page is ${wider} px wider than the window` });
    }
  }

  // The things a reader sees: text by its glyphs, line by line, pictures
  // by their box, and list markers. A text line's box holds the space a
  // font leaves above and below its letters, so a tenth of its height is
  // taken off each end, and lines that only touch are not counted.
  // A page with nothing fixed or sticky in view has nothing moving to check.
  if (stage === "moving") {
    const inView = (el) => {
      const r = el.getBoundingClientRect();
      return r.bottom > 0 && r.top < window.innerHeight && r.width > 1 && r.height > 1;
    };
    const movers = [...document.body.querySelectorAll("*")].filter((el) => ["fixed", "sticky"].includes(style(el).position) && shown(el) && inView(el));
    if (movers.length === 0) return problems;
  }
  const items = [];
  const add = (el, kind, rect) => {
    // Scrolled, only what is in the window can be under a moving part.
    const area = meet(meet(box(rect), clipOf(kind === "text" ? el : el.parentElement)), stage === "moving" ? { l: -1e9, t: sy, r: 1e9, b: sy + window.innerHeight } : everywhere);
    if (area.r - area.l > 1 && area.b - area.t > 1) items.push({ el, kind, area, moving: moving(el) });
  };
  const skip = "script, style, noscript, template, svg, math, select, textarea, [aria-hidden='true'] *";
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT, {
    acceptNode: (n) => ((n.textContent ?? "").trim() ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT),
  });
  const range = document.createRange();
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const el = node.parentElement;
    if (!el || el.closest(skip) || !shown(el)) continue;
    range.selectNodeContents(node);
    for (const r of range.getClientRects()) {
      if (r.width <= 1 || r.height <= 1) continue;
      const trim = r.height * 0.1;
      add(el, "text", new DOMRect(r.left, r.top + trim, r.width, r.height - 2 * trim));
    }
  }
  for (const el of document.body.querySelectorAll("img, picture > source + img, video, iframe, canvas, svg")) {
    if (el.parentElement?.closest("svg") || !shown(el)) continue;
    const r = el.getBoundingClientRect();
    // Icons sit beside their labels; a picture is larger than a letter.
    if (el.tagName === "svg" && (r.width < 24 || r.height < 24)) continue;
    add(el, "picture", r);
  }

  // A list item's marker: the ::before box a template draws when it is
  // placed on its own, or the browser's ::marker outside the item, whose
  // end is the item's start edge.
  const markers = [];
  for (const li of document.body.querySelectorAll("li")) {
    if (!shown(li) || li.closest(skip)) continue;
    const r = li.getBoundingClientRect();
    if (r.width <= 1 || r.height <= 1) continue;
    const s = style(li);
    const before = getComputedStyle(li, "::before");
    let mark = null;
    if (before.content !== "none" && before.content !== "normal" && before.display !== "none" && (before.position === "absolute" || before.position === "fixed")) {
      let holder = li;
      while (holder.parentElement && style(holder).position === "static" && style(holder).transform === "none") holder = holder.parentElement;
      const h = holder.getBoundingClientRect();
      const hs = style(holder);
      const width = Number.parseFloat(before.width) || 0;
      const height = Number.parseFloat(before.height) || 0;
      const inner = { l: h.left + Number.parseFloat(hs.borderLeftWidth), t: h.top + Number.parseFloat(hs.borderTopWidth), r: h.right - Number.parseFloat(hs.borderRightWidth) };
      const left = before.left !== "auto" ? inner.l + Number.parseFloat(before.left) : before.right !== "auto" ? inner.r - Number.parseFloat(before.right) - width : r.left;
      const top = before.top !== "auto" ? inner.t + Number.parseFloat(before.top) : r.top;
      mark = { name: "::before", rect: new DOMRect(left, top, width, height) };
    } else if (s.display === "list-item" && s.listStylePosition === "outside" && (s.listStyleType !== "none" || s.listStyleImage !== "none")) {
      const m = getComputedStyle(li, "::marker");
      const width = Number.parseFloat(m.width) || 0;
      const lineHeight = Number.parseFloat(s.lineHeight) || Number.parseFloat(s.fontSize) * 1.2;
      // The marker's own width ends with a space before the text.
      const space = Number.parseFloat(s.fontSize) * 0.25;
      if (width > space) mark = { name: "::marker", rect: new DOMRect(rtl(li) ? r.right + space : r.left - width, r.top, width - space, lineHeight) };
    }
    if (!mark || mark.rect.width <= 1 || mark.rect.height <= 1) continue;
    markers.push({ li, mark });
    add(li, "marker", mark.rect);
  }

  // 2. Text never overlaps other text or a picture by more than 4 px, and
  // a marker never covers text. Two parts of one element, or of an element
  // and the one inside it, are one thing and not compared.
  const row = 64;
  const rows = new Map();
  items.forEach((item, at) => {
    for (let y = Math.floor(item.area.t / row); y <= Math.floor(item.area.b / row); y += 1) {
      if (!rows.has(y)) rows.set(y, []);
      rows.get(y).push(at);
    }
  });
  const found = new Map();
  for (const list of rows.values()) {
    for (let i = 0; i < list.length; i += 1) {
      for (let j = i + 1; j < list.length; j += 1) {
        const a = items[list[i]];
        const b = items[list[j]];
        if (a.kind !== "text" && b.kind !== "text") continue;
        if (stage === "moving" && a.moving === b.moving) continue;
        if (a.el === b.el || a.el.contains(b.el) || b.el.contains(a.el)) continue;
        const o = meet(a.area, b.area);
        const w = o.r - o.l;
        const h = o.b - o.t;
        if (w <= 4 || h <= 4) continue;
        // A marker, then a picture, then a moving part is named first, so
        // each pair is named one way only.
        const rank = (x) => ["marker", "picture", "text"].indexOf(x.kind) * 2 + (x.moving ? 0 : 1);
        const pair = [a, b].sort((x, y) => rank(x) - rank(y) || (name(x.el) < name(y.el) ? -1 : 1));
        const id = `${name(pair[0].el)}|${name(pair[1].el)}`;
        const old = found.get(id);
        if (!old || old.w * old.h < w * h) found.set(id, { pair, w, h, o });
      }
    }
  }
  for (const { pair, w, h, o } of found.values()) {
    const [a, b] = pair;
    const check = a.kind === "marker" || b.kind === "marker" ? "marker-over-text" : a.kind === "picture" || b.kind === "picture" ? "text-over-picture" : "text-over-text";
    const what = (item) => (item.kind === "marker" ? `marker of ${name(item.el)}` : name(item.el));
    const at = `${Math.round(w)}x${Math.round(h)} px at ${Math.round(o.l)},${Math.round(o.t)}`;
    if (stage === "moving") {
      // Named by the moving part itself, such as the side contents column,
      // so layout.mjs can list everything it passes over in one line.
      const [mover, under] = a.moving ? [a, b] : [b, a];
      problems.push({ check, a: name(mover.moving), b: what(under), detail: `${at}, scrolled ${Math.round(sy)} px down`, moving: true });
    } else {
      problems.push({ check, a: what(a), b: what(b), detail: at });
    }
  }

  // 5. A table column is never narrower than its longest word, which would
  // break the word mid-way or push it over the next cell. Each word is
  // measured as drawn, each part in its own font: a word that starts in
  // <code> and ends in the cell's text is narrower than in the cell's font.
  if (stage === "page") {
    const drawn = (node, from, to) => {
      const range = document.createRange();
      range.setStart(node, from);
      range.setEnd(node, to);
      return [...range.getClientRects()].reduce((sum, rect) => sum + rect.width, 0);
    };
    for (const cell of document.querySelectorAll("td, th")) {
      if (!shown(cell) || cell.closest(skip)) continue;
      const s = style(cell);
      const room = cell.clientWidth - Number.parseFloat(s.paddingLeft) - Number.parseFloat(s.paddingRight);
      if (room <= 0) continue;
      const words = [];
      let word = { text: "", width: 0 };
      const walker = document.createTreeWalker(cell, NodeFilter.SHOW_TEXT);
      for (let node = walker.nextNode(); node; node = walker.nextNode()) {
        // Text a reader cannot see, such as a label for screen readers in
        // a 1 px box, is no word of the column.
        const area = clipOf(node.parentElement);
        if (!shown(node.parentElement) || area.r - area.l < 2) continue;
        const text = node.textContent ?? "";
        let start = 0;
        for (let i = 0; i <= text.length; i += 1) {
          const gap = i === text.length || /[\s/-]/.test(text[i]);
          if (!gap) continue;
          if (i > start) {
            word.text += text.slice(start, i);
            word.width += drawn(node, start, i);
          }
          if (i < text.length && word.text) {
            words.push(word);
            word = { text: "", width: 0 };
          }
          start = i + 1;
        }
      }
      if (word.text) words.push(word);
      const widths = words.map((w) => w.width);
      const widest = Math.max(0, ...widths);
      if (widest > room + 1) {
        problems.push({ check: "crushed-cell", a: name(cell), b: `the word "${words[widths.indexOf(widest)].text}"`, detail: `the word needs ${Math.round(widest)} px and the column gives ${Math.round(room)} px, at ${Math.round(cell.getBoundingClientRect().left + sx)},${Math.round(cell.getBoundingClientRect().top + sy)}` });
      }
    }
  }

  // 3. A marker never covers the first letter of its own item.
  if (stage === "page") {
    for (const { li, mark } of markers) {
      const first = document.createTreeWalker(li, NodeFilter.SHOW_TEXT, {
        acceptNode: (n) => ((n.textContent ?? "").trim() && n.parentElement && shown(n.parentElement) && !n.parentElement.closest(skip) ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT),
      }).nextNode();
      if (!first) continue;
      const at = first.textContent.search(/\S/);
      range.setStart(first, at);
      range.setEnd(first, at + 1);
      const g = range.getBoundingClientRect();
      const m = mark.rect;
      const across = Math.min(m.right, g.right) - Math.max(m.left, g.left);
      const down = Math.min(m.bottom, g.bottom) - Math.max(m.top, g.top);
      if (across > 0.5 && down > 0.5) {
        problems.push({ check: "marker-over-text", a: `${mark.name} of ${name(li)}`, b: `its first letter "${first.textContent.charAt(at)}"`, detail: `${Math.round(across)} px at ${Math.round(m.left + sx)},${Math.round(m.top + sy)}` });
      }
    }
  }

  // 4. No picture failed to load. One still loading, such as a lazy one
  // far down the page, is not a failure.
  if (stage === "page") {
    for (const img of document.querySelectorAll("img")) {
      if (img.complete && img.naturalWidth === 0 && (img.currentSrc || img.getAttribute("src"))) {
        problems.push({ check: "broken-picture", a: name(img), b: "", detail: `${img.currentSrc || img.getAttribute("src")} did not load` });
      }
    }
  }
  return problems;
}
