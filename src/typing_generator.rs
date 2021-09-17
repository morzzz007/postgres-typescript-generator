use crate::database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod table;
mod toml_value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlHashMap {
  #[serde(flatten)]
  typings: HashMap<String, toml::Value>,
}

pub enum Source<'a> {
  DatabaseTable(&'a database::Table),
  TomlHashMap(TomlHashMap),
}

pub fn generate_typing(source: Source) -> String {
  match source {
    Source::DatabaseTable(table) => table::generate(table),
    Source::TomlHashMap(hash) => toml_value::generate(hash),
  }
}
