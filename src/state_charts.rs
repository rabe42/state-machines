use std::collections::BTreeMap;
use open_api_matcher::ValidatedValue;
use crate::error::StateChartError;

/// The node is the heard of the state chart definition. A node can be a single state or a state
/// chart of its own.
pub struct Node {
    id: NodeId,
    name: String,
    description: String,
    on_entry: ActionCall,
    on_exit: ActionCall,
    start_node: Box<Node>,
    out_transitions: Vec<Transition>,
    attributes: Vec<VariableDeclarations>,
}
impl Node {
    pub fn from_validate_value(object: &ValidatedValue) -> Result<Self, StateChartError> {
        if let ValidatedValue::Object(attributes) = object {

            // let vid = get_mandatory_attribute(attributes, "Id")?;
            // let id: String = vid.try_into()?;
            // return Ok(Node::default());
            let id = get_mandatory(attributes, "id")?;
            let id: String = id.try_into()?;
            let name = get_mandatory(attributes, "name")?;
            let name: String = name.try_into()?;

            Ok(Node {
                id,
                name,
                description: get_mandatory(attributes, "description")?.try_into()?,
                on_entry: get_mandatory(attributes, "on_entry")?.try_into()?,
                on_exit: get_mandatory(attributes, "on_exit")?.try_into()?,
                start_node: Box::new(Node::from_validate_value(
                        get_mandatory(attributes, "start_node")?)?),
                out_transitions: Node::transitions_from_validated_value(get_mandatory(attributes, "out_transitions")?)?,
                attributes: Node::attributes_from_validated_value(get_mandatory(attributes, "attributes")?)?,
            })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }

    fn transitions_from_validated_value(vtransitions: &ValidatedValue)
        -> Result<Vec<Transition>, StateChartError>
    {
        Ok(Vec::new())
    }

    fn attributes_from_validated_value(vattributes: &ValidatedValue)
        -> Result<Vec<VariableDeclarations>, StateChartError>
    {
        Ok(Vec::new())
    }
}

/// Retrieves a mandatory attribute from a standard map.
fn get_mandatory<'a>(attributes: &'a BTreeMap<String, ValidatedValue>, name: &str) 
    -> Result<&'a ValidatedValue, StateChartError>
{
    if let Some(attribute) = attributes.get(name) {
        Ok(attribute)
    }
    else {
        Err(StateChartError::MandatoryAttributeMissing(name.into()))
    }
}

/// A system wide unique Id for a node. The for is defined in the OpenApi specification.
type NodeId = String;

pub struct ActionCall {
    name: ActionId,
    parameters: Vec<Parameter>,
}
impl TryFrom<&ValidatedValue> for ActionCall {
    type Error = StateChartError;

    /// Creates an ActionCall from the provided validated value.
    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            
            let name: String = get_mandatory(attributes, "name")?.try_into()?;
            let parameters: Vec<Parameter> = parameters_from_validated_values(get_mandatory(attributes, "parameters")?)?;
            Ok(ActionCall { name, parameters })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

fn parameters_from_validated_values(values: &ValidatedValue)
    -> Result<Vec<Parameter>, StateChartError>
{
    Ok(Vec::new())
}

type ActionId = String;

pub struct Parameter {
    name: VariableId,
    value: VariableValue,
}
impl TryFrom<&ValidatedValue> for Parameter {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            let name: String = get_mandatory(attributes, "name")?.try_into()?;
            let value: VariableValue = get_mandatory(attributes, "value")?.try_into()?;
            Ok(Self { name, value })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

type VariableId = String;

pub struct Transition {
    /// The check, which tests, if the transaction is activated.
    guard: Guard,
    /// The node, we reach after the transaction is activated.
    to: NodeId,
    /// The action called, if the transition is activated.
    action: ActionCall,
}

pub enum Guard {
    Event(EventId),
    Predicate(PredicateCall),
}

type EventId = String;

pub struct PredicateCall {
    name: PredicateId,
    parameters: Vec<Parameter>,
}

type PredicateId = String;

#[derive(Default)]
pub struct VariableDeclarations {
    name: String,
    r#type: String,
    value: VariableValue,
}

pub enum VariableValue {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    None,
}
impl Default for VariableValue {
    fn default() -> Self {
        Self::None
    }
}
impl TryFrom<&ValidatedValue> for VariableValue {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        match value {
            ValidatedValue::String(s) => Ok(Self::String(s.into())),
            ValidatedValue::Integer(i) => Ok(Self::Integer(*i)),
            ValidatedValue::Number(n) => Ok(Self::Number(*n)),
            ValidatedValue::Bool(b) => Ok(Self::Boolean(*b)),
            ValidatedValue::None => Ok(Self::None),
            _ => Err(StateChartError::UnexpectedType),
        }
    }
}
