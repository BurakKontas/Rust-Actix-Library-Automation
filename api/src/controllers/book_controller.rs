use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::AppState;

// Request yapılandırması
#[derive(Deserialize)]
pub struct CreateBookRequest {
    title: String,
    author: String,
    library_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateBookRequest {
    title: String,
    author: String,
}

pub async fn create_book(
    repos: web::Data<AppState>,
    form: web::Json<CreateBookRequest>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().create_book(&form.title, &form.author, &form.library_id);

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to create book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to create book: {}", e))
        }
    }
}

pub async fn update_book(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
    form: web::Json<UpdateBookRequest>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().update_book(&id, &form.title, &form.author);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to update book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to update book: {}", e))
        }
    }
}

pub async fn delete_book(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().delete_book(&id);

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("Failed to delete book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to delete book: {}", e))
        }
    }
}

pub async fn get_book(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().get_book_by_id(&id);

    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => {
            println!("Failed to get book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get book: {}", e))
        }
    }
}

pub async fn get_books(
    repos: web::Data<AppState>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().get_books();

    match result {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => {
            println!("Failed to get books: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get books: {}", e))
        }
    }
}

pub async fn get_books_by_library_id(
    repos: web::Data<AppState>,
    library_id: web::Path<i32>,
) -> impl Responder {
    let book_repo = repos.book_repo.clone();
    let result = book_repo.lock().unwrap().get_books_by_library_id(&library_id);

    match result {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => {
            println!("Failed to get books by library id: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get books by library id: {}", e))
        }
    }
}

// Actix-web yapılandırması
pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/books")
            .route(web::post().to(create_book))
            .route(web::get().to(get_books))
    )
    .service(
        web::resource("/books/{id}")
            .route(web::get().to(get_book))
            .route(web::put().to(update_book))
            .route(web::delete().to(delete_book))
    )
    .service(
        web::resource("/libraries/{library_id}/books")
            .route(web::get().to(get_books_by_library_id))
    );
}