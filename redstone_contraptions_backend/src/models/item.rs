use crate::models::root::Context;
use crate::schema::item;
use juniper::GraphQLInputObject;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub image: Option<Vec<u8>>,
}

#[derive(GraphQLInputObject, Insertable)]
#[graphql(description = "Item Input")]
#[table_name = "item"]
pub struct ItemInput {
    pub name: String,
}

#[juniper::graphql_object(Context = Context)]
impl Item {
    fn id(&self) -> &i32 {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
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
