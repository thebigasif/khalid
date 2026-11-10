Added
- `khalid completions <shell>` prints a script that makes bash, zsh, fish, PowerShell or elvish finish Khalid's commands and options when you press Tab. `docs/cli.md` gives the one line that installs it for each shell.
- Each release now has a manual page for khalid and for every command, such as `man khalid-init`, in `khalid-<version>-man.tar.gz` beside the binaries, and the Docker image holds them in `/usr/share/man/man1`. `just man` makes them from the source code.
