use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use crate::config::config::Data;

pub mod config;
pub mod commands;

pub struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<Data>;
}
