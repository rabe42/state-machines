use crate::ids::NodeId;
use crate::state_charts::VariableId;
use std::time::SystemTime;

/// The log of all events, variable changes and node changes of a particular state machine.
#[allow(dead_code)]
pub struct StateMachineLog {
    // Database connection...
}
#[allow(dead_code)]
impl StateMachineLog {
    /// Initializes the log with the connection to the log store.
    pub fn init() {}

    /// Creates a new log entry.
    pub fn log(_entry: LogEntryType) {}
}

/// The log entry will document when a operation was conducted.
#[allow(dead_code)]
struct StateMachineLogEntry {
    timestamp: SystemTime,
    entry: LogEntryType,
}

/// The relevant entry types.
#[allow(dead_code)]
pub enum LogEntryType {
    Event(String),
    VariableSetting(VariableId, String),
    Transaction(NodeId, NodeId),
}

#[cfg(test)]
mod tests {}
