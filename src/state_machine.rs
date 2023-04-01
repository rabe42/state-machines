use crate::state_charts::Node;

/// The state holds a reference to the root and the current state of a state chart.
/// @see StateMachines.yml
type StateId = String;

/// A state machine is a running state chart.
pub struct StateMachine {
    /// Contains a complete copy of the
    state_chart: Node,
    root_state: StateId,
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
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_state_machine_new() {
        todo!()
    }
}
