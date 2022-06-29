use crate::schema::tours;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Tour {
    id: Uuid,
    name: String,
    src: String,
    dst: String,
    total_days: i32,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "tours"]
pub struct NewTour {
    pub id: Uuid,
    pub name: String,
    pub src: String,
    pub dst: String,
    pub total_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourData {
    pub name: String,
    pub src: String,
    pub dst: String,
    pub total_days: i32,
}
