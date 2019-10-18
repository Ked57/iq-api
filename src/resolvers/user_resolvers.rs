use crate::graphql_schema::Context;
use crate::typedefs::user_typedefs::User;
use uuid::Uuid;
use crate::schema::user::dsl::*;
use diesel::prelude::*;

pub fn users_resolver(context: &Context) -> Vec<User> {
    let connection = context.db.get().unwrap();
    let query_result = user.limit(100)
        .load::<User>(&connection);
    match query_result {
        Ok(users_result) => users_result,
        Err(_) => Vec::new()
    }
}

pub fn user_resolver_by_id(context: &Context, uuid: Uuid) -> Option<User> {
    let connection = context.db.get().unwrap();
    let query_result = user.find(uuid)
        .first(&connection);
    match query_result {
        Ok(user_result) => Some(user_result),
        Err(_) => None
    }
}
