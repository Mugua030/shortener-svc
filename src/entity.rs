use anyhow::Result;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::repo::repo::*;
use crate::error::AppError;
use std::pin::Pin;
//use std::future::Future;
use futures::future::{BoxFuture, FutureExt};

use async_trait::async_trait;

#[derive(Clone)]
pub struct AppState {
    repo: Repo,
}

#[derive(Debug, Deserialize)]
pub struct ShortenReq {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenRes {
    pub url: String,
}

#[async_trait]
pub trait ShortenService {
    async fn shorten(&self, url: &str) -> Result<String, AppError>;
}

#[async_trait]
impl ShortenService for AppState {
    async fn shorten(&self, url: &str) -> Result<String, AppError> {
        let retry_times = 3;
        let ret = self.record_one(retry_times, url).await;
        ret
        //Ok(ret)
    }
}

impl AppState {
    pub async fn try_new(repo: Repo) -> Result<Self> {
        Ok(AppState {
            repo,
        })
    }


    //async fn record_one<'b>(&'b self, retry_times: i32, url: &str) -> Result<String, AppError> {
    fn record_one<'a>(&'a self, retry_times: i32, url: &'a str) ->  BoxFuture<'a, Result<String, AppError>>{
        async move {
            if retry_times <= 0 {
                return Err(AppError::RetryLimitReached);
            }

            // store the sid to database 
            let sid = nanoid!(6);
            let ret = self.repo.db.create(sid.clone(), url.into()).await;
            match ret {
                Ok(rid) => Ok(rid),
                Err(sqlx_err) => {
                    if let Some(db_err) = sqlx_err.as_database_error() {
                        if db_err.is_unique_violation() {
                            return self.record_one(retry_times - 1, url).await;
                        }
                    } 
                    Err(AppError::SqlxError(sqlx_err))
                }
            }

        }.boxed()
    }

    pub async fn get_url(&self, id: &str) -> Result<String, AppError> {
        let ret = self.repo.db.fetch_one(id).await?;

        Ok(ret)
    }
}

// db data
#[derive(FromRow)]
pub struct UrlRecord {
    #[sqlx(default)]
    pub id: String,
    #[sqlx(default)]
    pub url: String,
}