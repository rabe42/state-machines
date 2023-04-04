use crate::ids::{NodeId, StateId};
use open_api_matcher::OpenApiValidationError;
use thiserror::Error;

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
