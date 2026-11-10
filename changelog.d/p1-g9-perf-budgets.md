Changed
- The performance budgets job now checks the admin's initial JavaScript against its 350 KB budget, from the admin CI already builds, and fails when no admin build reaches it. Lint time, requests per second and the public stylesheet were already checked; the budgets page now says how each is measured.
