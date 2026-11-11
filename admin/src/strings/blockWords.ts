// The words for the blocks the editor learned with backlog row 16, beside
// en.ts, which is past its size limit. en.ts spreads each part into its own.

/** The name each new block card shows. */
const names = {
  book: "Book",
  chart: "Chart",
  course: "Course",
  dataset: "Dataset",
  event: "Event",
  location: "Location",
  "privacy-facts": "Privacy facts",
  table: "Folding table",
  takeaways: "Key takeaways",
};

/** What each new block is for, in the slash menu under its name. */
const hints = {
  book: "A book with its author, so Google knows which book you mean",
  chart: "A bar, column or line chart drawn from numbers you type",
  course: "A course you teach and what it covers, for Google",
  dataset: "Data you publish, with its licence and download, for Google",
  event: "An event with its date and place, for Google",
  location: "One of your business's places, as set in the site settings",
  "privacy-facts": "What this blog collects about readers, written from your settings",
  table: "A long table that shows its first rows and folds the rest away",
  takeaways: "The post's main points as a short list",
};

/** The labels of the details only the new blocks ask for, by attribute name. */
const fields = {
  download: "Download address of the data file",
  end: "End date and time, written the same way as the start",
  fold: "Rows shown before the rest fold away, 8 when left empty",
  height: "Height in pixels, from 120 to 2000",
  isbn: "ISBN, the book's number, such as 978-0-00-000000-2",
  keywords: "Keywords, split by commas",
  license: "Licence address, the page that says how others may use it",
  more: "Words on the line that opens the rest",
  online: "Online address, when it is held online instead of at a venue",
  performer: "Who speaks or performs",
  start: "Start date and time, such as 2026-11-12T18:00+00:00",
  status: "Status",
  thumbnail: "Picture search results show, when not the one shown before it plays",
  unit: "Unit of the numbers, such as visits",
  uploaded: "Date it was first published, such as 2024-05-16",
  venue: "Venue name",
};

/** Labels that say more than a detail's usual label in one block. */
const blockFields: Record<string, Record<string, string>> = {
  book: { name: "Book title", author: "Who wrote it", href: "Link to the book" },
  chart: {
    type: "Kind of chart",
    title: "One line that says what the numbers show",
    src: "CSV file beside the post, such as traffic.csv, instead of numbers below",
    body: "Numbers: a first line such as Month, Visits, then one line per row",
  },
  course: { href: "Link to the course", body: "What the course teaches" },
  dataset: { body: "What the data holds and how it was gathered" },
  event: {
    name: "What the event is called",
    price: "Ticket price, such as 15.00, or 0 when free",
    body: "What happens at the event",
  },
  location: { name: "Location name, as written in the site settings" },
  table: { body: "One markdown table: a header row, a line of dashes, then one row per line" },
  takeaways: {
    title: "Title over the list, Key takeaways when left empty",
    body: "One list, a line starting with - for each point",
  },
};

/** The names of the new one-word choices. */
const choices = {
  column: "Columns",
  bar: "Bars",
  line: "Line",
  scheduled: "Going ahead",
  cancelled: "Cancelled",
  postponed: "Postponed, no new date yet",
  rescheduled: "Moved to a new date",
  "moved-online": "Moved online",
};

export const blockWords = { names, hints, fields, blockFields, choices };
