use super::schema::categories;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::categories::schema::categories::dsl::*;
use diesel::{Queryable, Insertable};

#[derive(Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}
