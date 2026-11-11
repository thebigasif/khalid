//! The redirect files `khalid build` writes beside the pages: `_redirects`
//! for Netlify and Cloudflare Pages, `nginx-redirects.conf` for nginx and
//! `.htaccess` for Apache, each with the same moves in the same order, so
//! a moved post keeps its links on any static host (KH-SEO-8, plan section
//! 7, gap audit 4 row 28).

use std::fmt::Write as _;

use crate::build_files::SiteFile;

/// One move a static host answers: an old path from the site root, mount
/// folder included, where it now leads, and the status, one of 301, 302,
/// 307 or 308.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RedirectLine {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) status: u16,
}

/// The first line of the nginx and Apache files.
const HEAD: &str = "# Redirects from khalid build.";

/// The three redirect files for `lines`, or none when there is nothing to
/// redirect.
pub(crate) fn files(lines: &[RedirectLine]) -> Vec<SiteFile> {
    if lines.is_empty() {
        return Vec::new();
    }
    vec![netlify(lines), nginx(lines), apache(lines)]
}

/// `_redirects`: one `from to status` line per move.
fn netlify(lines: &[RedirectLine]) -> SiteFile {
    let mut text = String::new();
    for line in lines {
        let _ = writeln!(text, "{} {} {}", line.from, line.to, line.status);
    }
    SiteFile {
        file: "_redirects".to_owned(),
        text,
    }
}

/// `nginx-redirects.conf`: one exact `location` per move, for an `include`
/// inside the blog's `server` block.
fn nginx(lines: &[RedirectLine]) -> SiteFile {
    let mut text = format!(
        "{HEAD} Add `include /path/to/nginx-redirects.conf;` inside the blog's server block.\n"
    );
    for line in lines {
        let _ = writeln!(
            text,
            "location = \"{}\" {{ return {} \"{}\"; }}",
            line.from.replace('"', "\\\""),
            line.status,
            encoded(&line.to),
        );
    }
    SiteFile {
        file: "nginx-redirects.conf".to_owned(),
        text,
    }
}

/// `.htaccess`: one anchored `RedirectMatch` per move, which Apache reads
/// from the folder it serves when `AllowOverride` lets it.
fn apache(lines: &[RedirectLine]) -> SiteFile {
    let mut text = format!("{HEAD} Apache reads it with mod_alias.\n<IfModule mod_alias.c>\n");
    for line in lines {
        let _ = writeln!(
            text,
            "RedirectMatch {} \"^{}$\" \"{}\"",
            line.status,
            pattern(&line.from),
            encoded(&line.to).replace('&', "\\&"),
        );
    }
    text.push_str("</IfModule>\n");
    SiteFile {
        file: ".htaccess".to_owned(),
        text,
    }
}

/// `path` as a regular expression that matches only itself, quotes
/// escaped for Apache's quoted argument.
fn pattern(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for c in path.chars() {
        if matches!(
            c,
            '.' | '^'
                | '$'
                | '|'
                | '?'
                | '*'
                | '+'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '\\'
                | '"'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// `to` with every byte nginx or Apache would read as code, and every
/// byte outside ASCII, percent-encoded: `$` starts a variable in both, and
/// a quote or a brace would end the line's value.
fn encoded(to: &str) -> String {
    let mut out = String::with_capacity(to.len());
    for c in to.chars() {
        let plain = c.is_ascii_graphic()
            && !matches!(
                c,
                '"' | '\\' | '$' | '{' | '}' | '<' | '>' | '^' | '`' | '|'
            );
        if plain {
            out.push(c);
            continue;
        }
        let mut bytes = [0; 4];
        for byte in c.encode_utf8(&mut bytes).bytes() {
            let _ = write!(out, "%{byte:02X}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(from: &str, to: &str, status: u16) -> RedirectLine {
        RedirectLine {
            from: from.to_owned(),
            to: to.to_owned(),
            status,
        }
    }

    fn text<'a>(files: &'a [SiteFile], name: &str) -> &'a str {
        files
            .iter()
            .find(|file| file.file == name)
            .map(|file| file.text.as_str())
            .unwrap()
    }

    #[test]
    fn no_moves_write_no_files() {
        assert!(files(&[]).is_empty());
    }

    #[test]
    fn special_characters_cannot_break_out_of_a_line() {
        let files = files(&[line(
            "/blog/a.b(c)\"$/",
            "https://example.com/über?x=1&y=$2",
            308,
        )]);
        assert_eq!(
            text(&files, "_redirects"),
            "/blog/a.b(c)\"$/ https://example.com/über?x=1&y=$2 308\n"
        );
        assert!(
            text(&files, "nginx-redirects.conf").ends_with(
                "location = \"/blog/a.b(c)\\\"$/\" \
                 { return 308 \"https://example.com/%C3%BCber?x=1&y=%242\"; }\n"
            ),
            "{files:?}"
        );
        assert!(
            text(&files, ".htaccess").contains(
                "RedirectMatch 308 \"^/blog/a\\.b\\(c\\)\\\"\\$/$\" \
                 \"https://example.com/%C3%BCber?x=1\\&y=%242\"\n"
            ),
            "{files:?}"
        );
    }
}
