---
title: "The khalid command"
order: 15
---

# The khalid command

Everything Khalid does starts from one command, `khalid`, followed by what you want it to do, such as `khalid build`. This page shows how to see every command, and how to make your shell finish the commands for you.

## See every command

`khalid --help` lists every command with one line on what it does. `khalid <command> --help` explains one command and each of its options, such as `khalid lint --help`.

```console
$ khalid --help
An open-source blog engine with SEO defaults built in, by SerpDad.

Usage: khalid [OPTIONS] [COMMAND]
...
```

Every command also takes `--json`, which prints the result as JSON for scripts and AI agents instead of sentences.

## Finish commands with Tab

Shell completion means your shell finishes a command or an option when you press Tab, and lists the choices when there are several. Type `khalid li`, press Tab, and the shell writes `khalid lint`.

`khalid completions <shell>` prints the script that teaches your shell Khalid's commands. It works for bash, zsh, fish, PowerShell and elvish. The script comes from the binary you run, so after an update, run the line for your shell again to pick up new commands.

```console
$ khalid completions zsh
#compdef khalid
...
```

Run the line for your shell once, then open a new terminal.

**bash**, on Linux, or on a Mac with the bash-completion package:

```text
mkdir -p ~/.local/share/bash-completion/completions
khalid completions bash > ~/.local/share/bash-completion/completions/khalid
```

**zsh**, the shell on a Mac. The first line writes the script; the second tells zsh where to find it, and goes in `~/.zshrc` before any line that runs `compinit`:

```text
mkdir -p ~/.zfunc && khalid completions zsh > ~/.zfunc/_khalid
echo 'fpath=(~/.zfunc $fpath); autoload -Uz compinit && compinit' >> ~/.zshrc
```

**fish**:

```text
khalid completions fish > ~/.config/fish/completions/khalid.fish
```

**PowerShell**, on Windows. This adds one line to your PowerShell profile, the script PowerShell runs each time it starts:

```text
Add-Content $PROFILE 'khalid completions powershell | Out-String | Invoke-Expression'
```

**elvish**. This adds one line to `~/.config/elvish/rc.elv`:

```text
echo 'eval (khalid completions elvish | slurp)' >> ~/.config/elvish/rc.elv
```

## Read the manual pages

A manual page is the help for one command in the form the `man` command shows, on Linux and on a Mac. Khalid has one for itself and one for each command, such as `khalid-init.1` for `khalid init`.

Each release has them in `khalid-<version>-man.tar.gz`, beside the binaries. Unpack it into a folder `man` reads, then open a page:

```text
mkdir -p ~/.local/share/man/man1
tar -xzf khalid-<version>-man.tar.gz -C ~/.local/share/man/man1
man khalid-init
```

The Docker image holds them in `/usr/share/man/man1`. To make them from the source code, run `just man`, which writes them to `target/man`.
