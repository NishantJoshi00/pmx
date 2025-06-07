pub mod cli;
pub mod commands;
pub mod storage;
pub mod utils;

pub(crate) type Result<T> = anyhow::Result<T>;
