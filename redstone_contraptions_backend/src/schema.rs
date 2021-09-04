table! {
    contraption (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        image -> Nullable<Text>,
        itemslist -> Nullable<Text>,
    }
}

table! {
    contraption_item (contraptions_id, items_id) {
        contraptions_id -> Int4,
        items_id -> Int4,
    }
}

table! {
    contraption_tag (contraptions_id, tags_id) {
        contraptions_id -> Int4,
        tags_id -> Int4,
    }
}

table! {
    item (id) {
        id -> Int4,
        name -> Varchar,
        image -> Bytea,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(contraption_item -> contraption (contraptions_id));
joinable!(contraption_item -> item (items_id));
joinable!(contraption_tag -> contraption (contraptions_id));
joinable!(contraption_tag -> item (tags_id));

allow_tables_to_appear_in_same_query!(
    contraption,
    contraption_item,
    contraption_tag,
    item,
    tag,
);
