table! {
    contraption (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        image -> Nullable<Bytea>,
    }
}

table! {
    contraption_item (contraption_id, item_id) {
        contraption_id -> Int4,
        item_id -> Int4,
        quantity -> Int4,
    }
}

table! {
    contraption_tag (contraption_id, tag_id) {
        contraption_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    item (id) {
        id -> Int4,
        name -> Varchar,
        image -> Nullable<Bytea>,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(contraption_item -> contraption (contraption_id));
joinable!(contraption_item -> item (item_id));
joinable!(contraption_tag -> contraption (contraption_id));
joinable!(contraption_tag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    contraption,
    contraption_item,
    contraption_tag,
    item,
    tag,
);
