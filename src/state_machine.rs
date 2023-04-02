use std::time::SystemTime;
use open_api_matcher::OpenApi;

use crate::state_charts::{ Node, NodeId, VariableId };

/// The state holds a reference to the root and the current state of a state chart.
/// @see StateMachines.yml
type StateId = String;

/// A state machine is a running state chart.
pub struct StateMachine {
    /// Contains a complete copy of the root node of the state chart
    state_chart: Node,
    /// This *MUST* be the id of the root state.
    root_state: StateId,
    /// This is the id of the current state.
    current_state: StateId,
}
impl StateMachine {

    /// Creates a new state machine, based on the provided state chart.
    fn new(state_chart: &Node) -> StateMachine
    {
        StateMachine {
            state_chart: state_chart.clone(),
            root_state: "".into(),
            current_state: "".into(),
        }
    }

    // TODO: The operations regarding the setting of variables and events goes here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_new() {
        // Read a state chart.
        let open_api_file = std::fs::File::open("StateMachines.yml").unwrap();
        let open_api = OpenApi::new(&open_api_file).unwrap();
        let sc = std::fs::read_to_string("tests/simple-task.json").unwrap();
        // TODO: Create a state machine from it.
        // TODO: Check, that the root_state is set correctly.
        // TODO: Check, that the current_state is set correctly.
    }
}
