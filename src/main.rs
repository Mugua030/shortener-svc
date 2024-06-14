use anyhow::{Ok, Result};
use tokio::net::TcpListener;
use axum::{
    routing::{get, post}, Router
};
use std::sync::Arc;

use shortener_svc::handler::*;
use shortener_svc::entity::*;
use shortener_svc::config::*;
use shortener_svc::repo::repo::*;



#[tokio::main]
async fn main() -> Result<()> {
    let app_conf = AppConfig::load()?;

    let listen_addr = format!("127.0.0.1:{}", app_conf.app.port);
    let listener = TcpListener::bind(listen_addr).await?;
    println!("server on {}", app_conf.app.port);

    let db_nsq = format!("postgres://{}:{}@{}:{}/shortener", 
                            app_conf.database.username,
                            app_conf.database.password,
                            app_conf.database.host, 
                            app_conf.database.port);
    let db = DBPostgres::new(db_nsq.as_str()).await?;
    let cache = ZRedis::new();
    let repo = Repo::new(Box::new(db), Box::new(cache));
    
    let state = Arc::new(AppState::try_new(repo).await?);
    let app = Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn app_init() -> Result<Repo> {
    todo!()
}