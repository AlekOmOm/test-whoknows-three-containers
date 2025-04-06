use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct SearchResult {
    title: String,
    description: String,
    url: String,
}

async fn search(
    query: web::Query<std::collections::HashMap<String, String>>,
    db: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    if let Some(q) = query.get("q") {
        // Real database query
        let results = sqlx::query_as!(
            SearchResult,
            "SELECT title, description, url FROM search_results WHERE title ILIKE $1 OR description ILIKE $1",
            format!("%{}%", q)
        )
        .fetch_all(db.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        
        Ok(HttpResponse::Ok().json(results))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/search").to(search))
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}