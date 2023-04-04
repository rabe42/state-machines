use crate::error::StateChartError;
use crate::ids::StateId;
use crate::state_charts::Node;

/// A state machine is a running state chart.
pub struct StateMachine {
    /// This is the id of the root state. Constructed from an unique name of the machine and the id
    /// of the state chart.
    id: StateId,
    /// Contains a complete copy of the state chart
    state_chart: Node,
    /// This is the id of the current state.
    current_state: StateId,
}
impl StateMachine {
    /// Creates a new state machine, based on the provided state chart.
    pub fn new(state_chart: Node) -> Result<StateMachine, StateChartError> {
        let id = StateId::new(state_chart.id())?;
        if let Some(start_node) = state_chart.start_node() {
            let current_state = StateId::new_with_node(&id, &start_node)?;
            Ok(StateMachine {
                id,
                state_chart,
                current_state,
            })
        } else {
            Err(StateChartError::NoRoot)
        }
    }

    // TODO: generate a StateId from the NodeId of the state chart.
    // TODO: The operations regarding the setting of variables and events goes here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_api_matcher::{OpenApi, ValidatedValue};

    #[test]
    fn test_extract_node_path() {
        let open_api_file = std::fs::File::open("StateMachines.yml").unwrap();
        let open_api = OpenApi::new(&open_api_file).unwrap();
        let sc = std::fs::read_to_string("tests/simple-task.json").unwrap();
        let sc_schema = open_api.get_schema("#/components/schemas/Node").unwrap();
        let vvsc = ValidatedValue::new(&sc, &sc_schema, &open_api).unwrap();
        let node: Node = (&vvsc).try_into().unwrap();
        let _state_machine = StateMachine::new(node).unwrap();
    }

    #[test]
    fn test_state_machine_new() {
        // Read a state chart.
        let open_api_file = std::fs::File::open("StateMachines.yml").unwrap();
        let _open_api = OpenApi::new(&open_api_file).unwrap();
        let _sc = std::fs::read_to_string("tests/simple-task.json").unwrap();
        // TODO: Create a state machine from it.
        // TODO: Check, that the root_state is set correctly.
        // TODO: Check, that the current_state is set correctly.
    }
}
