//! The AI crawlers Khalid knows by name, split by what they do with a post:
//! copy it to train a model, or fetch it to quote and link it in an answer
//! (decision D1, 2026-10-28). Each entry names the page where its operator
//! says what the crawler is for.

/// A crawler Khalid knows by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct KnownCrawler {
    /// The user-agent token it obeys in robots.txt, such as `GPTBot`.
    pub token: &'static str,
    /// Who runs it and what it reads pages for, in one plain line.
    pub about: &'static str,
}

const fn crawler(token: &'static str, about: &'static str) -> KnownCrawler {
    KnownCrawler { token, about }
}

/// Crawlers that copy posts to train AI models. Blocking them keeps posts
/// out of future models and costs no search or answer traffic.
/// `[seo.crawlers] ai_training = "block"` blocks every one.
pub const AI_TRAINING_CRAWLERS: [KnownCrawler; 10] = [
    // https://platform.openai.com/docs/bots
    crawler("GPTBot", "OpenAI, to train its models"),
    // https://support.claude.com/en/articles/8896518 (Anthropic's crawlers)
    crawler("ClaudeBot", "Anthropic, to train its models"),
    // The same page: older Anthropic names it still honours.
    crawler(
        "anthropic-ai",
        "Anthropic, an older name for its training crawler",
    ),
    crawler("Claude-Web", "Anthropic, an older name it may still use"),
    // https://developers.google.com/search/docs/crawling-indexing/google-common-crawlers
    crawler(
        "Google-Extended",
        "Google, to train Gemini. Google Search is not affected",
    ),
    // https://support.apple.com/en-us/119829
    crawler(
        "Applebot-Extended",
        "Apple, to train Apple Intelligence. Siri and Spotlight are not affected",
    ),
    // https://commoncrawl.org/ccbot
    crawler(
        "CCBot",
        "Common Crawl, a public copy of the web many models train on",
    ),
    // ByteDance publishes no page; https://darkvisitors.com/agents/bytespider
    crawler("Bytespider", "ByteDance, to train its models"),
    // https://developers.facebook.com/docs/sharing/webmasters/web-crawlers
    crawler("meta-externalagent", "Meta, to train its models"),
    // https://developer.amazon.com/amazonbot says it may train Amazon's models.
    crawler("Amazonbot", "Amazon, for Alexa and to train its models"),
];

/// Crawlers that fetch posts to quote and link them in AI answers. They
/// stay allowed when the training switch is on; block one by name in
/// `[seo.crawlers] block`.
pub const AI_ANSWER_CRAWLERS: [KnownCrawler; 6] = [
    // https://platform.openai.com/docs/bots
    crawler(
        "OAI-SearchBot",
        "ChatGPT search, which links the pages it finds",
    ),
    crawler(
        "ChatGPT-User",
        "ChatGPT, when a person asks it to open a page",
    ),
    // https://support.claude.com/en/articles/8896518
    crawler(
        "Claude-SearchBot",
        "Claude's search, which links the pages it finds",
    ),
    crawler(
        "Claude-User",
        "Claude, when a person asks it to open a page",
    ),
    // https://docs.perplexity.ai/guides/bots
    crawler(
        "PerplexityBot",
        "Perplexity search, which links the pages it finds",
    ),
    crawler(
        "Perplexity-User",
        "Perplexity, when a person asks it to open a page",
    ),
];
