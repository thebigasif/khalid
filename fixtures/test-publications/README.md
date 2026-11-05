# A sample demo publication, for the tests only

`harbor/` is a tiny demo publication in the shape `fixtures/showcase-publications/README.md` sets out: two posts, one category, one author and an about page. The tests in `crates/khalid/tests/website.rs` and `scripts/demo-check/publication-selftest.sh` apply it to a copy of the showcase site with `scripts/demo-publication.sh`, by pointing `DEMO_PUBLICATIONS` at this folder and `DEMO_PHOTOS_URL` at `demo-photos.tar.gz` here.

It is marked ready so it applies as a finished publication does. It never moves to `fixtures/showcase-publications/`, so no real demo shows it.

`demo-photos.tar.gz` holds its two pictures at `harbor/<file>`, where `<file>` is the path `photos.lock` names. Each picture is a 187-byte PNG of one colour, 1200 by 630 px, made for the tests. To change one, rebuild the archive with `tar --sort=name --owner=0 --group=0 --mtime=@0 -czf` and write the new checksum into `photos.lock`.
