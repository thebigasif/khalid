//! `scripts/homebrew-formula.sh`: the Homebrew formula for a release points
//! at each binary the release workflow builds, with the checksum from the
//! release's `SHA256SUMS`, and the script refuses a release it cannot check
//! with a message that says what, why and what to do (backlog 4 row 38,
//! KH-INS-1). `brew` itself installs the formula in the Homebrew workflow.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

/// The release name every test writes a formula for.
const TAG: &str = "v9.8.7";

/// The platforms Homebrew installs on, as the release workflow names them,
/// in the order the formula lists them.
const PLATFORMS: [&str; 4] = [
    "macos-aarch64",
    "macos-x86_64",
    "linux-aarch64",
    "linux-x86_64",
];

/// The path of a file in the repository.
#[cfg(test)]
fn repo_file(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}

/// The release file name for `platform`.
#[cfg(test)]
fn binary(platform: &str) -> String {
    format!("khalid-{TAG}-{platform}")
}

/// The manual pages archive of the release.
#[cfg(test)]
fn man() -> String {
    format!("khalid-{TAG}-man.tar.gz")
}

/// A made-up checksum, different for each `seed`.
#[cfg(test)]
fn sum(seed: char) -> String {
    seed.to_string().repeat(64)
}

/// The release's files and their made-up checksums, as `SHA256SUMS` lists
/// them. One line uses the `*` that marks a file read in binary mode.
#[cfg(test)]
fn release_files() -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = PLATFORMS
        .iter()
        .zip(['1', '2', '3', '4'])
        .map(|(platform, seed)| (binary(platform), sum(seed)))
        .collect();
    files.push((man(), sum('5')));
    files.push((format!("*khalid-{TAG}-windows-x86_64.exe"), sum('6')));
    files
}

/// Runs the script for `tag` with a `SHA256SUMS` holding `files`.
#[cfg(test)]
fn formula(tag: &str, files: &[(String, String)]) -> Output {
    let folder = tempfile::tempdir().unwrap();
    let mut sums = folder.path().to_path_buf();
    sums.push("SHA256SUMS");
    let lines: Vec<String> = files
        .iter()
        .map(|(name, sum)| format!("{sum}  {name}\n"))
        .collect();
    fs::write(&sums, lines.concat()).unwrap();
    Command::new("sh")
        .arg(repo_file("scripts/homebrew-formula.sh"))
        .arg(tag)
        .arg(&sums)
        .output()
        .unwrap()
}

/// The formula for the whole made-up release.
#[cfg(test)]
fn written() -> String {
    let output = formula(TAG, &release_files());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    String::from_utf8(output.stdout).unwrap()
}

/// The refusal starts the same way every time, says `why`, gives a fix on
/// the next line, and writes nothing to standard output.
#[cfg(test)]
fn assert_refused(output: &Output, why: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(output.stdout.is_empty(), "{stderr}");
    let mut lines = stderr.lines();
    let what = lines.next().unwrap_or_else(|| panic!("{stderr}"));
    assert!(what.starts_with("No formula was written. "), "{stderr}");
    assert!(what.contains(why), "{stderr}");
    assert!(lines.next().is_some(), "no fix given: {stderr}");
}

#[test]
fn the_formula_names_every_binary_with_its_checksum() {
    let formula = written();
    let lines: Vec<&str> = formula.lines().map(str::trim).collect();
    let download = format!("https://github.com/thebigasif/khalid/releases/download/{TAG}");
    for (name, sum) in release_files().iter().take(5) {
        let pair = [
            &format!("url \"{download}/{name}\""),
            &format!("sha256 \"{sum}\""),
        ];
        assert!(lines.windows(2).any(|two| two == pair), "{formula}");
    }
    assert!(formula.contains("  version \"9.8.7\"\n"), "{formula}");
    assert!(!formula.contains("windows"), "{formula}");
}

/// Homebrew picks the block for the machine it runs on, so each file must
/// sit inside the blocks for its system and its processor.
#[test]
fn each_binary_sits_in_the_block_for_its_system_and_processor() {
    let formula = written();
    let order = [
        "on_macos do",
        "on_arm do",
        &binary(PLATFORMS[0]),
        "on_intel do",
        &binary(PLATFORMS[1]),
        "on_linux do",
        "on_arm do",
        &binary(PLATFORMS[2]),
        "on_intel do",
        &binary(PLATFORMS[3]),
        "resource \"man\" do",
        &man(),
    ];
    let mut from = 0;
    for wanted in order {
        let rest = formula.get(from..).unwrap();
        let at = rest
            .find(wanted)
            .unwrap_or_else(|| panic!("{wanted}: {formula}"));
        from += at + wanted.len();
    }
}

