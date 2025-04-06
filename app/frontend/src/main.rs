use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
use tera::Tera;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SearchResult {
    title: String,
    description: String,
    url: String,
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let ctx = tera::Context::new();
    let html = tmpl.render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn about(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let ctx = tera::Context::new();
    let html = tmpl.render("about.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn login(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let ctx = tera::Context::new();
    let html = tmpl.render("login.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn register(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let ctx = tera::Context::new();
    let html = tmpl.render("register.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn search(
    query: web::Query<HashMap<String, String>>,
    client: web::Data<reqwest::Client>,
    tmpl: web::Data<Tera>
) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();
    
    if let Some(q) = query.get("q") {
        let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://backend:8081".to_string());
        let results: Vec<SearchResult> = client
            .get(&format!("{}/api/search?q={}", api_url, q))
            .send()
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("API error"))?
            .json()
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("JSON error"))?;
            
        ctx.insert("search_results", &results);
        ctx.insert("query", q);
    }
    
    let html = tmpl.render("search.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    
    let client = reqwest::Client::new();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(web::resource("/").to(index))
            .service(web::resource("/about").to(about))
            .service(web::resource("/login").to(login))
            .service(web::resource("/register").to(register))
            .service(web::resource("/search").to(search))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}