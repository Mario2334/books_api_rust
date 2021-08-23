use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, PgPool};
use actix_web::{Responder, HttpRequest, HttpResponse,Error};
use anyhow::Result;
use futures::future::{ready, Ready};

#[derive(Deserialize,Serialize, FromRow)]
pub struct PublicUsers{
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String
}

impl Responder for PublicUsers{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        ))
    }
}

impl PublicUsers{
    pub async fn find_all(pool:&PgPool) -> Result<Vec<PublicUsers>>{
        let mut users = vec![];
        let recs = sqlx::query!(
            r#"select * from public_users;"#
        ).fetch_all(pool).await?;
        for rec in recs{
            users.push( PublicUsers{
                user_id: rec.user_id,
                firstname: rec.firstname,
                lastname: rec.lastname
            });
        }
        Ok(users)
    }

    pub async fn create(&self, pool:&PgPool) -> Result<PublicUsers,sqlx::error::Error> {
        println!("{}",self.firstname.as_str());
        // let raw_query = format!(
        //     r#"INSERT INTO public_users (firstname, lastname) VALUES ('{}','{}') "#,
        //     self.firstname,self.lastname);

        let rec = sqlx::query_as!(PublicUsers,"INSERT INTO public_users (firstname, lastname) VALUES ( $1, $2) RETURNING *;",self.firstname.as_str(),self.lastname.as_str())
            .fetch_one(pool).await?;
        println!("{}",self.firstname.as_str());
        Ok(rec)
    }
}