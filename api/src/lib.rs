use std::sync::{Arc, Mutex};
use domain::traits::{BookRepositoryTrait, LibraryRepositoryTrait, MemberRepositoryTrait};
use infrastructure::repositories::book_repository::BookRepository;
use infrastructure::establish_connection;
use infrastructure::repositories::create_tables;
use infrastructure::repositories::library_repository::LibraryRepository;
use infrastructure::repositories::member_repository::MemberRepository;

pub mod controllers;

pub struct AppState {
    pub book_repo: Arc<Mutex<dyn BookRepositoryTrait + Send + Sync>>,
    pub lib_repo : Arc<Mutex<dyn LibraryRepositoryTrait + Send + Sync>>,
    pub member_repo : Arc<Mutex<dyn MemberRepositoryTrait + Send + Sync>>,
}


pub fn create_app_state() -> AppState {
    let pool = establish_connection();
    let arc_pool = Arc::new(pool);
    let book_repo = Arc::new(Mutex::new(BookRepository::new(arc_pool.clone())));
    let lib_repo = Arc::new(Mutex::new(LibraryRepository::new(arc_pool.clone())));
    let member_repo = Arc::new(Mutex::new(MemberRepository::new(arc_pool.clone())));

    create_tables(&arc_pool);

    AppState {
        book_repo,
        lib_repo,
        member_repo,
    }
}