/// The names the formula downloads are the ones the release workflow
/// gives its files, so a renamed build fails here, not on a user's Mac.
#[test]
fn the_formula_downloads_the_files_the_release_workflow_builds() {
    let workflow = fs::read_to_string(repo_file(".github/workflows/release.yml")).unwrap();
    for platform in PLATFORMS {
        assert!(
            workflow.contains(&format!("- {{ name: {platform}, ")),
            "release.yml builds no {platform}"
        );
    }
    for name in [
        "khalid-${{ github.ref_name }}-${{ matrix.name }}${{ matrix.exe }}",
        "khalid-${{ github.ref_name }}-man.tar.gz",
        "sha256sum khalid-* > SHA256SUMS",
    ] {
        assert!(workflow.contains(name), "release.yml no longer has {name}");
    }
}

/// The formula installs the binary, the manual pages and Tab completion,
/// and its test makes a site as the README does (`packages.rs`).
#[test]
fn the_formula_installs_the_binary_pages_and_completion_and_tests_a_site() {
    let formula = written();
    for line in [
        "bin.install Dir[\"khalid-v*\"].first => \"khalid\"",
        "resource(\"man\").stage { man1.install Dir[\"*.1\"] }",
        "generate_completions_from_executable(bin/\"khalid\", \"completions\")",
        "assert_match version.to_s, shell_output(\"#{bin}/khalid --version\")",
        "system bin/\"khalid\", \"init\", testpath/\"blog\", \"--address\", \"https://blog.example.com\", \"--yes\"",
    ] {
        assert!(formula.contains(line), "{formula}");
    }
}

/// Homebrew wants the summary without a leading article or a full stop.
#[test]
fn the_formula_licence_and_summary_match_the_crate() {
    let formula = written();
    let workspace = fs::read_to_string(repo_file("Cargo.toml")).unwrap();
    let crate_toml = fs::read_to_string(repo_file("crates/khalid/Cargo.toml")).unwrap();
    let value = |text: &str, key: &str| {
        let prefix = format!("{key} = \"");
        let line = text.lines().find_map(|line| line.strip_prefix(&prefix));
        line.unwrap_or_else(|| panic!("no {key}"))
            .trim_end_matches('"')
            .to_owned()
    };
    let licence = value(&workspace, "license");
    assert!(
        formula.contains(&format!("  license \"{licence}\"\n")),
        "{formula}"
    );
    let description = value(&crate_toml, "description");
    let summary = description
        .strip_prefix("An open")
        .and_then(|rest| rest.strip_suffix('.'))
        .unwrap_or_else(|| panic!("{description}"));
    assert!(
        formula.contains(&format!("  desc \"Open{summary}\"\n")),
        "{formula}"
    );
}

#[test]
fn a_release_with_no_checksum_for_a_file_is_refused() {
    for missing in [binary(PLATFORMS[2]), man()] {
        let files: Vec<(String, String)> = release_files()
            .into_iter()
            .filter(|(name, _)| *name != missing)
            .collect();
        assert_refused(&formula(TAG, &files), &format!("has no line for {missing}"));
    }
}

#[test]
fn a_checksum_that_is_not_one_sha256_is_refused() {
    let mut short = release_files();
    short[0].1 = "abc".to_owned();
    let mut twice = release_files();
    twice.push((man(), sum('7')));
    for files in [short, twice] {
        assert_refused(&formula(TAG, &files), "is not one SHA-256 checksum");
    }
}

#[test]
fn a_prerelease_or_a_name_that_is_not_a_release_is_refused() {
    let files = release_files();
    assert_refused(&formula("v9.8.7-rc.1", &files), "is a prerelease");
    for tag in ["9.8.7", "v9..7", "v9.8", "v9.8.7.", "vnext", "v9.8.x"] {
        assert_refused(&formula(tag, &files), "is not a release name");
    }
    let alone = Command::new("sh")
        .arg(repo_file("scripts/homebrew-formula.sh"))
        .arg(TAG)
        .output()
        .unwrap();
    assert_refused(&alone, "needs a release name and its SHA256SUMS file");
}

/// Ruby reads the formula as code. It runs where Ruby is installed, as on
/// the CI runners, and is skipped elsewhere.
#[test]
fn the_formula_is_ruby_that_parses() {
    let Ok(probe) = Command::new("ruby").arg("--version").output() else {
        eprintln!("Ruby is not installed here, so the formula is not parsed");
        return;
    };
    assert!(probe.status.success());
    let folder = tempfile::tempdir().unwrap();
    let mut path = folder.path().to_path_buf();
    path.push("khalid.rb");
    fs::write(&path, written()).unwrap();
    let parsed = Command::new("ruby").arg("-c").arg(&path).output().unwrap();
    let said = String::from_utf8_lossy(&parsed.stderr);
    assert!(parsed.status.success(), "{said}");
}
