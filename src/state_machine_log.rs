use std::system::SystemTime;
use crate::state_chart::{VariableId, NodeId};

/// The log of all events, variable changes and node changes of a particular state machine.
pub struct StateMachineLog {
}

/// The log entry will document when a operation was conducted.
pub struct StateMachineLogEntry {
    timestamp: SystemTime,
    entry: LogEntryType,
}

/// The relevant entry types.
pub enum LogEntryType {
    Event(String),
    VariableSetting(VariableId, String),
    Transaction(NodeId, NodeId),
}

#[cfg(test)]
mod tests {

}
