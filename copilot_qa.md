# Copilot Q/A

## Q1

Here's my code-snippet:

```rust
#[cfg(test)]
mod basic_crud {
  use super::*;
  use sea_orm_demo::entities::{prelude::*, *};
  use std::collections::HashSet;

  async fn delete_all_then_close(db: DatabaseConnection) -> Result<(), DbErr> {
    delete_all_chefs(&db).await?;
    delete_all_bakeries(&db).await?;
    db.close().await
  }

  #[tokio::test]
  async fn insert_and_update() -> Result<(), DbErr> {
    let mut db = establish_connection().await?;

    let bakery = insert_bakery(&mut db, "Bakery", 0.0).await?;
    let bakery_id = bakery.last_insert_id;
    let chef = insert_chef(&mut db, "John Doe", bakery_id).await?;
    let chef_id = chef.last_insert_id;

    let updated_chef = update_chef(&mut db, chef_id, Some("John"), None, None).await?;
    let updated_bakery = update_bakery(&mut db, bakery_id, Some("Happy Bakery"), None).await?;

    assert_eq!(updated_bakery.name, "Happy Bakery");
    assert_eq!(updated_chef.name, "John");

    delete_all_then_close(db).await
  }

  #[tokio::test]
  async fn find() -> Result<(), DbErr> {
    let mut db = establish_connection().await?;

    let bakery = insert_bakery(&mut db, "La Boulangerie", 0.0).await?;
    let bakery_id = bakery.last_insert_id;

    let chef_names = ["Jolie", "Charles", "Madeleine", "Frederic"]
      .into_iter()
      .collect::<HashSet<_>>();
    for chef_name in &chef_names {
      insert_chef(&mut db, chef_name, bakery_id).await?;
    }

    let jolie = find_chef_by_name(&db, "Jolie").await?.unwrap();
    let charles = find_chef_by_name(&db, "Charles").await?.unwrap();
    let madeleine = find_chef_by_name(&db, "Madeleine").await?.unwrap();
    let frederic = find_chef_by_name(&db, "Frederic").await?.unwrap();

    assert_eq!(jolie.name, "Jolie");
    assert_eq!(charles.name, "Charles");
    assert_eq!(madeleine.name, "Madeleine");
    assert_eq!(frederic.name, "Frederic");

    let all = find_all_chefs(&db).await?;
    assert_eq!(all.len(), 4);
    all.iter().for_each(|chef| {
      assert!(chef_names.contains(&chef.name.as_str()));
    });

    let jolie = find_chef_by_id(&db, jolie.id).await?.unwrap();
    let charles = find_chef_by_id(&db, charles.id).await?.unwrap();
    let madeleine = find_chef_by_id(&db, madeleine.id).await?.unwrap();
    let frederic = find_chef_by_id(&db, frederic.id).await?.unwrap();

    assert_eq!(jolie.name, "Jolie");
    assert_eq!(charles.name, "Charles");
    assert_eq!(madeleine.name, "Madeleine");
    assert_eq!(frederic.name, "Frederic");

    delete_all_then_close(db).await
  }

  #[tokio::test]
  async fn relational_select() -> Result<(), DbErr> {
    let mut db = establish_connection().await?;

    let bakery = insert_bakery(&mut db, "La Boulangerie", 0.0).await?;
    let bakery_id = bakery.last_insert_id;
    let chef_names = ["Jolie", "Charles", "Madeleine", "Frederic"];
    for chef_name in &chef_names {
      insert_chef(&mut db, chef_name, bakery_id).await?;
    }

    let la_boulangerie = find_bakery_by_id(&db, bakery_id).await?.unwrap();
    let mut chefs = la_boulangerie.find_related(Chef).all(&db).await?;
    chefs.sort_unstable_by_key(|chef| chef.name.clone());

    let mut directly_find_chefs = Chef::find()
      .filter(chef::Column::BakeryId.eq(bakery_id))
      .all(&db)
      .await?;
    directly_find_chefs.sort_unstable_by_key(|chef| chef.name.clone());

    assert_eq!(chefs.len(), 4);
    assert_eq!(directly_find_chefs.len(), 4);
    assert_eq!(chefs, directly_find_chefs);

    delete_all_then_close(db).await
  }

  #[tokio::test]
  async fn loader() -> Result<(), DbErr> {
    let mut db = establish_connection().await?;

    let la = insert_bakery(&mut db, "La Boulangerie", 0.0).await?;
    let la_id = la.last_insert_id;
    let arte = insert_bakery(&mut db, "Arte by Padaria", 0.2).await?;
    let arte_id = arte.last_insert_id;

    let la_chef_names = ["Jolie", "Charles", "Madeleine", "Frederic"];
    for chef_name in &la_chef_names {
      insert_chef(&mut db, chef_name, la_id).await?;
    }
    let arte_chef_names = ["Brian", "Charles", "Kate", "Samantha"];
    for chef_name in &arte_chef_names {
      insert_chef(&mut db, chef_name, arte_id).await?;
    }

    let bakeries = Bakery::find()
      .filter(
        Condition::any()
          .add(bakery::Column::Id.eq(la_id))
          .add(bakery::Column::Id.eq(arte_id)),
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
    assert_eq!(arte_chef_names, ["Brian", "Charles", "Kate", "Samantha"]);

    delete_all_then_close(db).await
  }
}

```

As we all know, rust will run all unit-tests via multi-threads.

However, this could cause `serious data racing` while executing `asynchronous unit-tests`, because NO default `synchronizing methods` / `critical data's ownership & race condition detector` are provided.

I can't accept to run the unit-tests in a single thread, then how could I synchronize data between all these tests?
