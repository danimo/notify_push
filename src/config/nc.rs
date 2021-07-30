use crate::config::PartialConfig;
use color_eyre::{eyre::WrapErr, Result};
use nextcloud_config_parser::{parse, parse_glob};
use std::path::Path;

pub(super) fn parse_config_file(path: impl AsRef<Path>, glob: bool) -> Result<PartialConfig> {
    let config = if glob { parse_glob(path) } else { parse(path) }
        .wrap_err("Failed to parse nextcloud config")?;

    Ok(PartialConfig {
        database: Some(config.database.into()),
        database_prefix: Some(config.database_prefix),
        nextcloud_url: Some(config.nextcloud_url),
        redis: config.redis.into_vec(),
        ..PartialConfig::default()
    })
}
