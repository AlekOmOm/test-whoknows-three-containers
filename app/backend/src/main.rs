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
        // Example query - replace with your actual database logic
        let results = vec![
            SearchResult {
                title: format!("Result for: {}", q),
                description: "This is a sample result".to_string(),
                url: "https://example.com".to_string(),
            }
        ];
        
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