//! Filler words by language: the small words that carry no topic, left
//! out when a search meets a heading (GEO M3).
//!
//! `typos.toml` skips this file, because its English dictionary reads
//! words such as "als" or "quel" as typos. Keep only word lists here.

/// The filler words of each language, in lowercase, by its language code.
const FILLER: &[(&str, &[&str])] = &[
    (
        "en",
        &[
            "a", "an", "and", "are", "as", "at", "be", "by", "can", "do", "does", "for", "from",
            "how", "has", "have", "i", "if", "in", "is", "it", "me", "my", "of", "on", "or",
            "should", "so", "that", "the", "there", "this", "to", "what", "when", "where", "which",
            "who", "why", "will", "with", "you", "your",
        ],
    ),
    (
        "de",
        &[
            "als", "am", "an", "auch", "auf", "aus", "bei", "bis", "da", "das", "dass", "dem",
            "den", "der", "des", "die", "du", "ein", "eine", "einem", "einen", "einer", "eines",
            "es", "für", "gibt", "haben", "hat", "ich", "ihr", "im", "in", "ist", "kann", "man",
            "mein", "meine", "mit", "nach", "oder", "sein", "seine", "sich", "sie", "sind", "so",
            "soll", "sollte", "um", "und", "vom", "von", "wann", "warum", "was", "welche",
            "welcher", "welches", "wer", "wie", "wir", "wird", "wo", "zu", "zum", "zur",
        ],
    ),
    (
        "fr",
        &[
            "à", "au", "aux", "avec", "c", "ce", "ces", "combien", "comment", "d", "dans", "de",
            "des", "doit", "du", "elle", "en", "est", "et", "faut", "il", "j", "je", "l", "la",
            "le", "les", "leur", "ma", "mon", "n", "ne", "on", "ou", "où", "par", "pas", "peut",
            "pour", "pourquoi", "qu", "quand", "que", "quel", "quelle", "quelles", "quels", "qui",
            "s", "sa", "se", "son", "sont", "sur", "un", "une", "vos", "votre", "vous", "y",
        ],
    ),
    (
        "es",
        &[
            "a", "al", "como", "cómo", "con", "cual", "cuál", "cuales", "cuáles", "cuando",
            "cuándo", "cuanto", "cuánto", "cuántos", "cuántas", "de", "debe", "del", "donde",
            "dónde", "el", "en", "es", "esta", "este", "hay", "la", "las", "lo", "los", "mi",
            "mis", "o", "para", "por", "porque", "qué", "que", "quién", "se", "ser", "si", "son",
            "su", "sus", "tu", "un", "una", "uno", "y", "yo",
        ],
    ),
    (
        "it",
        &[
            "a", "al", "alla", "che", "chi", "come", "con", "cosa", "da", "dei", "del", "della",
            "di", "dove", "e", "è", "gli", "i", "il", "in", "l", "la", "le", "lo", "ma", "mi",
            "nel", "nella", "o", "per", "perché", "può", "quale", "quali", "quando", "quanti",
            "quanto", "se", "si", "sono", "su", "sul", "tu", "un", "una", "uno",
        ],
    ),
    (
        "pt",
        &[
            "a", "ao", "as", "com", "como", "da", "das", "de", "deve", "do", "dos", "e", "é", "em",
            "na", "nas", "no", "nos", "o", "onde", "os", "ou", "para", "pode", "por", "porque",
            "quais", "qual", "quando", "quanto", "quantos", "que", "quem", "se", "ser", "seu",
            "sua", "um", "uma",
        ],
    ),
    (
        "nl",
        &[
            "aan", "als", "bij", "de", "die", "dit", "een", "en", "hoe", "hoeveel", "ik", "in",
            "is", "je", "kan", "met", "moet", "naar", "of", "om", "op", "te", "van", "voor",
            "waar", "waarom", "wanneer", "wat", "welke", "wie", "wordt", "zijn",
        ],
    ),
];

/// The filler words of `language`, such as `de`, or none for a language
/// this list does not hold, whose words are then all matched.
pub(crate) fn filler(language: &str) -> &'static [&'static str] {
    FILLER
        .iter()
        .find(|(code, _)| *code == language)
        .map_or(&[], |(_, words)| *words)
}
