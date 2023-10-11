use sea_orm_migration::prelude::*;

use crate::m20231010_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(User::Table)
          .add_column_if_not_exists(
            ColumnDef::new(NewPasswordCol::Ref)
              .string()
              .not_null()
              .default("NONE"),
          )
          .to_owned(),
      )
      .await?;

    manager
      .get_connection()
      .execute_unprepared(
        r#"
            ALTER TABLE IF EXISTS public."user"
                ALTER COLUMN password DROP DEFAULT;
        "#,
      )
      .await?; // remove default

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    print!(
      "{}",
      Table::alter()
        .drop_column(NewPasswordCol::Ref)
        .to_owned()
        .to_string(PostgresQueryBuilder)
    );
    manager
      .alter_table(
        Table::alter()
          .table(User::Table)
          .drop_column(NewPasswordCol::Ref)
          .to_owned(),
      )
      .await
  }
}

#[derive(Iden)]
enum NewPasswordCol {
  #[iden = "password"]
  Ref,
}
