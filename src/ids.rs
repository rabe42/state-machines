use crate::error::StateChartError;
use open_api_matcher::ValidatedValue;
use regex::Regex;
use uuid::Uuid;

/// A system wide unique Id for a node.
#[derive(Clone, Debug)]
pub struct NodeId(String);
impl NodeId {
    const REGEX: &'static str = r"^scn:///(?P<path>\p{L}[\w\.\-]*(/\w[\w\.\-]*)*)$";

    /// Creates an default node id.
    pub fn default() -> Self {
        NodeId("scn:///de4ult".into())
    }

    /// Creates a new node id from a given node path.
    pub fn new(node_path: &str) -> Self {
        Self(format!("scn:///{node_path}"))
    }

    pub fn path(&self) -> Result<&str, StateChartError> {
        let r = Regex::new(NodeId::REGEX).unwrap();
        let NodeId(id) = self;
        if let Some(captures) = r.captures(id.as_str()) {
            if let Some(path) = captures.name("path") {
                Ok(path.as_str().into())
            } else {
                Err(StateChartError::InvalidNodeId(self.clone()))
            }
        } else {
            Err(StateChartError::InvalidNodeId(self.clone()))
        }
    }
}

impl TryFrom<&ValidatedValue> for NodeId {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::String(id) = value {
            Ok(NodeId(id.into()))
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// Convenience trait to transform Strings to NodeIds.
impl From<String> for NodeId {
    fn from(value: String) -> Self {
        NodeId(value)
    }
}

/// Convenience trait to transfor &str to NodeIds.
impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        NodeId(value.into())
    }
}

/// The state holds a reference to the root and the current state of a state chart.
/// @see StateMachines.yml
/// pattern: '^sms:///\w[\w\.\-]*(/\w[\w\.\-]*)*$'
#[derive(Clone, Debug)]
pub struct StateId(String);
impl StateId {
    const REGEX: &'static str = r"^sms:///(?P<id>\w[\w\.-]*)(/\w[\w\.-]*)*$";

    pub fn default() -> Self {
        StateId("sms:///de4ult".into())
    }

    /// Creates a new state id from the provided node id. The assumption is, that the node id is
    /// the id of the state chart.
    pub fn new(node_id: &NodeId) -> Result<Self, StateChartError> {
        let id = format!("sms:///{}/{}", Uuid::new_v4(), node_id.path()?);
        Ok(StateId(id))
    }

    /// Creates a new state id from a given state id and a node id. The assumption is, that a
    /// substate of the state machine will be addressed.
    pub fn new_with_node(machine: &StateId, node_id: &NodeId) -> Result<Self, StateChartError> {
        Ok(StateId(format!(
            "sms:///{}/{}",
            machine.id()?,
            node_id.path()?
        )))
    }

    fn id(&self) -> Result<String, StateChartError> {
        let r = Regex::new(StateId::REGEX).unwrap();
        let StateId(id) = self;
        if let Some(captures) = r.captures(id.as_str()) {
            if let Some(path) = captures.name("id") {
                Ok(path.as_str().into())
            } else {
                Err(StateChartError::InvalidStateId(self.clone()))
            }
        } else {
            Err(StateChartError::InvalidStateId(self.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id() {
        let node_id: NodeId = "scn:///Simple-Task".into();
        assert_eq!("Simple-Task", node_id.path().unwrap());
        let node_id = NodeId::new("Complex-Task");
        assert_eq!("Complex-Task", node_id.path().unwrap());
    }

    #[test]
    fn test_state_id() {
        let node_id = NodeId::new("Complex-Task");
        let state_id = StateId::new(&node_id).unwrap();
        let _uuid = state_id.id().unwrap();
    }
}
