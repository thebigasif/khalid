//! The command line of the commands that keep a site's stored data:
//! `khalid webhooks`, `redirects` and `db`. Kept apart from `cli` to keep
//! each file short (BL-SIZE-1); `cli` names them all.

use std::path::PathBuf;

use clap::Subcommand;
use khalid_webhooks::Event;

/// The webhook commands. Webhooks live in `data/site.db`; a running
/// `khalid serve` sees a change at once.
#[derive(Debug, Clone, Subcommand)]
pub(crate) enum WebhooksCommand {
    /// Send one event to one address, signed with a new secret shown once
    Add {
        /// The event to send, such as post.published or comment.added
        #[arg(long, value_name = "EVENT", value_parser = |text: &str| text.parse::<Event>())]
        event: Event,
        /// Where to send it: a full http or https address
        #[arg(long, value_name = "URL")]
        url: String,
    },
    /// Show every webhook that is on, without its secret
    List,
    /// Switch a webhook off. Its past deliveries stay in the log
    Remove {
        /// The webhook's identity, as list shows it
        #[arg(value_name = "ID")]
        id: i64,
    },
}

/// The redirect commands. Stored redirects live in `data/site.db`;
/// `khalid serve` reads them when it starts.
#[derive(Debug, Clone, Subcommand)]
pub(crate) enum RedirectsCommand {
    /// Store every redirect in a CSV file with the columns from,to,status,match, or none
    Import {
        /// The CSV file, such as the import/wordpress-redirects.csv an import wrote
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Print every stored redirect as CSV, to keep or open in a spreadsheet
    Export,
    /// Say which stored redirect or post alias an address matches, and where a reader ends up
    Explain {
        /// The address from the site root, such as /blog/old-links/
        #[arg(value_name = "PATH")]
        path: String,
    },
}

/// The database commands. The database is `data/site.db` in the site folder.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub(crate) enum DbCommand {
    /// Make the database if it is missing and apply every new migration
    Migrate,
    /// Empty the index and fill it again from the content files, keeping visitor data
    Rebuild,
    /// Write every table of the site as one CSV and one JSON file each, for
    /// spreadsheets and Looker Studio. Sign-in material and secrets stay out
    Export {
        /// The folder to write the files to [default: data/export in the site folder]
        #[arg(long, value_name = "FOLDER")]
        out: Option<PathBuf>,
    },
}
