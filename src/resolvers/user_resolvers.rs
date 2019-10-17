use diesel::prelude::*;
use crate::typedefs::user_typedefs::User;
use crate::graphql_schema::Context;

pub fn users_resolver(context: &Context) -> Vec<User> {
        use crate::schema::user::dsl::*;
        let connection = context.db.get().unwrap();
        user.limit(100)
            .load::<User>(&connection)
            .expect("Couldn't load users")
    }