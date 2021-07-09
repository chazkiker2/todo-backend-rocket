table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        completed_at -> Nullable<Timestamptz>,
    }
}
