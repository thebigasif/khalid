//! `scripts/install.sh`: it installs the release file for this machine only
//! after its checksum matches, and refuses everything else with a message
//! that says what, why and what to do (task 5.14, KH-INS-1). The releases
//! come from a folder through `KHALID_RELEASES`, so no test touches the
//! network (RULES.md BL-TST-3). Each run gets its own home folder, so the
//! manual pages and the shell completion land where the test can see them.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

/// The release name every test installs.
const VERSION: &str = "v9.8.7";

/// What the stand-in binary prints, so a test can tell it ran.
const SAYS: &str = "khalid stand-in 9.8.7";

/// The first line of the completion script the stand-in prints.
const COMPLETES: &str = "# khalid completion for";

/// A folder laid out like GitHub's release downloads, a folder to install
/// into, and the home folder of the person installing.
struct Release {
    root: TempDir,
    into: TempDir,
    home: TempDir,
}

/// The platform name the script picks on this machine.
#[cfg(test)]
fn platform() -> String {
    let os = match std::env::consts::OS {
        "macos" => "macos",
        _ => "linux",
    };
    format!("{os}-{}", std::env::consts::ARCH)
}

#[cfg(test)]
fn file_name() -> String {
    format!("khalid-{VERSION}-{}", platform())
}

/// The path of `name` in the release's folder.
#[cfg(test)]
fn in_release(release: &Release, name: &str) -> PathBuf {
    let mut path = release.root.path().to_path_buf();
    path.push(VERSION);
    path.push(name);
    path
}

/// The SHA-256 of a file, from the same tool the script uses.
#[cfg(test)]
fn sha256(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .or_else(|_| {
            Command::new("shasum")
                .args(["-a", "256"])
                .arg(path)
                .output()
        })
        .unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    text.split_whitespace().next().unwrap().to_owned()
}

/// The name of the release's manual pages archive.
#[cfg(test)]
fn man_name() -> String {
    format!("khalid-{VERSION}-man.tar.gz")
}

/// A release holding a stand-in binary, its manual pages, and a SHA256SUMS
/// that matches both. The stand-in prints a completion script when asked.
#[cfg(test)]
fn release() -> Release {
    let release = Release {
        root: tempfile::tempdir().unwrap(),
        into: tempfile::tempdir().unwrap(),
        home: tempfile::tempdir().unwrap(),
    };
    fs::create_dir_all(in_release(&release, "")).unwrap();
    let binary = in_release(&release, &file_name());
    let stand_in = format!(
        "#!/bin/sh\nif [ \"$1\" = completions ]; then echo \"{COMPLETES} $2\"; exit 0; fi\necho '{SAYS}'\n"
    );
    fs::write(&binary, stand_in).unwrap();
    let pages = tempfile::tempdir().unwrap();
    for page in ["khalid.1", "khalid-init.1"] {
        let mut path = pages.path().to_path_buf();
        path.push(page);
        fs::write(path, ".TH KHALID 1\n").unwrap();
    }
    let archive = in_release(&release, &man_name());
    let packed = Command::new("tar")
        .arg("-czf")
        .arg(&archive)
        .arg("-C")
        .arg(pages.path())
        .arg(".")
        .status()
        .unwrap();
    assert!(packed.success());
    let sums = format!(
        "{}  {}\n{}  {}\n{}  khalid-{VERSION}-windows-x86_64.exe\n",
        sha256(&binary),
        file_name(),
        sha256(&archive),
        man_name(),
        "0".repeat(64),
    );
    fs::write(in_release(&release, "SHA256SUMS"), sums).unwrap();
    release
}

/// Runs the script with `args`, reading releases from `release`, for a
/// person whose shell is `sh`, which has no completion to set up.
#[cfg(test)]
fn install(release: &Release, args: &[&str]) -> Output {
    install_in(release, args, "/bin/sh")
}

/// Runs the script with `args` for a person whose shell is `shell`, with
/// their home folder in `release.home`.
#[cfg(test)]
fn install_in(release: &Release, args: &[&str], shell: &str) -> Output {
    let mut script = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    script.push("../../scripts/install.sh");
    let source = format!("file://{}", release.root.path().display());
    Command::new("sh")
        .arg(script)
        .args(args)
        .env("KHALID_RELEASES", source)
        .env("HOME", release.home.path())
        .env("SHELL", shell)
        .env_remove("XDG_DATA_HOME")
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .unwrap()
}

/// Whether the tests run as root, where the script writes to
/// /usr/local/share instead of the home folder.
#[cfg(test)]
fn as_root() -> bool {
    let id = Command::new("id").arg("-u").output().unwrap();
    String::from_utf8_lossy(&id.stdout).trim() == "0"
}

/// The path of `parts` inside the test's home folder.
#[cfg(test)]
fn in_home(release: &Release, parts: &[&str]) -> PathBuf {
    let mut path = release.home.path().to_path_buf();
    path.extend(parts);
    path
}

/// The folder the tests install into, as an argument.
#[cfg(test)]
fn into(release: &Release) -> &str {
    release.into.path().to_str().unwrap()
}

#[cfg(test)]
fn installed(release: &Release) -> PathBuf {
    let mut path = release.into.path().to_path_buf();
    path.push("khalid");
    path
}

/// The refusal starts the same way every time and gives a fix on the next
/// line. Before it may stand curl's own reason, such as a missing file.
#[cfg(test)]
fn assert_refused(output: &Output, why: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    let mut lines = stderr
        .lines()
        .skip_while(|line| !line.starts_with("Khalid was not installed. "));
    let what = lines.next().unwrap_or_else(|| panic!("{stderr}"));
    assert!(what.contains(why), "{stderr}");
    assert!(lines.next().is_some(), "no fix given: {stderr}");
}

