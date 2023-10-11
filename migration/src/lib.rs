pub use sea_orm_migration::prelude::*;

mod m20231010_000001_create_user_table;
mod m20231010_000002_create_post_table;
mod m20231010_000003_add_kind_col_to_post_table;
mod m20231011_000004_add_password_col_to_user_table;
mod m20231011_000005_add_unique_name_col_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20231010_000001_create_user_table::Migration),
      Box::new(m20231010_000002_create_post_table::Migration),
      Box::new(m20231010_000003_add_kind_col_to_post_table::Migration),
      Box::new(m20231011_000004_add_password_col_to_user_table::Migration),
      Box::new(m20231011_000005_add_unique_name_col_user_table::Migration),
    ]
  }
}
