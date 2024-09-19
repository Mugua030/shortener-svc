use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::IntoResponse, Json,
    debug_handler,
};
use http::{header::LOCATION,HeaderMap,StatusCode};
use tracing::{info, warn};
use url::Url;
use std::sync::Arc;
use crate::entity::*;
use crate::AppError;


#[debug_handler]
pub async fn shorten(
    State(state): State<Arc<AppState>>,
    Json(data): Json<ShortenReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("run here");
    let idret = state.shorten(&data.url).await;
    let id = match idret {
        Ok(v) => v,
        Err(_) => return Err(AppError::ParamErr("fail".to_string())),
    };
    //let id = "";
    println!("randon-id: {}", &id);

    let host_url = if let Ok(parsed_url) = Url::parse(&data.url) {
        parsed_url.host_str().map(|x| x.to_string()).unwrap_or_default()
    } else {
        String::new()
    };

    println!("[gen shorten url] {}", format!("{}/{}", &host_url, &id));

    let body = Json(ShortenRes {
        url: format!("{}/{}", host_url, id),
    });


    Ok((StatusCode::CREATED, body))
}

pub async fn redirect(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {

    let url = state.get_url(&id).await?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());

    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}