use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::demo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Demo {
    pub demo_key: String,
}
