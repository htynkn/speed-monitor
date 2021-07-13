use super::schema::speed;

use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct SpeedQuery {
    pub id: i32,
    pub download: i32,
    pub upload: i32,
}

#[derive(Insertable)]
#[table_name = "speed"]
pub struct SpeedInsert {
    pub download: i32,
    pub upload: i32,
}
