//! Reader lists from Khalid, Substack and Ghost (backlog row 9): each
//! reader keeps the status the source gave them, and Khalid's export reads
//! back the same.

#![cfg(test)]

use chrono::{TimeZone, Utc};
use khalid_core::Slug;
use khalid_import::subscribers::{
    Reader, ReaderRowError, ReaderSource, ReadersError, Standing, read_readers,
};

fn slugs(names: &[&str]) -> Vec<Slug> {
    names
        .iter()
        .map(|name| Slug::try_new(name).unwrap())
        .collect()
}

/// The readers without their lines.
fn only_readers(read: &[(usize, Reader)]) -> Vec<&Reader> {
    read.iter().map(|(_, reader)| reader).collect()
}

/// Substack's `email_list` file, as its export writes it.
const SUBSTACK: &str = "email,active_subscription,expiry,plan,email_disabled,created_at\n\
lena@example.com,false,,other,false,2021-03-04T05:06:07.000Z\n\
omar@example.com,true,2027-01-01T00:00:00.000Z,yearly,false,2022-01-01T00:00:00.000Z\n\
ines@example.org,false,,other,true,2020-05-06T07:08:09.000Z\n";

/// Ghost's members export, as Ghost 5 writes it.
const GHOST: &str = "id,email,name,note,subscribed_to_emails,complimentary_plan,\
stripe_customer_id,created_at,deleted_at,labels,tiers\n\
6512,lena@example.com,Lena,,true,false,,2023-01-02T03:04:05.000Z,,\"VIP, From Launch\",\n\
6513,omar@example.com,Omar,,true,false,cus_123,2023-02-03 04:05:06,,,Gold\n\
6514,ines@example.org,Ines,,false,true,,2023-03-04T05:06:07.000Z,,,\n";

#[test]
fn substack_readers_keep_their_status_and_paid_ones_get_a_label() {
    let read = read_readers(SUBSTACK).unwrap();
    assert_eq!(read.source, ReaderSource::Substack);
    assert_eq!(read.unread, []);
    let readers = only_readers(&read.readers);
    let standing: Vec<_> = readers
        .iter()
        .map(|r| (r.email.as_str(), r.standing))
        .collect();
    assert_eq!(
        standing,
        [
            ("lena@example.com", Standing::Confirmed),
            ("omar@example.com", Standing::Confirmed),
            ("ines@example.org", Standing::Unsubscribed),
        ]
    );
    assert_eq!(read.readers[2].0, 4);
    assert_eq!(readers[1].labels, slugs(&["paid"]));
    assert_eq!(
        readers[0].since,
        Some(Utc.with_ymd_and_hms(2021, 3, 4, 5, 6, 7).unwrap())
    );
}

#[test]
fn ghost_members_keep_their_status_and_labels() {
    let read = read_readers(GHOST).unwrap();
    assert_eq!(read.source, ReaderSource::Ghost);
    assert_eq!(read.unread, []);
    let [lena, omar, ines] = only_readers(&read.readers)[..] else {
        panic!("three members: {:?}", read.readers);
    };
    assert_eq!(
        (lena.standing, &lena.labels),
        (Standing::Confirmed, &slugs(&["from-launch", "vip"]))
    );
    assert_eq!(
        (omar.standing, &omar.labels),
        (Standing::Confirmed, &slugs(&["paid"]))
    );
    assert_eq!(
        omar.since,
        Some(Utc.with_ymd_and_hms(2023, 2, 3, 4, 5, 6).unwrap())
    );
    assert_eq!(
        (ines.standing, &ines.labels),
        (Standing::Unsubscribed, &slugs(&["paid"]))
    );
}

#[test]
fn a_row_khalid_cannot_read_is_left_out_with_why() {
    let text = "email,status,created_at,lists,labels\n\
lena@example.com,confirmed,,main,\n\
,confirmed,,main,\n\
omar@example.com,gone,,main,\n\
ines@example.org,pending,yesterday,main,\n\
kim@example.com,pending,,Main!,\n";
    let read = read_readers(text).unwrap();
    assert_eq!(read.readers.len(), 1);
    assert_eq!(
        read.unread,
        [
            (3, ReaderRowError::NoEmail),
            (4, ReaderRowError::Status("gone".to_owned())),
            (5, ReaderRowError::Date("yesterday".to_owned())),
            (6, ReaderRowError::Slug("Main!".to_owned())),
        ]
    );
}

#[test]
fn a_file_that_is_no_reader_list_is_refused() {
    for text in [
        "",
        "post_id,title\n1,Hello\n",
        "email,name\nlena@example.com,Lena\n",
    ] {
        assert_eq!(read_readers(text), Err(ReadersError::Unknown), "{text:?}");
    }
    assert!(matches!(
        read_readers("email,status\n\"open"),
        Err(ReadersError::Csv(_))
    ));
}
