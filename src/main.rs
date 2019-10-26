use parse_wiktionary_de::Flowing;
use rayon::prelude::*;
use std::io::Write;

fn main() {
    let file = std::fs::File::open("data/dewiktionary-20191020-pages-articles.xml.bz2").unwrap();
    let file = std::io::BufReader::new(file);
    let file = bzip2::bufread::BzDecoder::new(file);
    let file = std::io::BufReader::new(file);
    // let pb = std::sync::Mutex::new(pbr::ProgressBar::on(std::io::stderr(), 1545237));

    /*let all: Vec<_> = */parse_mediawiki_dump::parse(file)
        .par_bridge()
        .filter_map(|result| {
            match result {
                Err(error) => {
                    eprintln!("Error: {}", error);
                    return None;
                }
                Ok(page) => {
                    if page.namespace == 0
                        && match &page.format {
                            None => false,
                            Some(format) => format == "text/x-wiki",
                        }
                        && match &page.model {
                            None => false,
                            Some(model) => model == "wikitext",
                        }
                    {
                        /*println!(
                            "The page {title:?} is an ordinary article with byte length {length}.",
                            title = page.title,
                            length = page.text.len()
                        );*/
                        if let Some(p) = parse_single_article(&page.title, &page.text) {
                            let mut x: Vec<_> = p
                                .pos_entries
                                .iter()
                                .flat_map(|p| p.ipa.iter())
                                .filter_map(|p| match p {
                                    Flowing::Ipa { ipa } => Some(ipa.to_owned()),
                                    _ => None,
                                })
                                .collect();
                            x.sort();
                            x.dedup();
                            if x.len() > 0 {
                                return Some(format!("{}: {}", page.title, x.join(", ")));
                            }
                        }
                    } else {
                        //println!("The page {:?} has something special to it.", page.title);
                    }
                    return None;
                }
            }
        })
        .for_each(|e| println!("{}", e));
        /*
        .map(|e| {
            pb.lock().unwrap().inc();
            e
        })
        .collect();
        println!("writing!");*/
    /*let stdo = std::io::stdout();
    let mut x = stdo.lock();
    for v in all {
        x.write(v.as_bytes()).unwrap();
        x.write(b"\n").unwrap();
    }*/
}

fn parse_single_article<'a>(
    title: &str,
    wiki_text: &'a str,
) -> Option<parse_wiktionary_de::LanguageEntry<'a>> {
    let result = parse_wiktionary_de::create_configuration().parse(wiki_text);
    /*if !result.warnings.is_empty() {
        eprintln!("Parse Wiki Text warnings: {:#?}", result.warnings);
    }*/
    let result = parse_wiktionary_de::parse(&title, &wiki_text, &result.nodes);
    /*println!("{:#?}", result);
    for warning in result.warnings {
        let mut warning_start = warning.start;
        while !wiki_text.is_char_boundary(warning_start) {
            warning_start -= 1;
        }
        let mut warning_end = warning.end;
        while !wiki_text.is_char_boundary(warning_end) {
            warning_end += 1;
        }
        let mut lines_remaining_start = 3;
        let mut snippet_start = warning_start;
        while snippet_start > 0 {
            if wiki_text.as_bytes()[snippet_start - 1] == b'\n' {
                if lines_remaining_start == 0 {
                    break;
                }
                lines_remaining_start -= 1;
            }
            snippet_start -= 1;
        }
        let mut lines_remaining_end = 3;
        let mut snippet_end = warning_end;
        while snippet_end < wiki_text.len() {
            if wiki_text.as_bytes()[snippet_end] == b'\n' {
                if lines_remaining_end == 0 {
                    break;
                }
                lines_remaining_end -= 1;
            }
            snippet_end += 1;
        }
        println!(
            "\n\x1b[9{color}m\x1b[1mwarning\x1b[m / \x1b[97mstart: {start}\x1b[m / \x1b[97mend: {end}\x1b[m / \x1b[97mlanguage: {language:?}\x1b[m / \x1b[97mmessage: {message:?}\x1b[m\n{snippet_start}\x1b[9{color}m{snippet_warning}\x1b[m{snippet_end}",
            color = if warning.message == parse_wiktionary_de::WarningMessage::Supplementary {
                '3'
            } else {
                '1'
            },
            start = warning.start,
            end = warning.end,
            language = warning.language,
            message = warning.message,
            snippet_start = &wiki_text[snippet_start..warning_start],
            snippet_warning = &wiki_text[warning_start..warning_end],
            snippet_end = &wiki_text[warning_end..snippet_end]
        );
    }*/
    let ents: Vec<_> = result
        .language_entries
        .into_iter()
        .filter(|p| p.language == parse_wiktionary_de::Language::De)
        .collect();
    //if ents.len() > 2 {
    //    panic!("multiple entries?")
    //}
    let x = ents.into_iter().next();
    return x;
}
