use crate::entities::{prelude::*, *};
use sea_orm::*;

pub async fn delete_chef_by_id(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
  Chef::delete_by_id(id).exec(db).await
}

pub async fn delete_chef_by_name(
  db: &DatabaseConnection,
  name: &str,
) -> Result<DeleteResult, DbErr> {
  Chef::delete_many()
    .filter(chef::Column::Name.eq(name))
    .exec(db)
    .await
}

pub async fn delete_all_chefs(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
  Chef::delete_many()
    .filter(chef::Column::Id.gte(0))
    .exec(db)
    .await
}

pub async fn delete_bakery_by_id(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
  Bakery::delete_by_id(id).exec(db).await
}

pub async fn delete_bakery_by_name(
  db: &DatabaseConnection,
  name: &str,
) -> Result<DeleteResult, DbErr> {
  Bakery::delete_many()
    .filter(bakery::Column::Name.eq(name))
    .exec(db)
    .await
}

pub async fn delete_all_bakeries(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
  Bakery::delete_many()
    .filter(bakery::Column::Id.gte(0))
    .exec(db)
    .await
}
