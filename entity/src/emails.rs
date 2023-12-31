//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::EmailCategory;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "emails")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category: EmailCategory,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub subject: String,
    pub content: String,
    pub date_time: NaiveDateTime,
    pub deleted_by_sender: bool,
    pub deleted_by_recipient: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::RecipientId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users2,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::SenderId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users1,
}

impl ActiveModelBehavior for ActiveModel {}
