// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_digest -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
