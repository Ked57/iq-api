table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        knockouts -> Int4,
        team_id -> Int4,
    }
}

table! {
    post (id) {
        id -> Uuid,
        user_id -> Uuid,
        date -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user (id) {
        id -> Uuid,
        user_id -> Varchar,
        user_name -> Varchar,
    }
}

joinable!(members -> teams (team_id));
joinable!(post -> user (user_id));

allow_tables_to_appear_in_same_query!(
    members,
    post,
    teams,
    user,
);
