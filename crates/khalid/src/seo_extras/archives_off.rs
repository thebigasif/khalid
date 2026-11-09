//! Author and date archives switched off in `[seo.index]` (`authors =
//! "off"`, `dates = "off"`): their pages are not made, and each address
//! they had sends readers to the home page with a 301, as Yoast does for
//! a one-author blog.

use khalid_core::{ArchiveSetting, Config, UserMessage};
use khalid_render::PageBody;
use khalid_server::{Moved, PagePath};

use crate::build_files::SiteFile;
use crate::build_message;
use crate::build_pages::LanguagePages;

/// One page that is no longer made, and the home page it now sends
/// readers to, each from the site root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Gone {
    pub(crate) from: String,
    pub(crate) to: String,
}

/// Takes the author pages, the authors list and the date archives out of
/// `languages` when `config` switches them off, and returns where each one
/// was.
pub(crate) fn remove(config: &Config, languages: &mut [LanguagePages]) -> Vec<Gone> {
    let index = &config.seo().index;
    let authors_off = index.authors == ArchiveSetting::Off;
    let dates_off = index.dates == ArchiveSetting::Off;
    let mut gone = Vec::new();
    for language in languages {
        let home = language.site.home.clone();
        language.pages.retain(|page| {
            let off = match page.body {
                PageBody::Author(_) | PageBody::Authors(_) => authors_off,
                PageBody::Date(_) => dates_off,
                _ => false,
            };
            if off {
                gone.push(Gone {
                    from: page.meta.path.clone(),
                    to: home.clone(),
                });
            }
            !off
        });
    }
    gone
}

/// The moves `khalid serve` answers for `gone`.
pub(crate) fn moves(gone: &[Gone]) -> Result<Vec<Moved>, UserMessage> {
    let path = |text: &str| PagePath::try_new(text).map_err(|_| build_message::address(text));
    gone.iter()
        .map(|gone| Ok(Moved::new(path(&gone.from)?, path(&gone.to)?)))
        .collect()
}

/// The `_redirects` file `khalid build` writes, with a 301 line for each of
/// `gone` after the lines it already has.
pub(crate) fn with_redirects(file: Option<SiteFile>, gone: &[Gone]) -> Option<SiteFile> {
    if gone.is_empty() {
        return file;
    }
    let mut file = file.unwrap_or_else(|| SiteFile {
        file: "_redirects".to_owned(),
        text: String::new(),
    });
    for gone in gone {
        for part in [gone.from.as_str(), " ", gone.to.as_str(), " 301\n"] {
            file.text.push_str(part);
        }
    }
    Some(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_gone_page_becomes_a_redirect_line_after_the_others() {
        let gone = [Gone {
            from: "/blog/authors/asif/".to_owned(),
            to: "/blog/".to_owned(),
        }];
        let file = SiteFile {
            file: "_redirects".to_owned(),
            text: "/blog/old/ /blog/new/ 301\n".to_owned(),
        };
        let text = with_redirects(Some(file), &gone).map(|file| file.text);
        assert_eq!(
            text.as_deref(),
            Some("/blog/old/ /blog/new/ 301\n/blog/authors/asif/ /blog/ 301\n")
        );
        assert!(with_redirects(None, &[]).is_none());
        assert_eq!(moves(&gone).unwrap().len(), 1);
    }
}
