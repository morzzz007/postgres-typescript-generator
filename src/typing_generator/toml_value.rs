use crate::typing_generator;
use convert_case::{Case, Casing};
use inflector::string::singularize::to_singular;

pub fn generate(decoded: typing_generator::TomlHashMap) -> String {
  let mut result = String::from("");

  for (table_name, sub_types) in decoded.typings.into_iter() {
    let singular_table_name = to_singular(&table_name).to_case(Case::UpperCamel);

    match sub_types {
      toml::Value::Table(sub_type) => {
        for (name, fields) in sub_type {
          let type_name = name.to_case(Case::UpperCamel);
          let typing_header = format!("export type {}{} = {{\n", singular_table_name, type_name);
          let typing_footer = "}\n\n";

          result.push_str(&typing_header);

          match fields {
            toml::Value::Table(fields) => {
              for (field_name, field_typing) in fields {
                result.push_str(&format!(
                  "  {}: {};\n",
                  field_name,
                  field_typing.as_str().unwrap()
                ))
              }
            }
            _ => {}
          }

          result.push_str(typing_footer);
        }
      }
      _ => {}
    }
  }
  result
}
