use crate::handler::ErrorResponder;
#[allow(unused_imports)]
use crate::opt::{delete::*, find::*, insert::*, update::*};
use rocket::{serde::json::Json, *};
use sea_orm::*;

#[get("/")]
pub async fn index() -> &'static str {
  "Hello, bakeries!"
}

#[get("/bakeries")]
pub async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
  let db = db as &DatabaseConnection;

  let bakery_names = find_all_bakeries(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?
    .into_iter()
    .map(|b| b.name)
    .collect::<Vec<String>>();

  Ok(Json(bakery_names))
}

#[get("/bakeries/<id>")]
pub async fn bakery_by_id(
  db: &State<DatabaseConnection>,
  id: i32,
) -> Result<String, ErrorResponder> {
  let db = db as &DatabaseConnection;

  let bakery = find_bakery_by_id(db, id)
    .await
    .map_err(Into::<ErrorResponder>::into)?;

  Ok(if let Some(bakery) = bakery {
    bakery.name
  } else {
    return Err(format!("No bakery with id {id} is found.").into());
  })
}

#[post("/bakeries?<name>&<profit_margin>")]
pub async fn new_bakery(
  db: &State<DatabaseConnection>,
  name: &str,
  profit_margin: Option<f64>,
) -> Result<(), ErrorResponder> {
  let db = db as &DatabaseConnection;

  insert_bakery(db, name, profit_margin.unwrap_or_default())
    .await
    .map_err(Into::<ErrorResponder>::into)?;

  Ok(())
}
