use crate::diesel::BelongingToDsl;
use crate::models::contraption_tag::ContraptionTag;
use crate::models::root::Context;
use crate::models::tag::Tag;
use crate::schema::{contraption, contraption_tag, tag};
use diesel::prelude::*;
extern crate base64;
use juniper::GraphQLInputObject;
use juniper::{FieldError, FieldResult, Value};

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

    fn tags(&self, context: &Context) -> FieldResult<Vec<Tag>> {
        use diesel::pg::expression::dsl::*;

        let conn = context.dbpool.get().map_err(|_| {
            FieldError::new("Could not open connection to the database", Value::null())
        })?;

        let contraption_tag_ids =
            ContraptionTag::belonging_to(self).select(contraption_tag::tag_id);

        tag::table.filter(tag::id.eq(any(contraption_tag_ids)))
            .load::<Tag>(&conn)
            .map_err(|_| FieldError::new("Error loading tags", Value::null()))
    }
}
