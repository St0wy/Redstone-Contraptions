use crate::models::root::Context;
use crate::schema::contraption;
use juniper::GraphQLInputObject;
extern crate base64;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "contraption"]
pub struct Contraption {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<Vec<u8>>,
}

#[derive(GraphQLInputObject, Insertable)]
#[graphql(description = "Contraption Input")]
#[table_name = "contraption"]
pub struct ContraptionInput {
    pub name: String,
    pub description: String,
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

    fn image(&self) -> Option<String> {
        if self.image.is_none() {
            None
        } else {
            let img = self.image.as_ref().unwrap();
            Some(format!("data:image/png;base64,{}", base64::encode(img)))
        }
    }
}
