// Ghost's own Content API client against a running Khalid (U26, KTD13).
// It browses and reads posts, pages, tags and authors, reads the settings,
// and checks the filters, pagination and errors a Ghost front end relies
// on. Each check prints one line; any failure ends with exit code 1.
//
// Usage: node check.mjs <blog address> <content key>
// The key comes from scripts/ghost-client.sh and is never printed.

import GhostContentAPI from "@tryghost/content-api";

const [url, key] = process.argv.slice(2);
if (!url || !key) {
  console.error("Usage: node check.mjs <blog address> <content key>");
  process.exit(2);
}

const api = new GhostContentAPI({ url, key, version: "v5.0" });
let failed = 0;

/** Runs one named check and prints whether it held. */
async function check(name, run) {
  try {
    await run();
    console.log(`ok   ${name}`);
  } catch (error) {
    failed += 1;
    console.log(`FAIL ${name}: ${error.message}`);
  }
}

/** Stops the check with `message` unless `holds`. */
function expect(holds, message) {
  if (!holds) throw new Error(message);
}

/** The status and Ghost error type of a refused request. */
async function refusal(request) {
  try {
    await request();
  } catch (error) {
    const response = error.response ?? {};
    return { status: response.status, type: response.data?.errors?.[0]?.type };
  }
  throw new Error("The request was answered, and it should have been refused.");
}

const slugsOf = (entries) => entries.map((entry) => entry.slug).sort();
const hasTag = (post, slugs) => (post.tags ?? []).some((tag) => slugs.includes(tag.slug));

const all = await api.posts.browse({ limit: "all", include: "tags,authors" });

await check("posts: browse all, with tags and authors", async () => {
  expect(all.length >= 2, `only ${all.length} posts came back`);
  for (const post of all) {
    for (const field of ["id", "slug", "title", "url", "published_at", "excerpt"]) {
      expect(post[field], `${post.slug} has no ${field}`);
    }
    // A members-only post comes with empty html, as Ghost sends it to a key
    // with no member.
    const forMembers = post.visibility === "members" && post.access === false;
    expect(forMembers ? post.html === "" : post.html, `${post.slug} has no html`);
    expect(Number.isInteger(post.reading_time), `${post.slug} has no reading_time`);
    expect(Array.isArray(post.tags) && Array.isArray(post.authors), `${post.slug} lacks tags or authors`);
    expect(post.primary_author?.slug === post.authors[0]?.slug, `${post.slug}'s primary_author is not its first author`);
  }
  expect(all.meta.pagination.total === all.length, "meta.pagination.total differs from the posts returned");
});

await check("posts: limit and page", async () => {
  const second = await api.posts.browse({ limit: 1, page: 2 });
  const { pagination } = second.meta;
  expect(second.length === 1, `page 2 held ${second.length} posts`);
  expect(pagination.page === 2 && pagination.limit === 1, "the pagination does not name page 2 of 1");
  expect(pagination.prev === 1, "page 2 does not point back to page 1");
  expect(pagination.total === all.length && pagination.pages === all.length, "the totals are wrong");
  expect(second[0].slug === all[1].slug, "page 2 is not the second post");
});

await check("posts: read by slug and by id", async () => {
  const first = all[0];
  const bySlug = await api.posts.read({ slug: first.slug }, { include: "tags,authors" });
  const byId = await api.posts.read({ id: first.id });
  expect(bySlug.id === first.id && byId.slug === first.slug, "the two reads give different posts");
  expect(bySlug.title === first.title, "the title differs from the browse");
});

await check("posts: formats=plaintext and fields", async () => {
  const [post] = await api.posts.browse({ limit: 1, formats: "plaintext", fields: "slug,plaintext" });
  expect(typeof post.plaintext === "string" && post.plaintext.length > 0, "no plaintext came back");
  expect(Object.keys(post).sort().join(",") === "plaintext,slug", `fields kept ${Object.keys(post)}`);
});

const tags = await api.tags.browse({ limit: "all" });
const [a, b] = slugsOf(tags);

await check("filters: tag and featured", async () => {
  const want = all.filter((post) => hasTag(post, [a]) && post.featured);
  const got = await api.posts.browse({ limit: "all", filter: `tag:${a}+featured:true` });
  expect(slugsOf(got).join() === slugsOf(want).join(), `got ${slugsOf(got)}, Ghost gives ${slugsOf(want)}`);
});

await check("filters: a tag list", async () => {
  const want = all.filter((post) => hasTag(post, [a, b]));
  const got = await api.posts.browse({ limit: "all", filter: `tag:[${a},${b}]` });
  expect(want.length > 0, "no post has either tag, so the check proves nothing");
  expect(slugsOf(got).join() === slugsOf(want).join(), `got ${slugsOf(got)}, Ghost gives ${slugsOf(want)}`);
});

await check("filters: not a primary tag", async () => {
  const primary = all.find((post) => post.primary_tag)?.primary_tag.slug;
  expect(primary, "no post has a primary tag");
  const got = await api.posts.browse({ limit: "all", filter: `primary_tag.slug:-${primary}`, include: "tags" });
  expect(got.length < all.length, "nothing was left out");
  expect(got.every((post) => post.primary_tag?.slug !== primary), "a post with that primary tag came back");
});

await check("filters: a filter outside the common part answers Ghost's 400", async () => {
  const { status, type } = await refusal(() => api.posts.browse({ filter: "title:~'speed'" }));
  expect(status === 400 && type === "BadRequestError", `got ${status} ${type}`);
});

await check("pages: browse and read", async () => {
  const pages = await api.pages.browse({ limit: "all" });
  expect(pages.length >= 1, "no pages came back");
  const page = await api.pages.read({ slug: pages[0].slug });
  expect(page.id === pages[0].id && page.html, "the page read differs or has no html");
});

await check("tags: browse and read", async () => {
  expect(tags.length >= 2, `only ${tags.length} tags came back`);
  const tag = await api.tags.read({ slug: a });
  expect(tag.slug === a && tag.name, "the tag read is wrong");
});

await check("authors: browse and read", async () => {
  const authors = await api.authors.browse({ limit: "all" });
  expect(authors.length >= 1, "no authors came back");
  const author = await api.authors.read({ slug: authors[0].slug });
  expect(author.id === authors[0].id && author.name, "the author read is wrong");
  expect("profile_image" in author, "the author has no profile_image field");
});

await check("settings", async () => {
  const settings = await api.settings.browse();
  expect(settings.title, "the settings have no title");
  expect(settings.url?.startsWith("http"), "the settings have no url");
  expect(Array.isArray(settings.navigation), "the settings have no navigation");
});

await check("a wrong key answers Ghost's 401", async () => {
  const wrong = new GhostContentAPI({ url, key: "0".repeat(26), version: "v5.0" });
  const { status, type } = await refusal(() => wrong.posts.browse());
  expect(status === 401 && type === "UnauthorizedError", `got ${status} ${type}`);
});

if (failed > 0) {
  console.log(`${failed} check(s) failed. Each FAIL line names the call and what came back.`);
  process.exit(1);
}
console.log("Ghost's client read every resource from Khalid.");
