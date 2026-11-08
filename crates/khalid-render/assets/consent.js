// Consent on a static site (privacy plan U6, KTD3, KTD7, KTD9). A static
// host cannot read a cookie, so this script does what the server does in
// server mode: it reads the reader's choice from the `khalid-consent`
// cookie, runs the owner code they allowed, shows the embeds they allowed,
// and shows the banner until they answer. Without it nothing optional ever
// loads: held scripts are plain text and players sit in inert templates.
//
// The cookie is `2.<rev>.<bits>.<day>.<flags>`. `rev` changes when the
// categories, the vendors or the lifetime change, and asks every reader
// again. `bits` has one bit per optional category, in the order of NAMES.
// `day` is the day of the answer, counted from 1970. `flags` has 1 for
// "Marketing allowed despite Global Privacy Control" and 2 for "answered
// only External media", which keeps the banner showing.
(() => {
  "use strict";
  const me = document.currentScript;
  if (!me) return;
  const set = me.dataset;
  const NAMES = ["preferences", "analytics", "marketing", "external-media"];
  const MARKETING = 4;
  const MEDIA = 8;
  const OVERRIDE = 1;
  const MEDIA_ONLY = 2;
  const days = Number(set.khalidDays) || 180;
  const today = Math.floor(Date.now() / 864e5);
  const gpc = navigator.globalPrivacyControl === true;
  // What the signal turns off: Marketing, or every optional category when
  // the owner reads it as Reject all.
  const locked = set.khalidGpc === "reject-all" ? 15 : MARKETING;
  const bit = (name) => 1 << NAMES.indexOf(name);
  const $ = (selector) => document.querySelectorAll(selector);

  // The reader's answer, or null when there is none, it is for other
  // settings, or it is older than the lifetime.
  const read = () => {
    const pair = document.cookie
      .split("; ")
      .find((cookie) => cookie.startsWith("khalid-consent="));
    const parts = pair ? pair.slice(15).split(".") : [];
    const day = Number(parts[3]);
    const fresh = day <= today && today < day + days;
    if (parts.length !== 5 || parts[0] !== "2" || parts[1] !== set.khalidRev || !fresh) return null;
    return { bits: Number(parts[2]) | 0, flags: Number(parts[4]) | 0 };
  };

  // Whether `answer` lets code of category `name` run. Under Global
  // Privacy Control, Marketing runs only after the reader turned it on
  // themselves on the choices page.
  const allows = (answer, name) => {
    const wanted = bit(name);
    if (!answer || !(answer.bits & wanted)) return false;
    return !(wanted & locked && gpc && !(answer.flags & OVERRIDE));
  };

  // Runs the held code and shows the players `answer` allows. A new script
  // element is made for each held one, since a browser never runs a
  // script whose type it once read as text.
  const release = (answer) => {
    for (const held of $("script[data-khalid-consent]")) {
      if (!allows(answer, held.dataset.khalidConsent)) continue;
      const code = document.createElement("script");
      for (const { name, value } of held.attributes) {
        if (name === "type" || name === "data-khalid-consent") continue;
        code.setAttribute(name === "data-type" ? "type" : name, value);
      }
      code.text = held.text;
      held.replaceWith(code);
    }
    for (const held of $("template[data-khalid-consent]")) {
      if (allows(answer, held.dataset.khalidConsent)) held.replaceWith(held.content);
    }
    if (allows(answer, "external-media")) {
      for (const card of $(".embed .embed-card")) card.remove();
      for (const player of $(".embed template")) player.replaceWith(player.content);
    }
  };

  const write = (bits, flags) => {
    const secure = location.protocol === "https:" ? "; Secure" : "";
    document.cookie = `khalid-consent=2.${set.khalidRev}.${bits}.${today}.${flags}; Path=${set.khalidPath || "/"}; Max-Age=${days * 86400}; SameSite=Lax${secure}`;
    const answer = read();
    release(answer);
    return answer;
  };

  const shown = (selector, show) => {
    for (const element of $(selector)) element.hidden = !show;
  };

  // Puts each inert template for which `wanted` holds in its place.
  const unwrap = (wanted) => {
    for (const template of $("template")) if (wanted(template)) template.replaceWith(template.content);
  };

  // The line that says a choice was saved: a template on most pages, and
  // a status line the choices page fills in once the reader saves.
  const saved = (answer, pressed) => {
    if (!answer || answer.flags & MEDIA_ONLY) return;
    unwrap((template) => "khalidSaved" in template.dataset);
    if (!pressed) return;
    for (const status of $("p[data-khalid-saved]")) status.textContent = status.dataset.khalidSaved;
  };

  // What each button of the banner, the choices page and an embed card
  // saves.
  const answers = {
    "accept-all": () => [gpc ? 15 & ~locked : 15, 0],
    "reject-all": () => [0, 0],
    "allow-external-media": (answer) =>
      answer ? [answer.bits | MEDIA, answer.flags] : [MEDIA, MEDIA_ONLY],
    save: () => {
      let bits = 0;
      for (const box of $("[data-khalid-choices] input:checked")) {
        if (NAMES.includes(box.name)) bits |= bit(box.name);
      }
      return [bits, gpc && bits & locked ? OVERRIDE : 0];
    },
  };

  // A static build locks every form, so the buttons are plain buttons. A
  // button inside a form, as an embed card has, is answered here too, and
  // its form is never sent.
  document.addEventListener("click", (event) => {
    const button = event.target.closest?.("button[value]");
    if (!button || !answers[button.value]) return;
    event.preventDefault();
    const answer = write(...answers[button.value](read()));
    shown("[data-khalid-banner]", answer && answer.flags & MEDIA_ONLY);
    saved(answer, true);
  });

  const answer = read();
  release(answer);
  saved(answer, false);
  // The choices page asks everything itself, so it never shows the banner.
  const choosing = $("[data-khalid-choices]").length > 0;
  shown("[data-khalid-banner]", !choosing && (!answer || answer.flags & MEDIA_ONLY));
  // The note that the signal is honoured waits in a template, as the
  // server keeps it.
  if (gpc) unwrap((template) => template.content.querySelector(".consent-gpc"));
  shown("[data-khalid-choices]", true);
  for (const name of NAMES) {
    for (const box of $(`[data-khalid-choices] [name="${name}"]`)) box.checked = allows(answer, name);
  }
})();
