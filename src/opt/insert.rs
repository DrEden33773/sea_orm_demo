use crate::entities::{prelude::*, *};
use sea_orm::*;

pub async fn insert_bakery(
  db: &mut DatabaseConnection,
  name: &str,
  profit_margin: f64,
) -> Result<InsertResult<bakery::ActiveModel>, DbErr> {
  let new_bakery = bakery::ActiveModel {
    name: Set(name.to_owned()),
    profit_margin: Set(profit_margin),
    ..Default::default()
  };

  Bakery::insert(new_bakery).exec(db).await
}

pub async fn insert_bakery_with_explicit_id(
  db: &mut DatabaseConnection,
  id: i32,
  name: &str,
  profit_margin: f64,
) -> Result<InsertResult<bakery::ActiveModel>, DbErr> {
  let new_bakery = bakery::ActiveModel {
    id: Set(id),
    name: Set(name.to_owned()),
    profit_margin: Set(profit_margin),
  };

  Bakery::insert(new_bakery).exec(db).await
}

pub async fn insert_chef(
  db: &mut DatabaseConnection,
  name: &str,
  bakery_id: i32,
) -> Result<InsertResult<chef::ActiveModel>, DbErr> {
  let new_chef = chef::ActiveModel {
    name: Set(name.to_owned()),
    bakery_id: Set(bakery_id),
    ..Default::default()
  };

  Chef::insert(new_chef).exec(db).await
}

pub async fn insert_chef_with_explicit_id(
  db: &mut DatabaseConnection,
  id: i32,
  name: &str,
  bakery_id: i32,
) -> Result<InsertResult<chef::ActiveModel>, DbErr> {
  let new_chef = chef::ActiveModel {
    id: Set(id),
    name: Set(name.to_owned()),
    bakery_id: Set(bakery_id),
    ..Default::default()
  };

  Chef::insert(new_chef).exec(db).await
}
