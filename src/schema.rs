table! {
    post (id) {
        id -> Uuid,
        user_id -> Uuid,
        date -> Varchar,
    }
}

table! {
    user (id) {
        id -> Uuid,
        user_id -> Varchar,
        user_name -> Varchar,
    }
}

joinable!(post -> user (user_id));

allow_tables_to_appear_in_same_query!(
    post,
    user,
);
