use actix::{Addr, SyncArbiter};
use actix_web::{
    delete, get,
    middleware::Logger,
    post, put,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};

#[macro_use]
extern crate diesel;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use dotenv::dotenv;
use std::env;
use uuid::Uuid;

mod actors;
mod models;
mod schema;
mod utils;

use crate::actors::{Create, Delete, GetAll, Update};
use actors::DbActor;
use models::TourData;

/// State used in the router
pub struct AppState {
    /// Address to the DbActor
    /// Used when trying to commiunicate with the Actor
    pub db: Addr<DbActor>,
}

#[post("/new")]
async fn create_tour(Json(tour): Json<TourData>, state: Data<AppState>) -> impl Responder {
    let db = &state.db;
    match db.send(Create::from(tour)).await {
        Ok(Ok(tour)) => HttpResponse::Ok().json(tour),
        _ => HttpResponse::InternalServerError().json("couldn't insert new tour"),
    }
}

#[put("/{id}")]
async fn update_tour(
    Path(id): Path<Uuid>,
    Json(tour): Json<TourData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = &state.db;
    match db
        .send(Update {
            id,
            name: tour.name,
            src: tour.src,
            dst: tour.dst,
            total_days: tour.total_days,
        })
        .await
    {
        Ok(Ok(tour)) => HttpResponse::Ok().json(tour),
        _ => HttpResponse::InternalServerError().json("couldn't update tour {id}"),
    }
}

#[delete("/{id}")]
async fn delete_tour(Path(id): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = &state.db;
    match db.send(Delete { id }).await {
        Ok(Ok(tour)) => HttpResponse::Ok().json(tour),
        _ => HttpResponse::InternalServerError().json("couldn't delete tour {id}"),
    }
}

#[get("/")]
async fn get_all_tours(state: Data<AppState>) -> impl Responder {
    let db = &state.db;
    match db.send(GetAll {}).await {
        Ok(Ok(tours)) => HttpResponse::Ok().json(tours),
        _ => HttpResponse::InternalServerError().json("couldn't retrieve tours"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db_url = env::var("DATABASE_URL").unwrap();
    let db_actor_concurrency: usize = env::var("API_PORT").unwrap().parse().unwrap();

    let api_port: u16 = env::var("API_PORT").unwrap().parse().unwrap();
    let api_domain = env::var("API_DOMAIN").unwrap();

    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(db_url))
        .expect("couldn't create DB pool");

    let db_addr = SyncArbiter::start(db_actor_concurrency, move || DbActor { pool: pool.clone() });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db: db_addr.clone(),
            })
            .wrap(Logger::default())
            .service(create_tour)
            .service(update_tour)
            .service(delete_tour)
            .service(get_all_tours)
    })
    .bind((api_domain, api_port))?
    .run()
    .await
}
