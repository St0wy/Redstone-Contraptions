table! {
    contraptions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        image -> Nullable<Text>,
        itemslist -> Nullable<Text>,
    }
}

table! {
    contraptionsitems (contraptions_id, items_id) {
        contraptions_id -> Int4,
        items_id -> Int4,
    }
}

table! {
    contraptionstags (contraptions_id, tags_id) {
        contraptions_id -> Int4,
        tags_id -> Int4,
    }
}

table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        image -> Bytea,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(contraptionsitems -> contraptions (contraptions_id));
joinable!(contraptionsitems -> items (items_id));
joinable!(contraptionstags -> contraptions (contraptions_id));
joinable!(contraptionstags -> items (tags_id));

allow_tables_to_appear_in_same_query!(
    contraptions,
    contraptionsitems,
    contraptionstags,
    items,
    tags,
);
