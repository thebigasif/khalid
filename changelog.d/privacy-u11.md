Added
- A Proof tab on the admin's Privacy screen: how many readers allowed each consent category, a week to a row, every version of the banner they were shown with its dates, and a CSV download. Khalid keeps only daily totals, with no address and no reader ID. The Admin API serves the same proof at `/api/admin/v1/consent/counts/` and `/api/admin/v1/consent/versions/`.

Changed
- The admin's editor still turns a typed or pasted address into a link when it starts with `http://`, `https://` or `www.`, or is an email address. A bare domain such as `example.com` now stays text. This keeps the admin inside its size budget.
