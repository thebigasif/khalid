//! The imprint page, the Impressum German law asks a website for: who runs
//! it and how to reach them (backlog row 36). The template holds the words;
//! this type holds only the owner's details.

use serde::Serialize;

/// The generated imprint of one language, at `<mount>impressum/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderImprint {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// The person or company that runs the site.
    pub name: String,
    /// Their postal address, on one line.
    pub address: String,
    /// Their email address.
    pub email: String,
    /// Their phone number.
    pub phone: Option<String>,
    /// The trade register entry, such as `Amtsgericht Berlin, HRB 12345`.
    pub register: Option<String>,
    /// The VAT identification number.
    pub vat_id: Option<String>,
    /// Who answers for the site's articles, with their address.
    pub responsible: Option<String>,
    /// The address of the privacy page in the same language.
    pub privacy: Option<String>,
}
