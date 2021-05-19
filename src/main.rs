#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate tantivy;

use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, LeasedItem, Searcher};
use tantivy::ReloadPolicy;
use tempfile::TempDir;
use rocket::State;
use rocket_contrib::json::Json;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Address {
  line1: String,
  line2: Option<String>,
  city: String,
  state: String,
  zip: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Addresses {
  addresses: Vec<Address>,
}

// TODO: Move the addresses.json file to a zip, and then load it.
// This file will sit in memory, and it'd be nice to move it, but
// then we'd need to change the build script. Not worth it for a
// first pass.
static ADDRESSES: &'static str = include_str!("addresses.json");

const DEFAULT_MAX_RESULTS: usize = 3;

const FIELD_LINE_1: &str = "line1";

const FIELD_LINE_2: &str = "line2";

const FIELD_CITY: &str = "city";

const FIELD_STATE: &str = "state";

const FIELD_ZIP: &str = "zip";

#[get("/addresses?<hint>&<limit>")]
fn addresses(
  hint: String,
  limit: Option<String>,
  query_parser: State<QueryParser>,
  searcher: State<LeasedItem<Searcher>>,
  fields: State<HashMap<&str, Field>>,
) -> Json<Vec<Address>> {
  let query = query_parser.parse_query(&hint).unwrap();

  let results_limit = match limit {
    Some(value) => value.parse().unwrap(),
    None => DEFAULT_MAX_RESULTS
  };

  let top_docs = searcher.search(&query, &TopDocs::with_limit(results_limit)).unwrap();

  let documents: Vec<Document> = top_docs.into_iter().map(
    |(_score, doc_address)|
      searcher.doc(doc_address).unwrap()
  ).collect();

  let addresses: Vec<Address> = documents.into_iter().map(
    |document|
      {
        let line2val = document.get_first(fields[FIELD_LINE_2]);
        let line2string = match line2val {
          Some(val) => String::from(val.text().unwrap().to_string()),
          None => String::from("")
        };

        Address {
          line1: document.get_first(fields[FIELD_LINE_1]).unwrap().text().unwrap().to_string(),
          line2: Option::from(line2string.to_string()),
          city: document.get_first(fields[FIELD_CITY]).unwrap().text().unwrap().to_string(),
          state: document.get_first(fields[FIELD_STATE]).unwrap().text().unwrap().to_string(),
          zip: document.get_first(fields[FIELD_ZIP]).unwrap().text().unwrap().to_string(),
        }
      }
  ).collect();


  Json(addresses)
}

fn main() {
  let index_path = TempDir::new().unwrap();

  let mut schema_builder = Schema::builder();

  schema_builder.add_text_field(FIELD_LINE_1, TEXT | STORED);
  schema_builder.add_text_field(FIELD_LINE_2, TEXT | STORED);
  schema_builder.add_text_field(FIELD_CITY, TEXT | STORED);
  schema_builder.add_text_field(FIELD_STATE, TEXT | STORED);
  schema_builder.add_text_field(FIELD_ZIP, TEXT | STORED);

  let schema = schema_builder.build();

  let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();

  let mut index_writer = index.writer(50_000_000).unwrap();

  let line1field = schema.get_field(FIELD_LINE_1).unwrap();
  let line2field = schema.get_field(FIELD_LINE_2).unwrap();
  let city_field = schema.get_field(FIELD_CITY).unwrap();
  let state_field = schema.get_field(FIELD_STATE).unwrap();
  let zip_field = schema.get_field(FIELD_ZIP).unwrap();

  let addresses: Vec<Address> = serde_json::from_str(ADDRESSES).unwrap();

  // todo: delete all of the addresses once they're read into the inverted index
  for address in addresses {
    let line2value = match &address.line2 {
      Some(val) => String::from(val),
      None => String::from("")
    };

    index_writer.add_document(
      doc!(
                line1field => String::from(&address.line1),
                // todo: figure out how to get an empty string when there's no line 2
                line2field => String::from(line2value),
                city_field => String::from(&address.city),
                state_field => String::from(&address.state),
                zip_field => String::from(&address.zip)
            )
    );
  }

  index_writer.commit().unwrap();

  let reader = index
    .reader_builder()
    .reload_policy(ReloadPolicy::OnCommit)
    .try_into().unwrap();

  let searcher = reader.searcher();

  let query_parser = QueryParser::for_index(
    &index,
    vec![line1field, line2field, city_field, state_field, zip_field],
  );

  let mut fields = HashMap::new();

  fields.insert(
    FIELD_LINE_1,
    line1field,
  );
  fields.insert(
    FIELD_LINE_2,
    line2field,
  );
  fields.insert(
    FIELD_CITY,
    city_field,
  );
  fields.insert(
    FIELD_STATE,
    state_field,
  );
  fields.insert(
    FIELD_ZIP,
    zip_field,
  );

  rocket::ignite()
    .mount("/", routes![addresses])
    .manage(query_parser)
    .manage(searcher)
    .manage(fields)
    .launch();
}