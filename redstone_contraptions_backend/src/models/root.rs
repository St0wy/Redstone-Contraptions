use super::contraption::{Contraption, ContraptionInput};
use crate::db::Pool;
use juniper::{EmptySubscription, FieldResult};

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all contraptions")]
    fn contraptions(context: &Context) -> FieldResult<Vec<Contraption>> {
        Ok(vec![Contraption {
            id: 1,
            name: "Ripe".to_string(),
            description: String::from("Salut"),
            image: Some(String::from("Salut")),
            items_list: Some(String::from("Salut")),
        }])
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_contraption(
        context: &Context,
        contraption: ContraptionInput,
    ) -> FieldResult<Contraption> {
        Ok(Contraption {
            id: 1,
            name: String::from("Salut"),
            description: String::from("Salut"),
            image: Some(String::from("Salut")),
            items_list: Some(String::from("Salut")),
        })
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
