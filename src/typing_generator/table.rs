use crate::database;
use crate::typing_generator;

use convert_case::{Case, Casing};

fn get_column_type(column: &database::Column, additional_types: &Vec<String>) -> String {
  if additional_types.contains(&column.path) {
    let existing_type = typing_generator::format_sub_type_class_name(&column.table, &column.name);

    if column.column_default.starts_with("'[]'::jsonb") {
      format!("Array<{}>", existing_type)
    } else {
      existing_type
    }
  } else {
    match column.udt.as_str() {
      "bool" => return String::from("boolean"),
      "text" | "citext" | "money" | "numeric" | "int8" | "char" | "character" | "bpchar"
      | "varchar" | "time" | "tsquery" | "tsvector" | "uuid" | "xml" | "cidr" | "inet"
      | "macaddr" => return String::from("string"),
      "smallint" | "integer" | "int" | "int4" | "real" | "float" | "float4" | "float8" => {
        return String::from("number")
      }
      "date" | "timestamp" | "timestamptz" => return String::from("Date"),
      &_ => return String::from("unknown"),
    }
  }
}

fn format_column(column: &database::Column, additional_types: &Vec<String>) -> String {
  let column_type = get_column_type(column, additional_types);
  let nullable = if column.is_nullable { " | null" } else { "" };
  format!(
    "  {}: {}{};\n",
    column.name.to_case(Case::Camel),
    column_type,
    nullable
  )
}

pub fn generate(
  table: &database::Table,
  additonal_types: &Vec<String>,
) -> typing_generator::TypingGeneratorResult {
  let type_class = typing_generator::format_type_class_name(&table.name);
  let typing_header = format!("export type {} = {{\n", type_class);
  let typing_footer = "}\n\n";
  let mut typing: String = typing_header.to_owned();
  for column in table.columns.iter() {
    typing.push_str(&format_column(column, additonal_types))
  }
  typing.push_str(typing_footer);

  typing_generator::TypingGeneratorResult {
    string_value: typing,
    types: Vec::new(),
  }
}
