use crate::models::contraption::Contraption;
use crate::models::item::Item;
use crate::schema::contraption_item;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "contraption_item"]
#[primary_key(contraption_id, item_id)]
#[belongs_to(Contraption)]
#[belongs_to(Item)]
pub struct ContraptionItem {
    pub contraption_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[table_name = "contraption_item"]
pub struct ContraptionTagInput {
    pub contraption_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}
