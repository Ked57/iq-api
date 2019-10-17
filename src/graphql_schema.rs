extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::resolvers::user_resolvers::{users_resolver};
use crate::schema::user;
use crate::typedefs::user_typedefs::{User, UserInput};

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot {}

#[juniper::object(Context = Context)]
impl QueryRoot {
   fn users(context: &Context) -> Vec<User> {
      users_resolver(context)
   }
}

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
