use anyhow::Result;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::repo::repo::*;

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

impl<'a> AppState {
    pub async fn try_new(repo: Repo) -> Result<Self> {
        Ok(AppState {
            repo,
        })
    }

    pub async fn shorten(&self, url: &str) -> Result<String, anyhow::Error> {
        let sid = nanoid!(6);
        // store the sid to database 
        let rid = self.repo.db.create(sid, url.into()).await?;

        // return the sid
        Ok(rid)
    }

    pub async fn get_url(&self, id: &str) -> Result<String, anyhow::Error> {
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