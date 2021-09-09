use crate::models::contraption::Contraption;
use crate::models::tag::Tag;
use crate::schema::contraption_tag;

#[derive(Default, Queryable, Identifiable, Associations)]
#[table_name = "contraption_tag"]
#[primary_key(contraption_id, tag_id)]
#[belongs_to(Contraption)]
#[belongs_to(Tag)]
pub struct ContraptionTag {
    pub contraption_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name = "contraption_tag"]
pub struct ContraptionTagInput {
    pub contraption_id: i32,
    pub tag_id: i32,
}
