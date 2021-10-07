table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        subject -> Varchar,
        author -> Varchar,
        published_date -> Nullable<Timestamptz>,
        editor -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}
