Added
- A site can give its pages their words in any language: a file such as `theme/strings/fr.toml` holds French words for the text around posts, dates and the "page not found" page, and a key it leaves out keeps the built-in or English word. The same file can change some English or German words. A key no template reads stops the build with a message that names the file and the key.
