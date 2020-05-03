// graphql_schema.rs
use juniper::{EmptyMutation, RootNode, FieldResult};

struct Pokemon {
  id: i32,
  name: String,
}

#[juniper::object(description = "A member of a team")]
impl Pokemon {
  pub fn id(&self) -> i32 {
    self.id  
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  fn pocket_monsters(id: i32) -> Vec<Pokemon> {
    vec![
      Pokemon {
        id: 1.to_owned(),
        name: "Bulbasaur".to_owned(),
      },
      Pokemon {
        id: 2.to_owned(),
        name: "Ivysaur".to_owned(),
      },
      Pokemon {
        id: 3.to_owned(),
        name: "Venusaur".to_owned(),
      }
    ]
    .into_iter()
    .filter(|pokemon| pokemon.id == id)
    .collect()
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}