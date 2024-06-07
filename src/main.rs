use rocket::{serde::json::Json, *};
use sea_orm::*;
use sea_orm_demo::{establish_connection, opt::find::find_all_bakeries};

#[get("/")]
async fn index() -> &'static str {
  "Hello, bakeries!"
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
  let db = db as &DatabaseConnection;

  let bakery_names = find_all_bakeries(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?
    .into_iter()
    .map(|b| b.name)
    .collect::<Vec<String>>();

  Ok(Json(bakery_names))
}

#[launch]
async fn rocket() -> _ {
  let db = match establish_connection().await {
    Ok(db) => db,
    Err(e) => panic!("{e}"),
  };

  rocket::build()
    .manage(db)
    .mount("/", routes![index, bakeries])
}
#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
  message: String,
}
impl From<DbErr> for ErrorResponder {
  fn from(err: DbErr) -> ErrorResponder {
    ErrorResponder {
      message: err.to_string(),
    }
  }
}
impl From<String> for ErrorResponder {
  fn from(string: String) -> ErrorResponder {
    ErrorResponder { message: string }
  }
}
impl From<&str> for ErrorResponder {
  fn from(str: &str) -> ErrorResponder {
    str.to_owned().into()
  }
}
