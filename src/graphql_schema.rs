extern crate dotenv;

use diesel::prelude::*;
use juniper::{RootNode, FieldResult, FieldError};

use crate::db::PgPool;
use crate::resolvers::user_resolvers::{users_resolver, user_resolver_by_id, user_resolver_by_user_id, user_resolver_by_user_name};
use crate::schema::user;
use crate::typedefs::user_typedefs::{User, UserInput, UserWhereInput};

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot {}

fn type_check_query_result<T>(query_result: Option<T>) -> Result<T, FieldError> {
    match query_result {
        Some(res) => Ok(res),
        None => Err(FieldError::new("Got an empty result", graphql_value!({"Empty result": "Got an empty result"})))
    }
}

graphql_object!(QueryRoot: Context |&self| {

    field users(&executor) -> FieldResult<Vec<User>> {
        Ok(users_resolver(executor.context()))
    }

    field user(&executor, where_input: UserWhereInput) -> FieldResult<User> {
        match where_input {
            UserWhereInput{ id, user_id, user_name} => match (id, user_id, user_name) {
                (Some(id), ..) => type_check_query_result(user_resolver_by_id(executor.context(), id)),
                (_,Some(user_id),_) => type_check_query_result(user_resolver_by_user_id(executor.context(), user_id)),
                (_,_,Some(usr_name)) => type_check_query_result(user_resolver_by_user_name(executor.context(), usr_name)),
                (None, None, None) => Err(FieldError::new("Please don't provide an empty variable", graphql_value!({ "bad_request": "Please don't provide an empty variable" })))
            }
        }
    }
});

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, data: UserInput) -> User {
        let connection = context.db.get().unwrap();;
        diesel::insert_into(user::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving new post")
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
