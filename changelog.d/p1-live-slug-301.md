Added
- A live post's web address can change in the admin and through the Admin API. The old address goes into the post's `aliases:`, so it sends readers and search engines to the new one with a 301 (a permanent redirect), on `khalid serve` and in a static build. Before, the change was refused (KH-SEO-8).
