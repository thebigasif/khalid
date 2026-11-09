// Run by scripts/starters-check.sh: checks a starter kit's built blog,
// served at the origin given, on its home page and on the first post the
// home page links to. Each page needs a title and a meta description in
// its head, and JSON-LD schema that parses. Exits 1 when any is missing.
//
// Usage: node starters-check.mjs http://127.0.0.1:14321

const origin = process.argv[2];
let failed = false;

/** Reads `path`, or says why it could not. */
async function read(path) {
  const answer = await fetch(new URL(path, origin));
  if (!answer.ok) {
    throw new Error(`${path} answered ${answer.status}, so it cannot be checked.`);
  }
  return answer.text();
}

/** Checks the head and schema of the page at `path`, and returns its title. */
function check(path, html) {
  const head = html.split("</head>")[0];
  const title = head.match(/<title>([^<]+)<\/title>/)?.[1]?.trim();
  const description = head.match(/<meta name="description" content="([^"]+)"/)?.[1]?.trim();
  const scripts = [...html.matchAll(/<script type="application\/ld\+json">(.*?)<\/script>/gs)];
  const problems = [];
  if (!title) problems.push("no title in its head");
  if (!description) problems.push("no meta description in its head");
  if (scripts.length === 0) problems.push("no JSON-LD schema");
  for (const [, json] of scripts) {
    try {
      const schema = JSON.parse(json);
      if (schema["@context"] !== "https://schema.org") problems.push("schema without the schema.org context");
    } catch {
      problems.push("JSON-LD schema that does not parse");
    }
  }
  if (problems.length > 0) {
    console.log(`${path} has ${problems.join(", ")}. Search engines need all three.`);
    failed = true;
  } else {
    console.log(`${path}: "${title}", a description and ${scripts.length} schema block(s).`);
  }
  return title;
}

try {
  const home = await read("/blog/");
  const homeTitle = check("/blog/", home);
  const link = home.match(/href="(\/blog\/[^"#?]+\/)"/)?.[1];
  if (!link) {
    console.log("/blog/ links to no post, so no post page can be checked.");
    failed = true;
  } else if (check(link, await read(link)) === homeTitle) {
    console.log(`${link} has the home page's title, not its own.`);
    failed = true;
  }
} catch (error) {
  console.log(error instanceof Error ? error.message : String(error));
  failed = true;
}
process.exit(failed ? 1 : 0);
