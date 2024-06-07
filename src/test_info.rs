pub mod bakery_info {
  pub struct BakeryInfo {
    pub id: i32,
    pub name: &'static str,
    pub profit_margin: f64,
  }
  pub const BAKERY: BakeryInfo = BakeryInfo {
    id: 1,
    name: "Bakery",
    profit_margin: 0.0,
  };
  pub const LA_BOULANGERIE: BakeryInfo = BakeryInfo {
    id: 2,
    name: "La Boulangerie",
    profit_margin: 0.0,
  };
  pub const ARTE_BY_PADARIA: BakeryInfo = BakeryInfo {
    id: 3,
    name: "Arte by Padaria",
    profit_margin: 0.2,
  };
}

pub mod chef_info {
  pub struct ChefInfo {
    pub id: i32,
    pub name: &'static str,
    pub bakery_id: i32,
  }
  pub const JOHN: ChefInfo = ChefInfo {
    id: 1,
    name: "John Doe",
    bakery_id: 1,
  };
  pub const JOLIE: ChefInfo = ChefInfo {
    id: 2,
    name: "Jolie",
    bakery_id: 2,
  };
  pub const CHARLES: ChefInfo = ChefInfo {
    id: 3,
    name: "Charles",
    bakery_id: 2,
  };
  pub const MADELEINE: ChefInfo = ChefInfo {
    id: 4,
    name: "Madeleine",
    bakery_id: 2,
  };
  pub const FREDERIC: ChefInfo = ChefInfo {
    id: 5,
    name: "Frederic",
    bakery_id: 2,
  };
  pub const BRIAN: ChefInfo = ChefInfo {
    id: 6,
    name: "Brian",
    bakery_id: 3,
  };
  pub const JERRY: ChefInfo = ChefInfo {
    id: 7,
    name: "Jerry",
    bakery_id: 3,
  };
  pub const KATE: ChefInfo = ChefInfo {
    id: 8,
    name: "Kate",
    bakery_id: 3,
  };
  pub const SAMANTHA: ChefInfo = ChefInfo {
    id: 9,
    name: "Samantha",
    bakery_id: 3,
  };
}
