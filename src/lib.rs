pub mod cli;
pub mod storage;
pub mod commands;
pub mod utils;

pub(crate) type Result<T> = anyhow::Result<T>;
