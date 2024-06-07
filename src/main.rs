use rocket::{serde::json::Json, *};
use sea_orm::*;
use sea_orm_demo::{establish_connection, opt::find::find_all_bakeries};

#[get("/")]
async fn index() -> &'static str {
  "Hello, bakeries!"
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
  let db = db as &DatabaseConnection;

  let bakery_names = find_all_bakeries(db)
    .await
    .unwrap()
    .into_iter()
    .map(|b| b.name)
    .collect::<Vec<_>>();

  Json(bakery_names)
}

#[launch] // The "main" function of the program
async fn rocket() -> _ {
  let db = match establish_connection().await {
    Ok(db) => db,
    Err(e) => panic!("{e}"),
  };

  rocket::build()
    .manage(db)
    .mount("/", routes![index, bakeries])
}
