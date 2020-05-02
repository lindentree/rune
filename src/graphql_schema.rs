// graphql_schema.rs
use juniper::{EmptyMutation, RootNode};

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
  fn pocket_monsters() -> Vec<Pokemon> {
    vec![
      Pokemon {
        id: 1,
        name: "Bulbasaur".to_owned(),
      },
      Pokemon {
        id: 2,
        name: "Ivysaur".to_owned(),
      }
    ]
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}