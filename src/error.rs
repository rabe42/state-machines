use crate::ids::{NodeId, StateId};
use open_api_matcher::{Value, OpenApiValidationError};
use thiserror::Error;
use std::collections::BTreeMap;

#[derive(Error, Debug)]
pub enum StateChartError {
    #[error("Missing mandatory attribute '{0}'.")]
    MandatoryAttributeMissing(String),
    #[error("Unexpected type provided!")]
    UnexpectedType,
    #[error("{0}")]
    ValidationError(#[from] OpenApiValidationError),
    #[error("NodeId isn't valid.")]
    InvalidNodeId(NodeId),
    #[error("StateId isn't valid.")]
    InvalidStateId(StateId),
    #[error("State chart is no root.")]
    NoRoot,
}
impl StateChartError {
    /// Assigns an error number to every known error. This is mandatory to support international
    /// service and support teams.
    fn error_id(&self) -> i64 {
        match self {
            Self::MandatoryAttributeMissing(_) => 0,
            Self::UnexpectedType => 1,
            Self::ValidationError(_) => 2,
            Self::InvalidNodeId(_) => 3,
            Self::InvalidStateId(_) => 4,
            Self::NoRoot => 5,
        }
    }
}

/// Converts an error into a return value.
impl From<StateChartError> for Value {
    fn from(error: StateChartError) -> Self {
        let mut content: BTreeMap<String, Value> = BTreeMap::new();
        content.insert("id".into(), Value::Integer(error.error_id()));
        content.insert("message".into(), Value::String(format!("{}", error)));
        Value::Object(content)
    }
}
