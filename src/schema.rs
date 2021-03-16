table! {
    messages (id) {
        id -> Int8,
        author -> Varchar,
        author_id -> Varchar,
        content -> Varchar,
        attachment -> Nullable<Varchar>,
        channel -> Varchar,
        channel_id -> Varchar,
        time_posted -> Nullable<Timestamp>,
        mentions -> Nullable<Varchar>,
        reactions -> Nullable<Varchar>,
    }
}
