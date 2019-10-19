extern crate dotenv;

use juniper::{RootNode, FieldResult, FieldError};
use diesel::result::Error;

use crate::db::PgPool;
use crate::resolvers::user_resolvers::{users_resolver, user_resolver_by_id, user_resolver_by_user_id, user_resolver_by_user_name, create_user_resolver};
use crate::typedefs::user_typedefs::{User, UserInput, UserWhereInput};

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot {}

fn type_check_query_result<T>(query_result: Result<T, Error>) -> Result<T, FieldError> {
    match query_result {
        Ok(res) => Ok(res),
        Err(e) => Err(FieldError::new(e, graphql_value!({"Empty result": "Got an empty result"})))
    }
}

graphql_object!(QueryRoot: Context |&self| {

    field users(&executor) -> FieldResult<Vec<User>> {
        Ok(users_resolver(executor.context()))
    }

    field user(&executor, where_input: UserWhereInput) -> FieldResult<Option<User>> {
        match where_input {
            UserWhereInput{ id, user_id, user_name} => match (id, user_id, user_name) {
                (Some(id), ..) => {
                    let result = type_check_query_result(user_resolver_by_id(executor.context(), id));
                    match result {
                        Ok(user_result) => Ok(user_result),
                        Err(e) => Err(FieldError::from(e))
                    }
                },
                (_,Some(user_id),_) => {
                    let result = type_check_query_result(user_resolver_by_user_id(executor.context(), user_id));
                    match result {
                        Ok(user_result) => Ok(user_result),
                        Err(e) => Err(FieldError::from(e))
                    }
                },
                (_,_,Some(user_name)) => {
                    let result = type_check_query_result(user_resolver_by_user_name(executor.context(), user_name));
                    match result {
                        Ok(user_result) => Ok(user_result),
                        Err(e) => Err(FieldError::from(e))
                    }
                },
                (None, None, None) => Err(FieldError::new("Please don't provide an empty variable", graphql_value!({ "bad_request": "Please don't provide an empty variable" })))
            }
        }
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {

    field create_user(&executor, input: UserInput) -> FieldResult<User> {
        let insert_result = create_user_resolver(executor.context(), input);
        match insert_result {
            Ok(inserted_user) => Ok(inserted_user),
            Err(e) => Err(FieldError::from(e))
        }
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
