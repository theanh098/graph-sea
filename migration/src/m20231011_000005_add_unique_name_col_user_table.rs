use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .get_connection()
      .execute_unprepared(
        r#"
            ALTER TABLE public."user" 
                ADD CONSTRAINT unique_user_name UNIQUE (name);
        "#,
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .get_connection()
      .execute_unprepared(
        r#"
            ALTER TABLE public."user"
                DROP CONSTRAINT unique_user_name;
      "#,
      )
      .await?;

    Ok(())
  }
}
