use actix_web::{
    Responder, post,
    web::{self, Json},
};
use sqlx::{Pool, Postgres};

use crate::{
    crypto::generate_token,
    data::{User, UserCredentials},
    db::{add_user, user_exists},
    request_data::{RegisterData, RegisterResponse},
    utils::check_valid_username,
};

#[post("/register")]
async fn register_user(
    data: web::Json<RegisterData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    println!("Registering user: {}", data.username);

    let valid_username = check_valid_username(&data.username);

    if !valid_username {
        println!("Invalid username: {}", data.username);
        return Json(RegisterResponse {
            message:
                "Invalid username. Usernames must be alphanumeric and up to 20 characters long."
                    .to_string(),
            auth_token: None,
        });
    }

    let exists = user_exists(&data.username, &pool).await;

    match exists {
        Ok(value) => {
            if value {
                println!("User already exists: {}", data.username);
                return Json(RegisterResponse {
                    message: "User already exists".to_string(),
                    auth_token: None,
                });
            }
        }
        Err(_) => {
            println!("Error checking if user exists: {}", data.username);
            return Json(RegisterResponse {
                message: "Error checking user existence".to_string(),
                auth_token: None,
            });
        }
    }

    let auth_token = generate_token();
    let user_credentials = UserCredentials {
        username: data.username.clone(),
        auth_token: auth_token.clone(),
    };

    let user = User {
        username: data.username.clone(),
        public_key: data.public_key.clone(),
        avatar: None,
    };

    let result = add_user(user_credentials, user, &pool).await;

    match result {
        Ok(_) => Json(RegisterResponse {
            message: "User registered successfully".to_string(),
            auth_token: Some(auth_token),
        }),
        Err(_) => Json(RegisterResponse {
            message: "Error registering user".to_string(),
            auth_token: None,
        }),
    }
}
