use rocket::serde::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::categories::schema::{categories, categories::dsl::*};
use crate::categories::models::{Category, NewCategory};
// use crate::categories::schema;
use rocket::response::status::Created;

#[post("/", format = "json", data = "<category_input>")]
pub fn create_category(category_input: Json<NewCategory>) -> Result<Created<Json<Category>>, Status> {
    let mut conn = establish_connection();

    diesel::insert_into(categories)
        .values(&*category_input)
        .get_result::<Category>(&mut conn)
        .map(|cat| Created::new("/categories").body(Json(cat)))
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
pub fn list_categories() -> Result<Json<Vec<Category>>, Status> {
    let mut conn = establish_connection();
    categories
        .order(id.asc())
        .load::<Category>(&mut conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
