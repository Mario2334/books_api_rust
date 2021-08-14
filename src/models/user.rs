use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct PublicUser{
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String

}