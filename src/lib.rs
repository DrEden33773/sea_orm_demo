pub mod entities;
pub mod handler;
pub mod opt;
pub mod router;
pub mod test_info;

use dotenvy::dotenv;
use opt::delete::{delete_all_bakeries, delete_all_chefs};
use sea_orm::*;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let db = Database::connect(&db_url).await?;
  Ok(db)
}

pub async fn establish_connection_then_delete_all() -> Result<DatabaseConnection, DbErr> {
  let db = establish_connection().await?;
  delete_all_chefs(&db).await?;
  delete_all_bakeries(&db).await?;
  Ok(db)
}

#[cfg(test)]
mod basic_crud {
  use super::*;
  use entities::{prelude::*, *};
  use opt::{delete::*, find::*, insert::*, update::*};
  use test_info::{bakery_info::*, chef_info::*};
  use tokio::sync::Mutex;

  static DB_MUTEX: Mutex<()> = Mutex::const_new(());

  async fn close(db: DatabaseConnection) -> Result<(), DbErr> {
    db.close().await
  }

  #[tokio::test]
  async fn insert_and_update() -> Result<(), DbErr> {
    let mut db = establish_connection().await?;

    {
      let _lock = DB_MUTEX.lock().await;
      delete_chef_by_id(&db, JOHN.id).await?;
      delete_bakery_by_id(&db, BAKERY.id).await?;
    }

    {
      let _lock = DB_MUTEX.lock().await;
      insert_bakery_with_explicit_id(&db, BAKERY.id, BAKERY.name, BAKERY.profit_margin).await?;
      insert_chef_with_explicit_id(&db, JOHN.id, JOHN.name, JOHN.bakery_id).await?;
    }

    {
      let _lock = DB_MUTEX.lock().await;
      let updated_chef = update_chef(&mut db, JOHN.id, Some("John"), None, None).await?;
      let updated_bakery = update_bakery(&mut db, BAKERY.id, Some("Happy Bakery"), None).await?;
      assert_eq!(updated_bakery.name, "Happy Bakery");
      assert_eq!(updated_chef.name, "John");
    }

    close(db).await
  }

  async fn find(db: &DatabaseConnection) -> Result<(), DbErr> {
    let jolie = find_chef_by_name(db, JOLIE.name).await?.unwrap();
    let charles = find_chef_by_name(db, CHARLES.name).await?.unwrap();
    let madeleine = find_chef_by_name(db, MADELEINE.name).await?.unwrap();
    let frederic = find_chef_by_name(db, FREDERIC.name).await?.unwrap();

    assert_eq!(jolie.name, JOLIE.name);
    assert_eq!(charles.name, CHARLES.name);
    assert_eq!(madeleine.name, MADELEINE.name);
    assert_eq!(frederic.name, FREDERIC.name);

    let jolie = find_chef_by_id(db, jolie.id).await?.unwrap();
    let charles = find_chef_by_id(db, charles.id).await?.unwrap();
    let madeleine = find_chef_by_id(db, madeleine.id).await?.unwrap();
    let frederic = find_chef_by_id(db, frederic.id).await?.unwrap();

    assert_eq!(jolie.name, JOLIE.name);
    assert_eq!(charles.name, CHARLES.name);
    assert_eq!(madeleine.name, MADELEINE.name);
    assert_eq!(frederic.name, FREDERIC.name);

    Ok(())
  }

  async fn relational_select(db: &DatabaseConnection) -> Result<(), DbErr> {
    let la_boulangerie = find_bakery_by_id(db, LA_BOULANGERIE.id).await?.unwrap();
    let mut chefs = la_boulangerie.find_related(Chef).all(db).await?;
    chefs.sort_unstable_by_key(|chef| chef.name.clone());

    let mut directly_find_chefs = Chef::find()
      .filter(chef::Column::BakeryId.eq(LA_BOULANGERIE.id))
      .all(db)
      .await?;
    directly_find_chefs.sort_unstable_by_key(|chef| chef.name.clone());

    assert_eq!(chefs.len(), 4);
    assert_eq!(directly_find_chefs.len(), 4);
    assert_eq!(chefs, directly_find_chefs);

    Ok(())
  }

  #[tokio::test]
  async fn loader() -> Result<(), DbErr> {
    let db = establish_connection().await?;

    {
      let _lock = DB_MUTEX.lock().await;
      for ChefInfo { id, .. } in [
        JOLIE, CHARLES, MADELEINE, FREDERIC, BRIAN, JERRY, KATE, SAMANTHA,
      ] {
        delete_chef_by_id(&db, id).await?;
      }
      for BakeryInfo { id, .. } in [LA_BOULANGERIE, ARTE_BY_PADARIA] {
        delete_bakery_by_id(&db, id).await?;
      }
    }

    {
      let _lock = DB_MUTEX.lock().await;
      insert_bakery_with_explicit_id(
        &db,
        LA_BOULANGERIE.id,
        LA_BOULANGERIE.name,
        LA_BOULANGERIE.profit_margin,
      )
      .await?;
      insert_bakery_with_explicit_id(
        &db,
        ARTE_BY_PADARIA.id,
        ARTE_BY_PADARIA.name,
        ARTE_BY_PADARIA.profit_margin,
      )
      .await?;

      let la_chefs = [JOLIE, CHARLES, MADELEINE, FREDERIC];
      for ChefInfo { id, name, .. } in &la_chefs {
        insert_chef_with_explicit_id(&db, *id, name, LA_BOULANGERIE.id).await?;
      }
      let arte_chefs = [BRIAN, JERRY, KATE, SAMANTHA];
      for ChefInfo { id, name, .. } in &arte_chefs {
        insert_chef_with_explicit_id(&db, *id, name, ARTE_BY_PADARIA.id).await?;
      }
    }

    find(&db).await?;
    relational_select(&db).await?;

    let bakeries = Bakery::find()
      .filter(
        Condition::any()
          .add(bakery::Column::Id.eq(LA_BOULANGERIE.id))
          .add(bakery::Column::Id.eq(ARTE_BY_PADARIA.id)),
      )
      .all(&db)
      .await?;

    let chefs = bakeries.load_many(Chef, &db).await?;
    let mut la_chef_names = chefs[0]
      .iter()
      .map(|chef| chef.name.to_owned())
      .collect::<Vec<_>>();
    let mut arte_chef_names = chefs[1]
      .iter()
      .map(|chef| chef.name.to_owned())
      .collect::<Vec<_>>();
    la_chef_names.sort_unstable();
    arte_chef_names.sort_unstable();

    assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
    assert_eq!(arte_chef_names, ["Brian", "Jerry", "Kate", "Samantha"]);

    close(db).await
  }
}
