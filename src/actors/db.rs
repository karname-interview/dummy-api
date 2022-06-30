use crate::diesel::prelude::*;
use crate::models::{NewTour, Tour};
use crate::schema::tours::dsl::{self as tours_column, tours};
use actix::{Actor, Handler, SyncContext};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use super::{Create, Delete, GetAll, Update};

/// Actor for comminiucating with DataBase
pub struct DbActor {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

// handle impls for Message types

impl Handler<Create> for DbActor {
    type Result = QueryResult<Tour>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self
            .pool
            .get()
            .expect("unable to retrieve connection from database");
        let new_tour: NewTour = msg.into();
        diesel::insert_into(tours)
            .values(new_tour)
            .get_result(&conn)
    }
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Tour>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self
            .pool
            .get()
            .expect("unable to retrieve connection from database");
        diesel::update(tours)
            .filter(tours_column::id.eq(msg.id))
            .set((
                tours_column::src.eq(msg.src),
                tours_column::dst.eq(msg.dst),
                tours_column::name.eq(msg.name),
                tours_column::total_days.eq(msg.total_days),
            ))
            .get_result(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Tour>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self
            .pool
            .get()
            .expect("unable to retrieve connection from database");
        diesel::delete(tours)
            .filter(tours_column::id.eq(msg.id))
            .get_result(&conn)
    }
}

impl Handler<GetAll> for DbActor {
    type Result = QueryResult<Vec<Tour>>;

    fn handle(&mut self, _: GetAll, _: &mut Self::Context) -> Self::Result {
        let conn = self
            .pool
            .get()
            .expect("unable to retrieve connection from database");
        tours.get_results(&conn)
    }
}
