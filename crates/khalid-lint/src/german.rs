//! German word lists for the rules that read words.
//!
//! `typos.toml` skips this file, because its English dictionary reads German
//! words such as "oder" as typos. Keep only word lists here.

/// Small words that carry no topic.
pub(crate) const STOPWORDS: &[&str] = &[
    "als", "am", "auch", "auf", "aus", "bei", "das", "dass", "dem", "den", "der", "des", "die",
    "ein", "eine", "einen", "einer", "eines", "es", "für", "hat", "ihr", "im", "in", "ist", "kann",
    "man", "mit", "nach", "nicht", "noch", "nur", "oder", "sich", "sie", "sind", "so", "um", "und",
    "von", "was", "wenn", "wie", "wir", "wird", "zu", "zum", "zur",
];

/// Question words that open a heading but never name its subject. "was"
/// and "wie" are stopwords already.
pub(crate) const QUESTION_WORDS: &[&str] = &[
    "wann", "warum", "welche", "welchem", "welchen", "welcher", "welches", "wer", "weshalb",
    "wieso", "wo", "woher", "wohin",
];

/// Title words that ask for a list of the best picks.
pub(crate) const BEST_OF_WORDS: &[&str] = &["beste", "besten", "bester", "bestes", "top"];

/// Title words that ask for the choices beside one thing.
pub(crate) const ALTERNATIVES_WORDS: &[&str] = &["alternative", "alternativen"];

/// Title words that ask for a verdict on one thing.
pub(crate) const REVIEW_WORDS: &[&str] = &[
    "bewertung",
    "erfahrungen",
    "erfahrungsbericht",
    "rezension",
    "test",
    "testbericht",
];

/// Short forms that end with a full stop inside a sentence, in lowercase.
pub(crate) const ABBREVIATIONS: &[&str] = &[
    "z.", "b.", "z.b.", "d.h.", "u.a.", "usw.", "bzw.", "ca.", "dr.", "nr.", "vgl.", "evtl.",
    "ggf.", "inkl.", "s.",
];

/// Linking words and phrases, in lowercase, between bars.
pub(crate) const TRANSITIONS: &str = "aber|allerdings|also|anfangs|anschließend|außerdem|\
     beispielsweise|bevor|dadurch|dagegen|daher|damit|danach|dann|darum|demnach|denn|deshalb|\
     deswegen|doch|drittens|ebenfalls|ebenso|einerseits|andererseits|erstens|ferner|folglich|\
     gleichzeitig|hingegen|indem|insbesondere|inzwischen|jedoch|letztlich|nachdem|nämlich|\
     obwohl|schließlich|sodass|somit|sonst|später|stattdessen|trotzdem|übrigens|während|weil|\
     weiterhin|zudem|zuerst|zuletzt|zunächst|zweitens|zusammenfassend|aus diesem grund|\
     das heißt|im gegensatz|im vergleich|mit anderen worten|vor allem|zum beispiel|zum schluss";

/// Long words that readers meet every day, more than ten letters, in
/// lowercase, between spaces.
pub(crate) const COMMON_LONG: &str = " ansprechpartner anforderungen beispielsweise \
     beschreibung bestellungen beziehungsweise bundesregierung durchschnitt einstellungen \
     empfehlungen entscheidung entsprechend entsprechende entsprechenden entwicklung \
     entwicklungen erfahrungen gemeinsamen geschichten gesellschaft gleichzeitig grundsätzlich \
     herausforderung herausforderungen informationen inhaltsverzeichnis insbesondere \
     internetseite kommunikation kundenservice mitarbeiter mitarbeitern mitarbeiterinnen \
     mittlerweile möglichkeit möglichkeiten nachrichten öffentlichkeit organisation \
     persönlichen regelmäßig schließlich selbstverständlich suchergebnisse suchmaschine \
     suchmaschinen tatsächlich teilnehmerin teilnehmern überschriften unterschied \
     unterschiede unterschiedlich unterschiedliche unterschiedlichen unternehmen \
     unternehmens unternehmer unterstützung untersuchung verantwortung veranstaltung \
     verschiedene verschiedenen verschiedener verschiedenste voraussetzung voraussetzungen \
     wahrscheinlich wirtschaftlich wissenschaft zahlreichen zusammenarbeit zusammenhang \
     zusätzliche zusätzlich ";

/// Everyday words that are not filler words but never name a topic, in
/// lowercase, between spaces, left out of the prominent words.
pub(crate) const EVERYDAY: &str = " aber alle auch bei bis dass diese dieser dieses doch durch \
     eine einem einen einer eines hier ihre immer jetzt können mehr muss nur schon sehr sein \
     seine über unter viel vom war werden wurde ";

/// Forms of "werden" that build the passive, "worden" included.
pub(crate) const WERDEN: &[&str] = &[
    "werde", "wirst", "wird", "werden", "werdet", "wurde", "wurdest", "wurden", "wurdet", "würde",
    "würdest", "würden", "worden",
];

/// Words that look like a past participle but are not: infinitives and
/// other words that start with "ge" and end in "-en" or "-t", between
/// spaces.
const NOT_PARTICIPLES: &str = " geben gebrauchen gefallen gefährden gegen gehen gehören \
     gelingen gelten genießen genügen geraten gerecht gesamt geschehen gestatten gestalten \
     gestehen gewähren gewinnen gewöhnen ";

/// Separable prefixes, which come before the "ge" of a participle, as in
/// "eingesetzt".
const SEPARABLE: &[&str] = &[
    "ab", "an", "auf", "aus", "bei", "dar", "ein", "fest", "her", "hin", "los", "mit", "nach",
    "um", "vor", "weg", "zu", "zurück",
];

/// Inseparable prefixes, whose weak participles end in "-t" with no "ge",
/// as in "verkauft".
const INSEPARABLE: &[&str] = &["be", "emp", "ent", "er", "ge", "miss", "ver", "zer"];

/// Whether `word`, in lowercase, is a past participle: "gelesen",
/// "eingesetzt", "verkauft" or "optimiert".
pub(crate) fn participle(word: &str) -> bool {
    let ge = |rest: &str| {
        rest.starts_with("ge")
            && rest.chars().count() >= 5
            && (rest.ends_with('t') || rest.ends_with("en"))
            && !NOT_PARTICIPLES.contains(&format!(" {rest} "))
    };
    let separable = SEPARABLE
        .iter()
        .filter_map(|prefix| word.strip_prefix(prefix))
        .any(ge);
    let weak = INSEPARABLE
        .iter()
        .filter_map(|prefix| word.strip_prefix(prefix))
        .any(|rest| rest.chars().count() >= 3 && rest.ends_with('t') && !rest.ends_with("st"));
    ge(word) || separable || weak || (word.ends_with("iert") && word.chars().count() > 5)
}

/// German sentences whose only repeated phrase is grammar, for the tests.
#[cfg(test)]
pub(crate) const GRAMMAR_ONLY: &str =
    "Nutzen Sie Links. Nutzen Sie Titel. Nutzen Sie Bilder. Fertig.";

/// Link words that say nothing about the page they open.
pub(crate) const VAGUE_LINK_TEXT: &[&str] = &[
    "hier",
    "hier klicken",
    "klicken sie hier",
    "mehr",
    "mehr lesen",
    "weiterlesen",
    "dieser artikel",
    "link",
];
