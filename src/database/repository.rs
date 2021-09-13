use super::Column;
use postgres::{Client, Error};

pub fn get_table_names(client: &mut Client) -> Result<Vec<String>, Error> {
  let mut table_names: Vec<String> = Vec::new();
  for row in client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name NOT LIKE 'knex_%' ORDER BY table_name asc", &[])? {
      let table_name: &str = row.get(0);
      table_names.push(table_name.into());
  }
  Ok(table_names)
}

pub fn get_columns(client: &mut Client, table_name: &String) -> Result<Vec<Column>, Error> {
  let mut columns: Vec<Column> = Vec::new();
  for row in client.query("SELECT column_name, is_nullable, udt_name FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position asc", &[&table_name])? {
      let column_name: &str = row.get(0);
      let is_nullable: &str = row.get(1);
      let udt: &str = row.get(2);

      columns.push(Column{ name: column_name.into(), is_nullable: is_nullable == "YES", udt: udt.into() });
  }
  Ok(columns)
}
