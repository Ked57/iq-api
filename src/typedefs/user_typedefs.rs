use crate::schema::user;
use uuid::Uuid;

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

#[derive(juniper::GraphQLInputObject)]
pub struct UserWhereInput {
    pub id: Option<Uuid>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}

