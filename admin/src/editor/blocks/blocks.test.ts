import { Editor, type JSONContent } from "@tiptap/core";
import { describe, expect, it } from "vitest";
import { extensions, fitsRichText, throughRichText } from "../richText";
import { nodeName } from "./directiveNode";
import { BLOCK_SPECS } from "./specs";

// One sample per plan 4.6 block beyond the starter kit, written the way
// `khalid-content` reads it. Each must come back byte for byte, so opening
// and saving a post with any block changes nothing in its file.
const BLOCKS: Record<string, string> = {
  audio: '::audio{src="/media/episode-12.mp3" title="Episode 12: internal links"}',
  book: '::book{name="Clear Writing" author="Maren Okafor" isbn="978-0-00-000000-2" href="https://www.example.com/clear-writing/"}',
  bookmark:
    '::bookmark{href="https://web.dev/articles/vitals" title="Web Vitals" description="Quality signals for a good experience." image="https://web.dev/vitals.png"}',
  business:
    '::business{name="Field Notes Studio" type="ProfessionalService" street="12 Mill Lane" locality="Leeds" postcode="LS1 4AB" country="GB" phone="+44 113 496 0000" email="hello@example.com" href="https://www.example.com"}',
  button: '::button{href="/pricing/" label="See prices"}',
  callout:
    ":::callout{kind=tip}\nLink from new posts to old ones, then go back the other way.\n:::",
  chart:
    ':::chart{type=line title="Visits by month" unit=visits}\nMonth, Visits\nJan, 120\nFeb, 140\n:::',
  course:
    ':::course{name="Links that rank" href="/courses/links/"}\nSix short lessons on internal links.\n:::',
  dataset:
    ':::dataset{name="Link audit 2026" keywords="links, audit" license="https://creativecommons.org/licenses/by/4.0/" download="/media/link-audit.csv"}\nEvery link on 400 blogs, gathered in May.\n:::',
  event:
    ':::event{name="Writing night" start="2026-11-12T18:00+00:00" end="2026-11-12T20:00+00:00" online="https://www.example.com/live/" status=scheduled}\nAn evening of writing together.\n:::',
  columns:
    '::::columns\n:::column{title="Before"}\nOne link at the end.\n:::\n:::column{title="After"}\nThree links in the text.\n:::\n::::',
  cta: "::cta",
  embed:
    '::embed{src="https://www.youtube.com/watch?v=dQw4w9WgXcQ" title="How search works" height=480 uploaded=2024-05-16}',
  faq: '::::faq\n:::question{text="Does it work offline?"}\nYes, every page works with scripts off.\n:::\n::::',
  figure:
    ':::figure{src="shelf.jpg" alt="A bakery shelf of sourdough loaves"}\nThe morning bake.\n:::',
  file: '::file{src="/media/link-audit.csv" title="The link audit sheet"}',
  form: ':::form{id="contact" button="Send"}\n- email reply_to: Your email address\n:::',
  gallery:
    ":::gallery\n![The editor](/media/editor.png)\n![The posts table](/media/posts.png)\n:::",
  gated:
    ':::gated{note="The rest is for subscribers." label="Email" button="Read"}\nThe full list.\n:::',
  header: ':::header{image="/media/cover.jpg" alt="A desk by a window"}\n# Writing that ranks\n:::',
  howto:
    '::::howto{name="Add a link" time="2:00"}\n:::step{name="Open the post"}\nFind it in Posts.\n:::\n::::',
  html: ':::html{show="visitors"}\n<p>Sign in to comment.</p>\n:::',
  location: '::location{name="Leeds office"}',
  maths: ':::maths{label="Compound growth"}\nA = P(1 + r)^t\n:::',
  newsletter: '::newsletter{label="Your email address" button="Get the weekly links"}',
  "privacy-facts": "::privacy-facts",
  product:
    ':::product{name="Kit" image="/media/kit.jpg" price="49.00" currency="USD" rating="4.5" average="4.6" ratings="52" href="/kit/" button="Buy the kit"}\nEverything to start.\n:::',
  proscons: ':::proscons{pros="What works" cons="What does not"}\n+ Fast\n- No team settings\n:::',
  pullquote: ':::pullquote{by="Maren Okafor"}\nA tool nobody runs finds nothing.\n:::',
  recipe:
    '::::recipe{name="Toast" servings="1"}\n:::ingredients\n- Bread\n:::\n:::method\n1. Toast it\n:::\n::::',
  review:
    ':::review{item="A book" type="Book" rating="4" average="4.2" ratings="310" author="Asif"}\nClear and short.\n:::',
  snippet: '::snippet{slug="affiliate-disclosure"}',
  table:
    ':::table{fold=2 more="Show every plan"}\n| Plan | Price |\n| --- | --- |\n| Solo | 9 |\n| Team | 29 |\n| Agency | 99 |\n:::',
  takeaways:
    ':::takeaways{title="In short"}\n- Link new posts to old ones.\n- Check links each month.\n:::',
  toc: "::toc",
  toggle: ':::toggle{summary="Why does this matter?"}\nSearch engines read it.\n:::',
  video:
    '::video{src="/media/site-tour.mp4" title="A tour of the site" poster="/media/site-tour.jpg" description="Where posts, pages and settings live." duration="2:05" thumbnail="/media/site-tour-card.jpg" uploaded=2024-05-16}',
  markdownTable: "| Block | Form |\n| --- | --- |\n| toc | leaf |",
  footnote: "Links help readers.[^1]\n\n[^1]: And search engines.",
  longFootnote: "One.[^long]\n\n[^long]: A note on\n    two lines.",
};

/** The editor's document for `markdown`, as Tiptap JSON. */
function read(markdown: string) {
  const editor = new Editor({ extensions, content: markdown, contentType: "markdown" });
  const json = editor.getJSON();
  editor.destroy();
  return json;
}

