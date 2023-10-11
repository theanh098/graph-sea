use sea_orm_migration::{prelude::*, sea_orm::EnumIter, sea_query::extension::postgres::Type};

use crate::m20231010_000002_create_post_table::Post;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_type(
        Type::create()
          .as_enum(Kind::Ref)
          .values([Kind::Feed, Kind::Story, Kind::Unknown])
          .to_owned(),
      )
      .await?;

    manager
      .alter_table(
        Table::alter()
          .table(Post::Table)
          .add_column_if_not_exists(
            ColumnDef::new(NewKindCol::Ref)
              .enumeration(Kind::Ref, [Kind::Feed, Kind::Story, Kind::Unknown])
              .not_null()
              .default("unknown"),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Post::Table)
          .drop_column(NewKindCol::Ref)
          .to_owned(),
      )
      .await?;

    manager
      .drop_type(
        Type::drop()
          .if_exists()
          .name(Kind::Ref)
          .restrict()
          .to_owned(),
      )
      .await
  }
}

#[derive(Iden, EnumIter)]
pub enum Kind {
  #[iden = "kind"]
  Ref,

  #[iden = "feed"]
  Feed,

  #[iden = "story"]
  Story,

  #[iden = "unknown"]
  Unknown,
}

#[derive(Iden)]
pub enum NewKindCol {
  #[iden = "kind"]
  Ref,
}
