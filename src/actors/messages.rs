use crate::diesel::prelude::*;
use crate::models::{NewTour, Tour, TourData};
use actix::Message;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<Tour>")]
pub struct Create {
    pub name: String,
    pub src: String,
    pub dst: String,
    pub total_days: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Tour>")]
pub struct Update {
    pub id: Uuid,
    pub name: String,
    pub src: String,
    pub dst: String,
    pub total_days: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Tour>")]
pub struct Delete {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Tour>>")]
pub struct GetAll;

impl From<TourData> for Create {
    fn from(new_tour: TourData) -> Self {
        Self {
            name: new_tour.name,
            src: new_tour.src,
            dst: new_tour.dst,
            total_days: new_tour.total_days,
        }
    }
}

impl Into<NewTour> for Create {
    fn into(self) -> NewTour {
        NewTour {
            id: Uuid::new_v4(),
            name: self.name,
            src: self.src,
            dst: self.dst,
            total_days: self.total_days,
        }
    }
}
