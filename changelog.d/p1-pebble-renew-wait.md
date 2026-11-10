Fixed
- The check that Khalid renews its HTTPS certificate while it runs no longer fails at random: it waits for the new certificate instead of guessing how long it takes, and compares when each certificate was issued.
