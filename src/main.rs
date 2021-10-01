extern crate dotenv;
extern crate inflector;

use dotenv::dotenv;
use std::env;
use std::error;
use std::fs;
use std::path::Path;

mod database;
mod typing_generator;

fn get_database_configuration() -> Result<database::ClientConfiguration, std::env::VarError> {
    let user = env::var("POSTGRES_USER")?;
    let password = env::var("POSTGRES_PASSWORD")?;
    let host = env::var("POSTGRES_HOST")?;
    let port = env::var("POSTGRES_PORT")?;
    let database = env::var("POSTGRES_DATABASE")?;

    Ok(database::ClientConfiguration {
        user,
        password,
        host,
        port,
        database,
    })
}

fn read_additional_typings() -> Option<String> {
    let filename = "psql-typings.toml";
    if !Path::new(filename).exists() {
        return None;
    }

    return Some(fs::read_to_string(filename).unwrap());
}

fn write_additional_typings_to_file(output: &mut String) -> Vec<String> {
    let additional_typings = read_additional_typings();
    match additional_typings {
        Some(additional_typings) => {
            let decoded: typing_generator::TomlHashMap =
                toml::from_str(&additional_typings).unwrap();
            let generated_typings = typing_generator::generate_typing(
                typing_generator::Source::TomlHashMap(decoded),
                &Vec::new(),
            );

            output.push_str(&generated_typings.string_value);
            generated_typings.types
        }
        None => Vec::new(),
    }
}

fn write_database_typings_to_file(output: &mut String, additional_types: &Vec<String>) {
    let mut client = database::connect(get_database_configuration().unwrap()).unwrap();

    let tables: Vec<database::Table>;
    match database::fetch_table_definitions(&mut client) {
        Ok(result) => tables = result,
        Err(why) => panic!("Couldn't get tables {}", why),
    }

    let mut iterator = tables.iter().peekable();

    while let Some(table) = iterator.next() {
        output.push_str(
            &typing_generator::generate_typing(
                typing_generator::Source::DatabaseTable(table),
                additional_types,
            )
            .string_value,
        );

        match iterator.peek() {
            Some(_) => {
                output.push_str("\n\n");
            }
            None => {
                output.push_str("\n");
            }
        }

        iterator.next();
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let mut output: String = "".to_owned();

    let additional_types = write_additional_typings_to_file(&mut output);
    write_database_typings_to_file(&mut output, &additional_types);

    print!("{}", output);
    Ok(())
}
