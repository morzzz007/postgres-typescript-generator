use postgres::{Client, Error, NoTls};
use std::env;
use std::error;

pub struct Column {
  pub name: String,
  pub is_nullable: bool,
  pub udt: String,
}

pub struct Table {
  pub name: String,
  pub columns: Vec<Column>,
}

pub fn get_tables() -> Result<Vec<Table>, Box<dyn error::Error>> {
  let mut client: Client = connect()?;

  let mut tables: Vec<Table> = Vec::new();
  let tables_vec: Vec<String> = get_table_names(&mut client)?;

  for table in tables_vec.iter() {
    let columns_vec: Vec<Column> = get_columns(&mut client, &table)?;

    tables.push(Table {
      name: table.into(),
      columns: columns_vec,
    })
  }

  Ok(tables)
}

fn connect() -> Result<Client, Box<dyn error::Error>> {
  let user = env::var("POSTGRES_USER")?;
  let password = env::var("POSTGRES_PASSWORD")?;
  let host = env::var("POSTGRES_HOST")?;
  let port = env::var("POSTGRES_PORT")?;
  let database = env::var("POSTGRES_DATABASE")?;

  let client = Client::connect(
    &format!(
      "host={} user={} password={} dbname={} port={}",
      host, user, password, database, port
    ),
    NoTls,
  )?;

  Ok(client)
}

fn get_table_names(client: &mut Client) -> Result<Vec<String>, Error> {
  let mut table_names: Vec<String> = Vec::new();
  for row in client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name NOT LIKE 'knex_%' ORDER BY table_name asc", &[])? {
      let table_name: &str = row.get(0);
      table_names.push(table_name.into());
  }
  Ok(table_names)
}

fn get_columns(client: &mut Client, table_name: &String) -> Result<Vec<Column>, Error> {
  let mut columns: Vec<Column> = Vec::new();
  for row in client.query("SELECT column_name, is_nullable, udt_name FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position asc", &[&table_name])? {
      let column_name: &str = row.get(0);
      let is_nullable: &str = row.get(1);
      let udt: &str = row.get(2);

      columns.push(Column{ name: column_name.into(), is_nullable: is_nullable == "YES", udt: udt.into() });
  }
  Ok(columns)
}
