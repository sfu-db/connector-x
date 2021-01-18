use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

#[derive(Error, Debug)]
pub enum ConnectorAgentError {}
