use anyhow::Result;
use std::error::Error;
use sqlx::{FromRow, PgPool};
use async_trait::async_trait;
use crate::entity::*;
use crate::error::AppError;

#[derive(Clone)]
pub struct Repo {
    pub db: Box<dyn DBStore>,
    pub cache: Box<dyn Cache>,
}

impl Repo {
    pub fn new(db: Box<dyn DBStore>, cache: Box<dyn Cache>) -> Self {
        Repo {
            db,
            cache,
        }
    }
}

// database
#[async_trait]
pub trait DBStore: CloneableDBStore + Send + Sync {
    async fn create(&self, id: String, url: String) -> Result<String, sqlx::Error>;
    async fn update(&self, sql: &str) -> Result<bool, sqlx::Error>;
    async fn delete(&self, sql: &str) -> Result<bool, sqlx::Error>;
    async fn fetch_one(&self, id: &str) -> Result<String, sqlx::Error>;
}

pub trait CloneableDBStore: Send + Sync {
    fn clone_box(&self) -> Box<dyn DBStore>;
}

impl<T> CloneableDBStore for T
where T: 'static + DBStore + Clone + Send + Sync {
    fn clone_box(&self) -> Box<dyn DBStore> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DBStore> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct DBPostgres {
    db: PgPool,
}

impl DBPostgres {
    pub async fn new(dns_url: &str) -> Result<Self, anyhow::Error> {
        let pool = PgPool::connect(dns_url).await?;

        Ok(Self {
            db: pool,
        })
    }
}

#[async_trait]
impl DBStore for DBPostgres {
    async fn create(&self, id: String, url: String) -> Result<String, sqlx::Error> {
        let sql = "INSERT INTO urls (id, url) VALUES ($1, $2) ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id";
        let ret: UrlRecord = sqlx::query_as(sql)
            .bind(&id)
            .bind(url)
            .fetch_one(&self.db)
            .await?;

        Ok(ret.id)
    }

    async fn delete(&self, sql: &str) -> Result<bool, sqlx::Error> {
        todo!()
    }

    async fn update(&self, sql: &str) -> Result<bool, sqlx::Error> {
        todo!()
    }

    async fn fetch_one(&self, id: &str) -> Result<String, sqlx::Error> {
        let ret: UrlRecord = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(ret.id)
    }
}

// Cache : redis or memcache
pub trait Cache: CloneableCache + Send + Sync {
    fn set(&self, key: &str, value: String) -> Result<u8, Box<dyn Error>>;
    fn get(&self, key: &str) -> Result<String, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct ZRedis {}

impl ZRedis {
    pub fn new() -> Self {
        ZRedis {}
    }
}

impl Cache for ZRedis {
    fn set(&self, key: &str, value: String) -> Result<u8, Box<dyn Error>> {
        todo!()
    }

    fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

pub trait CloneableCache: Send {
    fn clone_box(&self) -> Box<dyn Cache>;
}

impl<T> CloneableCache for T
where T: 'static + Cache + Clone + Send + Sync {
    fn clone_box(&self) -> Box<dyn Cache> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Cache> {
    fn clone(&self) -> Box<dyn Cache> {
        self.clone_box()
    }
}