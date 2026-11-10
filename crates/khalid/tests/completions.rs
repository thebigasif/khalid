//! `khalid completions <shell>`: the script that teaches a shell Khalid's
//! commands, for each shell, in both output modes (RULES.md BL-CLI-1,
//! BL-CLI-2).

use std::process::{Command, Output};

use serde_json::Value;

#[cfg(test)]
fn khalid(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn every_shell_gets_a_script_that_names_the_commands() {
    for shell in ["bash", "zsh", "fish", "powershell", "elvish"] {
        let output = khalid(&["completions", shell]);
        assert!(output.status.success(), "{shell}: {output:?}");
        let script = String::from_utf8(output.stdout).unwrap();
        assert!(script.len() > 1000, "{shell}: the script is too short");
        for command in ["init", "serve", "lint", "build", "publish", "completions"] {
            assert!(script.contains(command), "{shell}: no {command}");
        }
        assert!(script.contains("site"), "{shell}: no --site option");
    }
}

#[test]
fn json_holds_the_shell_and_the_script() {
    let output = khalid(&["completions", "zsh", "--json"]);
    assert!(output.status.success(), "{output:?}");
    let body: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["shell"], "zsh");
    assert!(
        body["script"]
            .as_str()
            .unwrap()
            .starts_with("#compdef khalid")
    );
}

#[test]
fn an_unknown_shell_is_a_usage_error() {
    let output = khalid(&["completions", "tcsh"]);
    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(message.contains("tcsh"), "{message}");
}

#[test]
fn completions_work_without_a_site_folder() {
    let empty = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .current_dir(empty.path())
        .args(["completions", "bash"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}
