use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "score_stat")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(version)]
    pub version: i64,
    #[sea_orm(create_time)]
    pub create_time: DateTime,
    #[sea_orm(update_time)]
    pub update_time: Option<DateTime>,

    pub creator_id: Option<i64>,
    pub updater_id: Option<i64>,
    #[sea_orm(default_value = 0)]
    pub del_flag: i16,

    pub scope: String,
    pub span: String,
    pub span_start_time: DateTime,
    pub span_end_time: DateTime,
    pub user_id: i64,
    pub content: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}