use crate::models::root::Context;
use crate::schema::contraptions;
use juniper::GraphQLInputObject;

#[derive(Default, Queryable)]
pub struct Contraption {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub itemslist: Option<String>,
}

#[derive(GraphQLInputObject, Insertable)]
#[graphql(description = "Contraption Input")]
#[table_name = "contraptions"]
pub struct ContraptionInput {
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub itemslist: Option<String>,
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

    fn itemslist(&self) -> &Option<String> {
        &self.itemslist
    }
}
