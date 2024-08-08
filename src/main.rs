use actix_web::{web, App, HttpServer};
use api::controllers::book_controller::book_routes;
use api::controllers::library_controller::library_routes;
use api::controllers::member_controller::member_routes;
use api::create_app_state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    let app_state = create_app_state();
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(book_routes)
            .configure(library_routes)
            .configure(member_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
