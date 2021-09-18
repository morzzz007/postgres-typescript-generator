extern crate dotenv;
extern crate inflector;

use dotenv::dotenv;
use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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

fn read_extra_typings() -> Option<String> {
    let filename = "psql-typings.toml";
    if !Path::new(filename).exists() {
        return None;
    }

    return Some(fs::read_to_string(filename).unwrap());
}

fn write_extra_typings_to_file(mut file: &File) {
    let extra_typings = read_extra_typings();
    match extra_typings {
        Some(extra_typings) => {
            let decoded: typing_generator::TomlHashMap = toml::from_str(&extra_typings).unwrap();
            file.write(
                typing_generator::generate_typing(typing_generator::Source::TomlHashMap(decoded))
                    .as_bytes(),
            )
            .unwrap();
        }
        None => println!("No extra typings info found..."),
    }
}

fn write_database_typings_to_file(mut file: &File) {
    let mut client = database::connect(get_database_configuration().unwrap()).unwrap();

    let tables: Vec<database::Table>;
    match database::fetch_table_definitions(&mut client) {
        Ok(result) => tables = result,
        Err(why) => panic!("Couldn't get tables {}", why),
    }

    for table in tables.iter() {
        file.write(
            typing_generator::generate_typing(typing_generator::Source::DatabaseTable(table))
                .as_bytes(),
        )
        .unwrap();
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let path = Path::new("types.d.ts");
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    write_extra_typings_to_file(&file);
    write_database_typings_to_file(&file);

    println!("Finished!");
    Ok(())
}
