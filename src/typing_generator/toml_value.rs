use crate::typing_generator;

pub fn generate(decoded: typing_generator::TomlHashMap) -> typing_generator::TypingGeneratorResult {
  let mut result = String::from("");
  let mut generated_types: Vec<String> = Vec::new();

  for (table_name, sub_types) in decoded.typings.into_iter() {
    match sub_types {
      toml::Value::Table(sub_type) => {
        for (name, fields) in sub_type {
          let type_class = typing_generator::format_sub_type_class_name(&table_name, &name);
          let typing_header = format!("export type {} = {{\n", type_class);
          let typing_footer = "}\n\n";

          result.push_str(&typing_header);

          match fields {
            toml::Value::Table(fields) => {
              for (field_name, field_typing) in fields {
                result.push_str(&format!(
                  "  {}: {};\n",
                  field_name,
                  field_typing.as_str().unwrap()
                ));
              }
            }
            _ => {}
          }
          generated_types.push(format!("{}.{}", table_name, name));
          result.push_str(typing_footer);
        }
      }
      _ => {}
    }
  }

  typing_generator::TypingGeneratorResult {
    string_value: result,
    types: generated_types,
  }
}
