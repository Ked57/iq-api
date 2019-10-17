extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;
use uuid::Uuid;

use crate::db::PgPool;
use crate::schema::user;

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
}

#[juniper::object(description = "A User of instaq")]
impl User {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub fn user_name(&self) -> &str {
        self.user_name.as_str()
    }
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "user"]
pub struct UserInput {
    pub user_id: String,
    pub user_name: String,
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn users(context: &Context) -> Vec<User> {
        use crate::schema::user::dsl::*;
        let connection = context.db.get().unwrap();
        user.limit(100)
            .load::<User>(&connection)
            .expect("Couldn't load users")
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
