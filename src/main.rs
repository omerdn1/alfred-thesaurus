use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Clone)]
pub struct ThesaurusResponse {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    #[serde(rename = "definitionData")]
    pub definition_data: DefinitionData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefinitionData {
    // #[serde(rename = "entry")]
    // pub entry: String,

    // #[serde(rename = "type")]
    // definition_data_type: String,

    #[serde(rename = "definitions")]
    pub definitions: Vec<Definition>,

    // #[serde(rename = "slug")]
    // slug: String,

    // #[serde(rename = "rawSlug")]
    // raw_slug: String,

    // #[serde(rename = "searchQueries")]
    // search_queries: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Definition {
    // #[serde(rename = "isVulgar")]
    // is_vulgar: String,
    pub definition: String,
    // pub pos: String,
    pub synonyms: Vec<Onym>,
    pub antonyms: Vec<Onym>,
    // #[serde(rename = "note")]
    // note: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Onym {
    // #[serde(rename = "similarity")]
    // pub similarity: String,

    // #[serde(rename = "isInformal")]
    // is_informal: String,

    // #[serde(rename = "isVulgar")]
    // is_vulgar: Option<serde_json::Value>,
    #[serde(rename = "term")]
    pub term: String,

    // #[serde(rename = "targetTerm")]
    // pub target_term: Option<String>,

    // #[serde(rename = "targetSlug")]
    // pub target_slug: Option<String>,
}

const WORD_LOOKUP_ENDPOINT: &'static str = "https://tuna.thesaurus.com/pageData/";

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args(&[
            Arg::from_usage("-s, --synonyms... 'looks up synonyms for given word'"),
            Arg::from_usage("-a, --antonyms... 'looks up antonyms for given word'"),
            Arg::with_name("query").required(true).index(1),
        ])
        .get_matches();
    let query = args.value_of("query").unwrap();

    let mut alfred_items: Vec<alfred::Item> = vec![];

    if let Some(thesaurus) = word_lookup(query) {
        for def in thesaurus.data.definition_data.definitions {
            // alfred_items.push(
            //     alfred::ItemBuilder::new(def.definition.to_owned())
            //         .subtitle("Definition".to_string())
            //         .into_item(),
            // );
            if args.contains_id("synonyms") {
                for s in def.synonyms {
                    alfred_items.push(
                        alfred::ItemBuilder::new(s.term.to_owned())
                            .arg(s.term.to_owned())
                            .into_item(),
                    );
                }
            }

            if args.contains_id("antonyms") {
                for a in def.antonyms {
                    alfred_items.push(
                        alfred::ItemBuilder::new(a.term.to_owned())
                            .arg(a.term.to_owned())
                            .into_item(),
                    );
                }
            }
        }

        alfred::json::Builder::with_items(&alfred_items)
            .write(io::stdout())
            .expect("Couldn't write items to Alfred");

        }
}

fn word_lookup(word: &str) -> Option<ThesaurusResponse> {
    let res: ThesaurusResponse = reqwest::blocking::get(WORD_LOOKUP_ENDPOINT.to_owned() + word)
        .unwrap()
        .json()
        .unwrap();

    Some(res)
}
