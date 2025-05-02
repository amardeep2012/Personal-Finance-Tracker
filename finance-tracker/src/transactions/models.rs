use crate::transactions::schema::transactions;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use diesel::{prelude::Queryable, Insertable};


#[derive(Queryable, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: BigDecimal,
    pub description: String,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub amount: BigDecimal,
    pub description: String,
    pub category_id: i32,
}
