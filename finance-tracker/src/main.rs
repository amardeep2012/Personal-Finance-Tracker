#[macro_use] extern crate rocket;

use rocket::launch;
use rocket::fairing::AdHoc;


mod db;
mod auth;
mod categories;
mod transactions;
mod reports;
use categories::handlers::{create_category, list_categories};
use transactions::handlers::{create_transaction, list_transactions};
use auth::{signup_route, login_route};
use reports::handlers::category_summary;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok(); // Load environment variables from .env
    rocket::build()
        .mount("/auth", routes![signup_route, login_route]) 
        .mount("/categories", routes![create_category, list_categories])
        .mount("/transactions", routes![create_transaction, list_transactions]) 
        .mount("/reports", routes![category_summary])
        // Mount auth routes
        .attach(AdHoc::on_ignite("Database Connection", |rocket| async {

            db::establish_connection();
            println!("✅ Successfully connected to database!");

            rocket
        }))
}

