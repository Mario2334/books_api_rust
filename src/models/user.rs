use books_api::schema::{public_users};
use books_api::db;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Deserialize,Serialize, Queryable)]
// #[sql_name="public_users"]
pub struct PublicUsers{
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String
}