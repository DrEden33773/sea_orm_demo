use crate::entities::*;
use sea_orm::prelude::*;
use sea_orm::*;

pub async fn update_bakery(
  db: &mut DatabaseConnection,
  id: i32,
  name: Option<&str>,
  profit_margin: Option<f64>,
) -> Result<bakery::Model, DbErr> {
  let mut to_update = bakery::ActiveModel {
    id: Set(id),
    ..Default::default()
  };
  if let Some(name) = name {
    to_update.name = Set(name.to_owned());
  }
  if let Some(profit_margin) = profit_margin {
    to_update.profit_margin = Set(profit_margin);
  }

  to_update.update(db).await
}

pub async fn update_chef(
  db: &mut DatabaseConnection,
  id: i32,
  name: Option<&str>,
  contact_details: Option<Option<Json>>,
  bakery_id: Option<i32>,
) -> Result<chef::Model, DbErr> {
  let mut to_update = chef::ActiveModel {
    id: Set(id),
    ..Default::default()
  };
  if let Some(name) = name {
    to_update.name = Set(name.to_owned());
  }
  if let Some(contact_details) = contact_details {
    to_update.contact_details = Set(contact_details);
  }
  if let Some(bakery_id) = bakery_id {
    to_update.bakery_id = Set(bakery_id);
  }

  to_update.update(db).await
}
