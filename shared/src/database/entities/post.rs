//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use super::sea_orm_active_enums::Kind;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub title: String,
  pub user_id: i32,
  pub text: String,
  pub kind: Option<Kind>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