/** The markdown the editor writes for the document `json`. */
function write(json: JSONContent): string {
  const editor = new Editor({ extensions, content: json });
  const markdown = editor.getMarkdown().trimEnd();
  editor.destroy();
  return markdown;
}

describe("the editor's blocks", () => {
  it("has a sample for every directive block", () => {
    for (const spec of BLOCK_SPECS) {
      expect(BLOCKS[spec.name]).toBeDefined();
    }
  });

  for (const [block, markdown] of Object.entries(BLOCKS)) {
    it(`gives back a ${block} unchanged`, () => {
      expect(throughRichText(markdown).trimEnd()).toBe(markdown);
      expect(fitsRichText(markdown)).toBe(true);
    });
  }

  for (const spec of BLOCK_SPECS) {
    it(`reads a ${spec.name} as its own node`, () => {
      const first = read(BLOCKS[spec.name] ?? "").content?.[0];
      expect(first?.type).toBe(nodeName(spec));
    });
  }

  it("reads a block's attributes into the node", () => {
    const button = read(BLOCKS.button ?? "").content?.[0];
    expect(button?.attrs).toMatchObject({ href: "/pricing/", label: "See prices" });
  });

  it("reads the columns inside a columns block as its text", () => {
    const columns = read(BLOCKS.columns ?? "").content?.[0];
    expect(columns?.attrs?.fence).toBe("::::");
    expect(columns?.attrs?.body).toContain(':::column{title="After"}');
  });

  it("writes new pros and cons, a pull quote and a call to action from their details", () => {
    const doc = {
      type: "doc",
      content: [
        { type: "pullquoteBlock", attrs: { by: "Maren", body: "Short wins." } },
        { type: "prosconsBlock", attrs: { pros: null, cons: null, body: "+ Fast\n- Dear" } },
        { type: "ctaBlock", attrs: {} },
      ],
    };
    expect(write(doc)).toBe(
      ":::pullquote{by=Maren}\nShort wins.\n:::\n\n:::proscons\n+ Fast\n- Dear\n:::\n\n::cta",
    );
  });

  it("names a block with a hyphen as one word, as Tiptap needs", () => {
    expect(read(BLOCKS["privacy-facts"] ?? "").content?.[0]?.type).toBe("privacyFactsBlock");
  });

  it("reads a folding table's markdown table as its text, not as a table of its own", () => {
    const table = read(BLOCKS.table ?? "").content?.[0];
    expect(table?.attrs).toMatchObject({ fold: "2", more: "Show every plan" });
    expect(table?.attrs?.body).toBe(
      "| Plan | Price |\n| --- | --- |\n| Solo | 9 |\n| Team | 29 |\n| Agency | 99 |",
    );
  });

  it("reads a video's thumbnail and upload date, and an embed's height", () => {
    expect(read(BLOCKS.video ?? "").content?.[0]?.attrs).toMatchObject({
      thumbnail: "/media/site-tour-card.jpg",
      uploaded: "2024-05-16",
    });
    expect(read(BLOCKS.embed ?? "").content?.[0]?.attrs).toMatchObject({
      height: "480",
      uploaded: "2024-05-16",
    });
  });

  it("writes a new event and a new chart from their details", () => {
    const event = { name: "Writing night", start: "2026-11-12T18:00+00:00", online: "/live/" };
    const chart = { type: "bar", title: "Visits", body: "Month, Visits\nJan, 3" };
    const doc = {
      type: "doc",
      content: [
        { type: "eventBlock", attrs: { ...event, body: "Come along." } },
        { type: "chartBlock", attrs: chart },
      ],
    };
    const opening = ':::event{name="Writing night" start="2026-11-12T18:00+00:00" online="/live/"}';
    expect(write(doc)).toBe(
      `${opening}\nCome along.\n:::\n\n:::chart{type=bar title=Visits}\nMonth, Visits\nJan, 3\n:::`,
    );
  });

  it("reads a table and a footnote as their own nodes", () => {
    expect(read(BLOCKS.markdownTable ?? "").content?.[0]?.type).toBe("markdownTable");
    const [text, note] = read(BLOCKS.footnote ?? "").content ?? [];
    expect(text?.content?.at(-1)).toMatchObject({ type: "footnoteMark", attrs: { label: "1" } });
    expect(note?.type).toBe("footnoteText");
  });

  it("keeps every block in a post in its place", () => {
    const post = Object.values(BLOCKS).join("\n\nA paragraph between blocks.\n\n");
    expect(throughRichText(post).trimEnd()).toBe(post);
  });

  it("writes a changed block in canonical form", () => {
    const doc = read('::button{label="See prices" href="/pricing/"}');
    const button = doc.content?.[0];
    expect(button?.attrs).toBeDefined();
    if (button?.attrs !== undefined) {
      button.attrs.label = "Compare plans";
    }
    expect(write(doc)).toBe('::button{href="/pricing/" label="Compare plans"}');
  });

  it("makes a new block from its attributes alone", () => {
    const doc = {
      type: "doc",
      content: [{ type: "calloutBlock", attrs: { kind: "warning", body: "Back up first." } }],
    };
    expect(write(doc)).toBe(":::callout{kind=warning}\nBack up first.\n:::");
  });

  it("keeps a post with a directive Khalid does not know out of the rich-text view", () => {
    expect(fitsRichText("::tco")).toBe(false);
    expect(fitsRichText(":::note\nHello\n:::")).toBe(false);
  });

  it("keeps a block with an attribute it does not take out of the rich-text view", () => {
    expect(fitsRichText(":::callout{kind=tip colour=blue}\nFeed it daily.\n:::")).toBe(false);
  });
});
