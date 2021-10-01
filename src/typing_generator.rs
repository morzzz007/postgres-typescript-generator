use crate::database;
use convert_case::{Case, Casing};
use inflector::string::singularize::to_singular;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod table;
mod toml_value;

pub struct TypingGeneratorResult {
  pub string_value: String,
  pub types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlHashMap {
  #[serde(flatten)]
  typings: HashMap<String, toml::Value>,
}

pub enum Source<'a> {
  DatabaseTable(&'a database::Table),
  TomlHashMap(TomlHashMap),
}

pub fn generate_typing(source: Source, additional_types: &Vec<String>) -> TypingGeneratorResult {
  match source {
    Source::DatabaseTable(table) => table::generate(table, additional_types),
    Source::TomlHashMap(hash) => toml_value::generate(hash),
  }
}

pub fn format_type_class_name(name: &str) -> String {
  to_singular(&name).to_case(Case::UpperCamel)
}

pub fn format_sub_type_class_name(main_type: &str, sub_type: &str) -> String {
  let m_type = format_type_class_name(main_type);
  let s_type = format_type_class_name(sub_type);
  format!("{}{}", m_type, s_type)
}
