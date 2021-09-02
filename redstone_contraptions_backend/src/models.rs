use juniper::{FieldResult, GraphQLObject, RootNode};

#[derive(Queryable, juniper::GraphQLObject)]
#[graphql(description = "A redstone contraption.")]
pub struct Contraption {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub items_list: Option<String>,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn contraptions() -> FieldResult<Vec<Contraption>> {
        Ok(vec![Contraption {
            id: 1,
            name: String::from("Flip flop"),
            description: String::from("C'est un flip flop quoi fait pas chier"),
            image: Some(String::from("svg chaipakoi j'ai pas le temps")),
            items_list: Some(String::from("De la cobble pierre")),
        }])
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
