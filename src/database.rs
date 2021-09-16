use postgres::{Client, Error, NoTls};
use std::error;

mod repository;

pub struct Column {
  pub name: String,
  pub is_nullable: bool,
  pub udt: String,
}

pub struct Table {
  pub name: String,
  pub columns: Vec<Column>,
}

pub struct ClientConfiguration {
  pub user: String,
  pub password: String,
  pub host: String,
  pub port: String,
  pub database: String,
}

pub fn connect(client_configuration: ClientConfiguration) -> Result<Client, Box<dyn error::Error>> {
  let client = Client::connect(
    &format!(
      "host={} user={} password={} dbname={} port={}",
      client_configuration.host,
      client_configuration.user,
      client_configuration.password,
      client_configuration.database,
      client_configuration.port
    ),
    NoTls,
  )?;

  Ok(client)
}

pub fn fetch_table_definitions(client: &mut Client) -> Result<Vec<Table>, Error> {
  let mut tables: Vec<Table> = Vec::new();
  let tables_vec: Vec<String> = repository::get_table_names(client)?;

  for table in tables_vec.iter() {
    let columns_vec: Vec<Column> = repository::get_columns(client, &table)?;

    tables.push(Table {
      name: table.into(),
      columns: columns_vec,
    })
  }

  Ok(tables)
}
