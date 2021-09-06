use crate::models::contraption::Contraption;
use crate::models::root::Context;
use crate::models::tag::Tag;
use crate::schema::contraption_item;
use juniper::GraphQLInputObject;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "contraption_item"]
#[belongs_to(Contraption)]
#[belongs_to(Tag)]
pub struct ContraptionItem {
    pub contraption_id: i32,
    pub item_id: i32,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "contraption_item"]
pub struct ContraptionItemInput {
    pub contraption_id: i32,
    pub item_id: i32,
}
