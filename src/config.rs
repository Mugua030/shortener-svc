use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppDetail {
    pub name: String,
    pub port: u16,
    pub default_log_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppDetail,
    pub database: DBConf,
}

#[derive(Debug, Deserialize)]
pub struct DBConf {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // TODO:: support toml and yaml file
        let conf_dir_default = "../config/config.toml";
        let content = fs::read_to_string(conf_dir_default)?;

        let conf: AppConfig = toml::from_str(&content)?;

        // app config
        let app_detail = AppDetail {
            name: conf.app.name,
            port: conf.app.port,
            default_log_dir: conf.app.default_log_dir,
        }; 

        // database
        let db_cfg = DBConf {
            host: conf.database.host,
            port: conf.database.port,
            username: conf.database.username,
            password: conf.database.password,
        };

        Ok(
            AppConfig{
                app: app_detail,
                database: db_cfg,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    #[test]
    fn it_work() {
        let cfg = AppConfig::load();
        println!("config: {:?}", cfg);
    }
}