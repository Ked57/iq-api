use crate::graphql_schema::Context;
use crate::typedefs::user_typedefs::{User, UserInput};
use uuid::Uuid;
use crate::schema::user::dsl::*;
use crate::schema::user;
use diesel::prelude::*;
use diesel::result::Error;

pub fn users_resolver(context: &Context) -> Vec<User> {
    let connection = context.db.get().unwrap();
    let query_result = user.limit(100)
        .load::<User>(&connection);
    match query_result {
        Ok(users_result) => users_result,
        Err(_) => Vec::new()
    }
}

pub fn user_resolver_by_id(context: &Context, id_param: Uuid) -> Result<Option<User>, Error> {
    let connection = context.db.get().unwrap();
    user.find(id_param)
        .first(&connection)
        .optional()
}

pub fn user_resolver_by_user_id(context: &Context, user_id_param: String) -> Result<Option<User>, Error> {
    let connection = context.db.get().unwrap();
    user
        .filter(user_id.eq(user_id_param))
        .first(&connection)
        .optional()
}

pub fn user_resolver_by_user_name(context: &Context, user_name_param: String) -> Result<Option<User>, Error> {
    let connection = context.db.get().unwrap();
    user
        .filter(user_name.eq(user_name_param))
        .first(&connection)
        .optional()
}

pub fn create_user_resolver(context: &Context, data: UserInput) -> Result<User, Error> {
        let connection = context.db.get().unwrap();
        diesel::insert_into(user::table)
            .values(&data)
            .get_result(&connection)
    }
