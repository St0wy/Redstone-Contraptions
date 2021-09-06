use crate::models::contraption::Contraption;
use crate::models::root::Context;
use crate::schema::tag;
use juniper::GraphQLInputObject;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "tag"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[graphql(description = "Tag Input")]
#[table_name = "tag"]
pub struct TagInput {
    pub name: String,
}

#[juniper::graphql_object(Context = Context)]
impl Tag {
    fn id(&self) -> &i32 {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }
}
