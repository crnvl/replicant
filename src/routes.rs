use actix_web::{HttpResponse, Responder, post, web};
use sqlx::{Pool, Postgres};

use crate::{
    data::{User, UserCredentials},
    db::{self, add_user},
    request_data::RegisterData,
};

#[post("/register")]
async fn register_user(
    data: web::Json<RegisterData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let user_credentials = UserCredentials {
        username: data.username.clone(),
        auth_token: "".to_string(),
    };

    let user = User {
        username: data.username.clone(),
        public_key: data.public_key.clone(),
        avatar: None,
    };

    let result = add_user(user_credentials, user, &pool).await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
