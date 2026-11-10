Added
- The one-line install script now also installs the manual pages and sets up Tab completion for bash, zsh or fish, in your home folder or, as root, for everyone under `/usr/local/share`. It says what it did with each, skips one it cannot do without stopping the install, and `--no-man` or `--no-completions` leaves one out. `docs/deploy.md` lists where each file goes.
