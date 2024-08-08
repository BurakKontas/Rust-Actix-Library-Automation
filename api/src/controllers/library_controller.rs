use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::AppState;

// Request yapılandırmaları
#[derive(Deserialize)]
pub struct CreateLibraryRequest {
    name: String,
    address: String,
    manager_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateLibraryRequest {
    name: String,
    address: String,
    manager_id: i32,
}

// Handlers
pub async fn create_library(
    repos: web::Data<AppState>,
    form: web::Json<CreateLibraryRequest>,
) -> impl Responder {
    let mut lib_repo = repos.lib_repo.lock().unwrap();
    
    match lib_repo.create_library(&form.name, &form.address, &form.manager_id) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to create library: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to create library: {}", e))
        }
    }
}

pub async fn update_library(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
    form: web::Json<UpdateLibraryRequest>,
) -> impl Responder {
    let mut lib_repo = repos.lib_repo.lock().unwrap();
    
    match lib_repo.update_library(&id, &form.name, &form.address, &form.manager_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to update library: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to update library: {}", e))
        }
    }
}

pub async fn delete_library(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut lib_repo = repos.lib_repo.lock().unwrap();
    
    match lib_repo.delete_library(&id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("Failed to delete library: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to delete library: {}", e))
        }
    }
}

pub async fn get_library(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut lib_repo = repos.lib_repo.lock().unwrap();
    
    match lib_repo.get_library_by_id(&id) {
        Ok(library) => HttpResponse::Ok().json(library),
        Err(e) => {
            println!("Failed to get library: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get library: {}", e))
        }
    }
}

pub async fn get_libraries(
    repos: web::Data<AppState>,
) -> impl Responder {
    let mut lib_repo = repos.lib_repo.lock().unwrap();
    
    match lib_repo.get_libraries() {
        Ok(libraries) => HttpResponse::Ok().json(libraries),
        Err(e) => {
            println!("Failed to get libraries: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get libraries: {}", e))
        }
    }
}

// Routes configuration
pub fn library_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/libraries")
            .route(web::post().to(create_library))
            .route(web::get().to(get_libraries))
    )
    .service(
        web::resource("/libraries/{id}")
            .route(web::get().to(get_library))
            .route(web::put().to(update_library))
            .route(web::delete().to(delete_library))
    );
}
