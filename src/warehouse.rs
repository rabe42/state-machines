use crate::{
    ids::{NodeId, StateId},
    state_charts::Node,
    state_machine::StateMachine,
};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
enum WarehouseError {
    #[error("Cannot save state chart!")]
    CannotSaveStateChart,
}

/// The warehouse is the abstraction of the persistence level.
struct Warehouse {
    state_charts: HashMap<NodeId, Node>,
    state_machines: HashMap<StateId, StateMachine>,
}
impl Warehouse {
    pub fn new() -> Warehouse {
        Warehouse {
            state_charts: HashMap::new(),
            state_machines: HashMap::new(),
        }
    }

    pub fn get_all_state_charts(&self) -> Vec<Node> {
        self.state_charts.values().cloned().collect()
    }

    pub fn get_state_chart(&self, node_id: &NodeId) -> Option<&Node> {
        self.state_charts.get(node_id)
    }

    pub fn save_state_chart(&mut self, node: Node) -> Result<(), WarehouseError> {
        self.state_charts.insert(node.id().clone(), node);
        Ok(())
    }

    pub fn get_state_machine(&self, sm_id: &StateId) -> Option<&StateMachine> {
        self.state_machines.get(sm_id)
    }

    pub fn save_state_machine(&mut self, machine: StateMachine) -> Result<(), WarehouseError> {
        self.state_machines.insert(machine.id().clone(), machine);
        Ok(())
    }
}
