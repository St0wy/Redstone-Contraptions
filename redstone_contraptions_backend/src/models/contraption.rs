use crate::models::root::Context;
use juniper::GraphQLInputObject;

#[derive(Default, Debug)]
pub struct Contraption {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub items_list: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Contraption Input")]
pub struct ContraptionInput {
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub items_list: Option<String>,
}

#[juniper::graphql_object(Context = Context)]
impl Contraption {
    fn id(&self) -> &i32 {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn image(&self) -> &Option<String> {
        &self.image
    }

    fn items_list(&self) -> &Option<String> {
        &self.items_list
    }
}
