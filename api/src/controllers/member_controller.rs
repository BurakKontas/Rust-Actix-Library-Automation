use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::AppState;

// Request yapılandırmaları
#[derive(Deserialize)]
pub struct CreateMemberRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateMemberRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct BorrowReturnRequest {
    member_id: i32,
    book_id: i32,
}

#[derive(Deserialize)]
pub struct GetBorrowedBooksRequest {
    member_id: i32,
    #[serde(default = "default_library_id")]
    library_id: i32,
}

fn default_library_id() -> i32 {
    -1
}

// Handlers
pub async fn create_member(
    repos: web::Data<AppState>,
    form: web::Json<CreateMemberRequest>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.create_member(&form.name, &form.email) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to create member: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to create member: {}", e))
        }
    }
}

pub async fn update_member(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
    form: web::Json<UpdateMemberRequest>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.update_member(&id, &form.name, &form.email) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to update member: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to update member: {}", e))
        }
    }
}

pub async fn delete_member(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.delete_member(&id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("Failed to delete member: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to delete member: {}", e))
        }
    }
}

pub async fn get_member(
    repos: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.get_member_by_id(&id) {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(e) => {
            println!("Failed to get member: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get member: {}", e))
        }
    }
}

pub async fn get_members(
    repos: web::Data<AppState>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.get_members() {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            println!("Failed to get members: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get members: {}", e))
        }
    }
}

pub async fn get_members_by_library_id(
    repos: web::Data<AppState>,
    library_id: web::Path<i32>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.get_members_by_library_id(&library_id) {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            println!("Failed to get members by library id: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get members by library id: {}", e))
        }
    }
}

pub async fn borrow_book(
    repos: web::Data<AppState>,
    path: web::Path<BorrowReturnRequest>,
) -> impl Responder {
    let BorrowReturnRequest { member_id, book_id } = path.into_inner();
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.borrow_book(&member_id, &book_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to borrow book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to borrow book: {}", e))
        }
    }
}

pub async fn return_book(
    repos: web::Data<AppState>,
    path: web::Path<BorrowReturnRequest>,
) -> impl Responder {
    let BorrowReturnRequest { member_id, book_id } = path.into_inner();
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.return_book(&member_id, &book_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to return book: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to return book: {}", e))
        }
    }
}

pub async fn get_borrowed_books(
    repos: web::Data<AppState>,
    path: web::Path<GetBorrowedBooksRequest>,
) -> impl Responder {
    let mut member_repo = repos.member_repo.lock().unwrap();
    match member_repo.get_borrowed_books(&path.member_id, &path.library_id) {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => {
            println!("Failed to get borrowed books: {:?}", e); // Hata mesajını logla
            HttpResponse::InternalServerError().body(format!("Failed to get borrowed books: {}", e))
        }
    }
}

// Routes configuration
pub fn member_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/members")
            .route(web::post().to(create_member))
            .route(web::get().to(get_members))
    )
    .service(
        web::resource("/members/{id}")
            .route(web::get().to(get_member))
            .route(web::put().to(update_member))
            .route(web::delete().to(delete_member))
    )
    .service(
        web::resource("/members/{member_id}/borrowed_books")
            .route(web::get().to(get_borrowed_books))
    )
    .service(
        web::resource("/members/{member_id}/borrowed_books/{library_id}")
            .route(web::get().to(get_borrowed_books))
    )
    .service(
        web::resource("/members/{member_id}/books/{book_id}")
            .route(web::post().to(borrow_book))
            .route(web::delete().to(return_book))
    )
    .service(
        web::resource("/libraries/{library_id}/members")
            .route(web::get().to(get_members_by_library_id))
    );
}
