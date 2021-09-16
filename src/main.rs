extern crate dotenv;
extern crate inflector;

use dotenv::dotenv;
use std::env;
use std::error;
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

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let mut client = database::connect(get_database_configuration()?)?;

    let path = Path::new("types.d.ts");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let tables: Vec<database::Table>;
    match database::fetch_table_definitions(&mut client) {
        Ok(result) => tables = result,
        Err(why) => panic!("Couldn't get tables {}", why),
    }

    for table in tables.iter() {
        file.write(typing_generator::generate_typing_from_table(table).as_bytes())?;
    }

    println!("Finished!");
    Ok(())
}
