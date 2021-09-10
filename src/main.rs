extern crate dotenv;
extern crate inflector;

use convert_case::{Case, Casing};
use dotenv::dotenv;
use inflector::string::singularize::to_singular;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod database;

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

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    let path = Path::new("types.d.ts");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let tables: Vec<database::Table>;
    match database::get_tables() {
        Ok(result) => tables = result,
        Err(why) => panic!("Couldn't get tables {}", why),
    }

    for table in tables.iter() {
        file.write(
            format!(
                "export type {} = {{\n",
                to_singular(&table.name).to_case(Case::UpperCamel)
            )
            .as_bytes(),
        )?;
        for column in table.columns.iter() {
            file.write(format_column(column).as_bytes())?;
        }
        file.write(format!("}}\n\n").as_bytes())?;
    }

    println!("Finished!");
    Ok(())
}
