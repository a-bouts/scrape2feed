table! {
    feeds (id) {
        id -> Text,
        title -> Text,
        link -> Text,
        node_selector -> Text,
        title_selector -> Nullable<Text>,
        link_selector -> Nullable<Text>,
    }
}

table! {
    items (id) {
        id -> Text,
        feed -> Text,
        title -> Text,
        link -> Text,
        publication_date -> Timestamp,
    }
}

joinable!(items -> feeds (feed));

allow_tables_to_appear_in_same_query!(
    feeds,
    items,
);
