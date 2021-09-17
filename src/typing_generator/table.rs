use crate::database;
use convert_case::{Case, Casing};
use inflector::string::singularize::to_singular;

fn get_column_type(column: &database::Column) -> &str {
  match column.udt.as_str() {
    "bool" => return "boolean",
    "text" | "citext" | "money" | "numeric" | "int8" | "char" | "character" | "bpchar"
    | "varchar" | "time" | "tsquery" | "tsvector" | "uuid" | "xml" | "cidr" | "inet"
    | "macaddr" => return "string",
    "smallint" | "integer" | "int" | "int4" | "real" | "float" | "float4" | "float8" => {
      return "number"
    }
    "date" | "timestamp" | "timestamptz" => return "Date",
    "json" | "jsonb" => return "unknown",
    &_ => return "unknown",
  }
}

fn format_column(column: &database::Column) -> String {
  let column_type = get_column_type(column);
  let nullable = if column.is_nullable { " | null" } else { "" };
  format!(
    "  {}: {}{};\n",
    column.name.to_case(Case::Camel),
    column_type,
    nullable
  )
}

pub fn generate(table: &database::Table) -> String {
  let singular_table_name = to_singular(&table.name).to_case(Case::UpperCamel);
  let typing_header = format!("export type {} = {{\n", singular_table_name);
  let typing_footer = "}\n\n";
  let mut typing: String = typing_header.to_owned();
  for column in table.columns.iter() {
    typing.push_str(&format_column(column))
  }
  typing.push_str(typing_footer);
  typing
}
