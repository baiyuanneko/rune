//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "media_cover_art")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub file_hash: String,
    #[sea_orm(column_type = "Blob")]
    pub binary: Vec<u8>,
    pub primary_color: Option<i32>,
    pub hlc_uuid: String,
    #[sea_orm(column_type = "Text")]
    pub created_at_hlc_ts: String,
    pub created_at_hlc_ver: i32,
    #[sea_orm(column_type = "Text")]
    pub created_at_hlc_nid: String,
    #[sea_orm(column_type = "Text")]
    pub updated_at_hlc_ts: String,
    pub updated_at_hlc_ver: i32,
    #[sea_orm(column_type = "Text")]
    pub updated_at_hlc_nid: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::media_files::Entity")]
    MediaFiles,
}

impl Related<super::media_files::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaFiles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
