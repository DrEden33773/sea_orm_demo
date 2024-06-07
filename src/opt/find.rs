use crate::entities::{prelude::*, *};
use sea_orm::*;

pub async fn find_all_bakeries(db: &DatabaseConnection) -> Result<Vec<bakery::Model>, DbErr> {
  Bakery::find().all(db).await
}

pub async fn find_bakery_by_id(
  db: &DatabaseConnection,
  id: i32,
) -> Result<Option<bakery::Model>, DbErr> {
  Bakery::find_by_id(id).one(db).await
}

pub async fn find_bakery_by_name(
  db: &DatabaseConnection,
  name: &str,
) -> Result<Option<bakery::Model>, DbErr> {
  Bakery::find()
    .filter(bakery::Column::Name.eq(name))
    .one(db)
    .await
}

pub async fn find_all_chefs(db: &DatabaseConnection) -> Result<Vec<chef::Model>, DbErr> {
  Chef::find().all(db).await
}

pub async fn find_chef_by_id(
  db: &DatabaseConnection,
  id: i32,
) -> Result<Option<chef::Model>, DbErr> {
  Chef::find_by_id(id).one(db).await
}

pub async fn find_chef_by_name(
  db: &DatabaseConnection,
  name: &str,
) -> Result<Option<chef::Model>, DbErr> {
  Chef::find()
    .filter(chef::Column::Name.eq(name))
    .one(db)
    .await
}
