use rocket::*;
use sea_orm_demo::{establish_connection, router::*};

#[launch]
async fn rocket() -> _ {
  let db = match establish_connection().await {
    Ok(db) => db,
    Err(e) => panic!("{e}"),
  };

  rocket::build()
    .manage(db)
    .mount("/", routes![index, bakeries, bakery_by_id, new_bakery])
}
