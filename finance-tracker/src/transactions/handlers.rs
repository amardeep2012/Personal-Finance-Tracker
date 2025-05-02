use rocket::serde::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use rocket::response::status::Created;

use crate::db::establish_connection;
use crate::transactions::models::{Transaction, NewTransaction};
use crate::transactions::schema::transactions::dsl::*;

#[post("/", format = "json", data = "<txn_input>")]
pub fn create_transaction(txn_input: Json<NewTransaction>) -> Result<Created<Json<Transaction>>, Status> {
    let mut conn = establish_connection();

    diesel::insert_into(transactions)
        .values(&*txn_input)
        .get_result::<Transaction>(&mut conn)
        .map(|txn| Created::new("/transactions").body(Json(txn)))
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
pub fn list_transactions() -> Result<Json<Vec<Transaction>>, Status> {
    let mut conn = establish_connection();

    transactions
        .order(id.desc())
        .load::<Transaction>(&mut conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
