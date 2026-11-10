Added
- A YouTube, Vimeo or Loom embed can now give the day its host published the video, such as `uploaded="2024-05-16"`. The page's structured data then describes the video as a `VideoObject`, so Google can show it in video results. An embed without that day is left out of the structured data, because Google requires it and Khalid never guesses it.
- A `::video` block can give its own `uploaded` day too, for a video first shown elsewhere. Without one it keeps using the post's date.

Fixed
- A `::video` block on a page with no date no longer writes a `VideoObject` without an upload date, which Google rejects. It still goes in the video sitemap.
