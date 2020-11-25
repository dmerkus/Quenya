use regex::Regex;

#[derive(Debug, PartialEq)]
enum Part {
    Adjective,
    Adverb,
    Noun,
    Verb,
}

#[derive(Debug, PartialEq)]
struct Lemma<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    lemma: &'a str,
}

#[derive(Debug, PartialEq)]
struct Definition<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    sid: i8,
    definition: &'a str,
}

#[derive(Debug, PartialEq)]
struct Example<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    example: &'a str,
}

#[derive(Debug, PartialEq)]
enum Entry<'a> {
    Definition(Definition<'a>),
    Example(Example<'a>),
    Lemma(Lemma<'a>),
}

fn parse_multilingual_wordnet_line(line: &str) -> Entry {
    let regex = Regex::new(r"^(?P<offset>\d{8})-(?P<pos>[nvars])\s(?P<language>\w{3}):(?P<type>lemma|def|exe)\s((?P<sid>\d{1})\s)?(?P<content>.+)\s*$").unwrap();

    let captures = regex.captures(line).unwrap();

    let language = captures.name("language").unwrap().as_str();
    let offset = captures.name("offset").unwrap().as_str();
    let pos = match &captures["pos"] {
        "a" => Part::Adjective,
        "n" => Part::Noun,
        "r" => Part::Adverb,
        "v" => Part::Verb,
        _ => panic!()
    };
    let content = captures.name("content").unwrap().as_str();

    let line_type = &captures["type"];

    if line_type == "lemma" {
        Entry::Lemma(Lemma {
            language,
            offset,
            pos,
            lemma: content,
        })
    } else if line_type == "def" {
        Entry::Definition(Definition {
            language,
            offset,
            pos,
            sid: captures.name("sid").unwrap().as_str().parse::<i8>().unwrap(),
            definition: content,
        })
    } else if line_type == "exe" {
        Entry::Example(Example {
            language,
            offset,
            pos,
            example: content,
        })
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_indonesian_lemma() {
        let line = "00018158-v	ind:lemma	membubung\n";

        let lemma = parse_multilingual_wordnet_line(line);

        assert_eq!(lemma, Entry::Lemma(Lemma {
            lemma: "membubung",
            language: "ind",
            offset: "00018158",
            pos: Part::Verb,
        }))
    }

    #[test]
    fn it_extract_indonesian_definition() {
        let line = "00006024-n	ind:def	0	organisme yang tergantung pada zat organik kompleks untuk gizi\n";

        let definition = parse_multilingual_wordnet_line(line);

        assert_eq!(definition, Entry::Definition(Definition {
            sid: 0,
            definition: "organisme yang tergantung pada zat organik kompleks untuk gizi",
            language: "ind",
            pos: Part::Noun,
            offset: "00006024",
        }))
    }
}