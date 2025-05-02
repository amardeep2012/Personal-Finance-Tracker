use rocket::serde::json::Json;
use diesel::prelude::*;
use diesel::sql_query;

use crate::db::establish_connection;
use crate::reports::models::CategorySummary;

#[get("/category-summary")]
pub fn category_summary() -> Json<Vec<CategorySummary>> {
    let mut conn = establish_connection();

    let results = sql_query(
        r#"
        SELECT c.name AS category, SUM(t.amount) AS total
        FROM transactions t
        INNER JOIN categories c ON t.category_id = c.id
        GROUP BY c.name
        ORDER BY total DESC
        "#
    )
    .load::<CategorySummary>(&mut conn)
    .expect("Failed to run report query");

    Json(results)
}
