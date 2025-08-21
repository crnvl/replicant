use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use sqlx::{Pool, Postgres};

#[post("/register")]
async fn register_user(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user: User = req.json().await.unwrap();
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        user.name,
        user.email
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
