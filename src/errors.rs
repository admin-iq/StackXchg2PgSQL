use thiserror::Error;

use crate::model;

#[derive(Debug, Error)]
pub enum EtlError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parsing error: {0}")]
    Xml(#[from] model::XmlError),
}