#[test]
fn the_script_installs_the_checked_binary_and_it_runs() {
    let release = release();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains(&format!("Installed Khalid {VERSION} at ")),
        "{stdout}"
    );
    let ran = Command::new(installed(&release)).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ran.stdout).trim(), SAYS);
}

#[test]
fn the_script_replaces_an_older_binary() {
    let release = release();
    fs::write(installed(&release), "#!/bin/sh\necho old\n").unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    assert!(output.status.success(), "{output:?}");
    let ran = Command::new(installed(&release)).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&ran.stdout).trim(), SAYS);
}

#[test]
fn a_download_that_does_not_match_its_checksum_is_not_installed() {
    let release = release();
    fs::write(
        in_release(&release, &file_name()),
        "#!/bin/sh\necho changed\n",
    )
    .unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    assert_refused(&output, "does not match its checksum");
    assert!(!installed(&release).exists());
}

#[test]
fn a_release_with_no_checksum_for_the_file_is_not_installed() {
    let release = release();
    fs::write(in_release(&release, "SHA256SUMS"), "").unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    assert_refused(&output, "SHA256SUMS has no line for");
    assert!(!installed(&release).exists());
}

#[test]
fn a_release_without_the_file_says_it_could_not_be_downloaded() {
    let release = release();
    fs::remove_file(in_release(&release, &file_name())).unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    assert_refused(&output, "could not be downloaded");
}

#[test]
fn a_name_that_is_not_a_release_is_refused_before_any_download() {
    let release = release();
    let output = install(&release, &["--version", "latest", "--dir", into(&release)]);
    assert_refused(&output, "latest is not a release name");
}

#[test]
fn an_unknown_option_is_refused() {
    let release = release();
    let output = install(&release, &["--prefix", "/opt"]);
    assert_refused(&output, "--prefix is not one this script knows");
}

#[test]
fn the_script_installs_the_manual_pages_in_the_home_folder() {
    let release = release();
    if as_root() {
        return;
    }
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    let page = in_home(
        &release,
        &[".local", "share", "man", "man1", "khalid-init.1"],
    );
    assert_eq!(fs::read_to_string(page).unwrap(), ".TH KHALID 1\n");
    assert!(
        stdout.contains("Installed the manual pages in "),
        "{stdout}"
    );
}

#[test]
fn a_release_without_manual_pages_still_installs_the_binary() {
    let release = release();
    fs::remove_file(in_release(&release, &man_name())).unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(installed(&release).exists());
    assert!(
        stdout.contains("The manual pages were not installed. Release "),
        "{stdout}"
    );
}

#[test]
fn manual_pages_that_do_not_match_their_checksum_are_not_installed() {
    let release = release();
    fs::write(in_release(&release, &man_name()), "changed").unwrap();
    let output = install(&release, &["--version", VERSION, "--dir", into(&release)]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(installed(&release).exists());
    assert!(!in_home(&release, &[".local", "share", "man"]).exists());
    assert!(stdout.contains("does not match SHA256SUMS"), "{stdout}");
}

#[test]
fn the_script_sets_up_completion_for_bash_zsh_and_fish() {
    if as_root() {
        return;
    }
    let places: [(&str, &[&str]); 3] = [
        (
            "bash",
            &[
                ".local",
                "share",
                "bash-completion",
                "completions",
                "khalid",
            ],
        ),
        ("zsh", &[".zfunc", "_khalid"]),
        ("fish", &[".config", "fish", "completions", "khalid.fish"]),
    ];
    for (shell, place) in places {
        let release = release();
        let login = format!("/usr/bin/{shell}");
        let output = install_in(
            &release,
            &["--version", VERSION, "--dir", into(&release)],
            &login,
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(output.status.success(), "{output:?}");
        let script = fs::read_to_string(in_home(&release, place)).unwrap();
        assert_eq!(script.trim(), format!("{COMPLETES} {shell}"));
        assert!(
            stdout.contains(&format!("Set up Tab completion for {shell} in ")),
            "{stdout}"
        );
        if shell == "zsh" {
            assert!(stdout.contains("fpath=(~/.zfunc $fpath)"), "{stdout}");
        }
    }
}

#[test]
fn a_shell_with_no_completion_gets_a_note_and_the_binary() {
    let release = release();
    let args = ["--version", VERSION, "--dir", into(&release)];
    let output = install_in(&release, &args, "/bin/tcsh");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(installed(&release).exists());
    assert!(
        stdout.contains("Tab completion was not set up. "),
        "{stdout}"
    );
}

#[test]
fn the_extras_can_be_left_out() {
    let release = release();
    let args = [
        "--version",
        VERSION,
        "--dir",
        into(&release),
        "--no-man",
        "--no-completions",
    ];
    let output = install_in(&release, &args, "/bin/bash");
    assert!(output.status.success(), "{output:?}");
    assert!(installed(&release).exists());
    let home = fs::read_dir(release.home.path()).unwrap().count();
    assert_eq!(home, 0, "{output:?}");
}

#[test]
fn the_script_is_plain_posix_sh() {
    let mut script = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    script.push("../../scripts/install.sh");
    let text = fs::read_to_string(&script).unwrap();
    assert!(text.starts_with("#!/bin/sh\n"));
    let parsed = Command::new("sh").arg("-n").arg(&script).output().unwrap();
    assert!(parsed.status.success(), "{parsed:?}");
}
