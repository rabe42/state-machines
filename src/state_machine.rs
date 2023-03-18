use crate::state_charts::Node;

type StateId = String;

/// A state machine is a running state chart.
struct StateMachine {
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
            state_chart: *state_chart.clone(),
            root_state: "".into(),
            current_state: "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
}
