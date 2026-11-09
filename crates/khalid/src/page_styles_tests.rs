use super::*;

/// A reader script that adds the class `lightbox`.
const SCRIPT: &str = "dialog.className = \"lightbox\";\n";

const CSS: &str = ":root {\n--a: 1;\n}\n\
    @font-face {\nfont-family: X;\nsrc: url(\"x.woff2\");\n}\n\
    .card, .post h2 {\ncolor: red;\n}\n\
    .home .card {\nmargin: 0;\n}\n\
    a:not(.button) {\ncolor: blue;\n}\n\
    @media (min-width: 40rem) {\n.home {\npadding: 0;\n}\n.post {\npadding: 1rem;\n}\n}\n\
    @keyframes spin {\nto {\ntransform: rotate(1turn);\n}\n}\n\
    .lightbox {\ndisplay: grid;\n}\n";

#[test]
fn a_page_keeps_only_the_rules_its_classes_can_match() {
    let styles = PageStyles::new(CSS, "", SCRIPT);
    let post = styles.for_page("<main><article class=\"post\"><h2>A</h2></article></main>");
    let text = post.text();
    assert!(text.contains(":root {\n--a: 1;\n}"), "{text}");
    assert!(text.contains("@font-face"), "{text}");
    assert!(text.contains(".post h2 {\ncolor: red;\n}"), "{text}");
    assert!(!text.contains(".card"), "{text}");
    assert!(!text.contains(".home"), "{text}");
    assert!(text.contains("a:not(.button)"), "{text}");
    assert!(
        text.contains("@media (min-width: 40rem) {\n.post {"),
        "{text}"
    );
    assert!(text.contains("@keyframes spin"), "{text}");
}

#[test]
fn the_reader_scripts_classes_are_on_every_page() {
    let styles = PageStyles::new(CSS, "", SCRIPT);
    assert!(styles.for_page("<p>x</p>").text().contains(".lightbox"));
}

#[test]
fn a_page_with_every_class_gets_the_whole_stylesheet() {
    let styles = PageStyles::new(CSS, "", SCRIPT);
    let html = "<div class=\"home\"><p class='card'></p><b CLASS = post></b></div>";
    let mut whole = String::new();
    for line in CSS.lines() {
        whole.push_str(line);
        whole.push('\n');
    }
    assert_eq!(styles.for_page(html).text(), whole);
}

#[test]
fn pages_with_the_same_classes_share_one_sheet() {
    let styles = PageStyles::new(CSS, "", SCRIPT);
    let one = styles.for_page("<a class=\"post\">1</a>");
    let two = styles.for_page("<p class=\"post\">2</p><p class=\"unstyled\"></p>");
    assert!(Arc::ptr_eq(&one, &two));
    assert_eq!(
        one.name(),
        Asset::hashed_name("site", "css", one.text().as_bytes())
    );
    let home = styles.for_page("<a class=\"home\">1</a>");
    assert_ne!(one.name(), home.name());
}

#[test]
fn strings_escapes_and_brackets_are_read_as_written() {
    let read = |prelude: &str| {
        let mut classes = BTreeMap::new();
        let found = selectors(prelude, &mut classes);
        let mut names: Vec<(usize, String)> = classes.into_iter().map(|(k, v)| (v, k)).collect();
        names.sort();
        let name = |bit: usize| names[bit].1.clone();
        found
            .into_iter()
            .map(|selector| {
                (
                    selector.text,
                    selector.needs.into_iter().map(name).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    let none: Vec<String> = Vec::new();
    assert_eq!(
        read(".a > .b:is(.c, .d) [data-x=\".e\"]"),
        [(
            ".a > .b:is(.c, .d) [data-x=\".e\"]".to_owned(),
            vec!["a".to_owned(), "b".to_owned()]
        )]
    );
    assert_eq!(
        read(".sm\\:wide"),
        [(".sm\\:wide".to_owned(), none.clone())]
    );
    assert_eq!(read("p.4"), [("p.4".to_owned(), none)]);
    let texts: Vec<String> = read(".a, :is(.b, .c) , .d")
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert_eq!(texts, [".a", ":is(.b, .c)", ".d"]);
    let styles = PageStyles::new(
        ".q::before {\ncontent: \"}\";\n}\n.r {\nx: y;\n}\n",
        "",
        SCRIPT,
    );
    let text = styles.for_page("<i class=\"q\"></i>").text().to_owned();
    assert_eq!(text, ".q::before {\ncontent: \"}\";\n}\n");
}

#[test]
fn a_broken_stylesheet_is_kept_from_where_it_breaks() {
    let styles = PageStyles::new(".alpha {\nx: y;\n}\n.beta {\nnever: closed;\n", "", SCRIPT);
    assert_eq!(
        styles.for_page("<p></p>").text(),
        ".beta {\nnever: closed;\n\n"
    );
}

#[test]
fn the_rules_kept_whole_stay_on_every_page() {
    let styles = PageStyles::new(".post {\nx: y;\n}\n", ".mine {\nx: z;\n}\n", SCRIPT);
    assert_eq!(styles.for_page("<p></p>").text(), ".mine {\nx: z;\n}\n");
}

#[test]
fn the_script_words_are_those_inside_its_strings() {
    let js = "// \"skip\"\nx.className = \"lightbox big\";\nlet t = `a ${b}\nc`;\nlet card = 'it\\'s';\n";
    assert_eq!(
        script_words(js),
        ["lightbox", "big", "a", "b", "c", "it", "s"]
    );
}
