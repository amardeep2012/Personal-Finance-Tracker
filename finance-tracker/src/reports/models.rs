
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use diesel::QueryableByName;
#[derive(Debug, Serialize, Deserialize, QueryableByName)]

pub struct CategorySummary {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub category: String,
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    pub total: BigDecimal,
}