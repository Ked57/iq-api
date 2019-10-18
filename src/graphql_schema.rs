extern crate dotenv;

use diesel::prelude::*;
use juniper::{RootNode, FieldResult};
use uuid::Uuid;

use crate::db::PgPool;
use crate::resolvers::user_resolvers::{users_resolver, user_resolver_by_id};
use crate::schema::user;
use crate::typedefs::user_typedefs::{User, UserInput};

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot {}

graphql_object!(QueryRoot: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        Ok(users_resolver(executor.context()))
    }

    field user(&executor, id: Uuid) -> FieldResult<Option<User>> {
        Ok(user_resolver_by_id(executor.context(), id))
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
