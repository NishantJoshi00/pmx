pub mod cli;
pub mod storage;
pub mod commands;

pub(crate) type Result<T> = anyhow::Result<T>;
