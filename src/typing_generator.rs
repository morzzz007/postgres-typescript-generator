use crate::database;

pub enum Source<'a> {
  DatabaseTable(&'a database::Table),
}

mod table;

pub fn generate_typing(source: Source) -> String {
  match source {
    Source::DatabaseTable(table) => table::generate(table),
  }
}
