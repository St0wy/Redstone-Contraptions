use crate::models::root::Context;
use crate::schema::{contraption, tag};
use juniper::GraphQLInputObject;
extern crate base64;
use crate::models::tag::Tag;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "contraption"]
#[has_many(tag)]
pub struct Contraption {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<Vec<u8>>,
    pub tags: Vec<Tag>,
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

    fn tags(&self) -> Vec<Tag> {
        let taggers = Tag::belonging_to();
    }
}
