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
    items (id) {
        id -> Int4,
        name -> Varchar,
        image -> Bytea,
    }
}

allow_tables_to_appear_in_same_query!(
    contraptions,
    items,
);
