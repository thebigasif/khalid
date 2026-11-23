Fixed
Links are read with url 2.5.9, which fixes a crash: a link wrapped in thousands of `blob:` prefixes could overflow the stack and stop Khalid (RUSTSEC-2026-0592).
