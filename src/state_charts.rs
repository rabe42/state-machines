use std::collections::BTreeMap;
use open_api_matcher::ValidatedValue;
use crate::error::StateChartError;

/// A system wide unique Id for a node.
type NodeId = String;

/// A system wide unique Id of a action.
type ActionId = String;

/// A system wide unique Id of a variable.
type VariableId = String;

/// A system wide unique Id of a event.
type EventId = String;

/// A system wide unique Id of a predicate.
type PredicateId = String;

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
/// Constructs a node from the validated value.
impl TryFrom<&ValidatedValue> for Node {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        if let ValidatedValue::Object(attributes) = value {

            let id = get_mandatory(&attributes, "id")?;
            let id: String = id.try_into()?;
            let name = get_mandatory(&attributes, "name")?;
            let name: String = name.try_into()?;

            Ok(Node {
                id,
                name,
                description: get_mandatory(&attributes, "description")?.try_into()?,
                on_entry: get_mandatory(&attributes, "on_entry")?.try_into()?,
                on_exit: get_mandatory(&attributes, "on_exit")?.try_into()?,
                start_node: Box::new(get_mandatory(&attributes, "start_node")?.try_into()?),
                out_transitions: transitions_from_validated_value(get_mandatory(attributes, "out_transitions")?)?,
                attributes: attributes_from_validated_value(get_mandatory(attributes, "attributes")?)?,
            })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

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

pub struct Transition {
    /// The check, which tests, if the transaction is activated.
    guard: Guard,
    /// The node, we reach after the transaction is activated.
    to: NodeId,
    /// The action called, if the transition is activated.
    action: ActionCall,
}

/// The guard on a trasition holds the condition under which a transaction is activated.
/// It will be evaluated by the state machine runtime.
pub enum Guard {
    Event(EventId),
    Predicate(PredicateCall),
}

/// The call of a predicate may be a guard. The predicate of all transactions of the current state
/// will be evaluated when ever a variable value was modified.
pub struct PredicateCall {
    name: PredicateId,
    parameters: Vec<Parameter>,
}

/// Declares a variable inside of a state chart state.
#[derive(Default)]
pub struct VariableDeclarations {
    name: String,
    r#type: String,
    value: VariableValue,
}

/// The variable value holds the value of a variable attribute or parameter.
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

/// Derives a vector of parameters from an array of validated values.
fn parameters_from_validated_values(values: &ValidatedValue)
    -> Result<Vec<Parameter>, StateChartError>
{
    if let ValidatedValue::Array(values) = values {
        let mut result = Vec::new();
        for v_param in values {
            let param: Parameter = v_param.try_into()?;
            result.push(param);
        }
        Ok(result)
    }
    else {
        Err(StateChartError::UnexpectedType)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_mandatory() {
        let mut attributes = BTreeMap::new();
        get_mandatory(&attributes, "name").is_err();
        attributes.insert("name".into(), ValidatedValue::String("Lucky".into()));
        get_mandatory(&attributes, "name").unwrap();
    }

    #[test]
    fn test_parameter_try_into() {

        let mut parameter_a: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_a.insert("name".into(), ValidatedValue::String("A".into()));
        parameter_a.insert("value".into(), ValidatedValue::Bool(true));

        let parameter: Parameter = (&ValidatedValue::Object(parameter_a)).try_into().unwrap();
    }

    #[test]
    fn test_parameters_from_vv() {
        let mut parameter_a: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_a.insert("name".into(), ValidatedValue::String("A".into()));
        parameter_a.insert("value".into(), ValidatedValue::Bool(true));
        let mut parameter_b: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_a.insert("name".into(), ValidatedValue::String("B".into()));
        parameter_a.insert("value".into(), ValidatedValue::Bool(false));
        let mut v_parameters: Vec<ValidatedValue> = Vec::new();
        v_parameters.push(ValidatedValue::Object(parameter_a));
        v_parameters.push(ValidatedValue::Object(parameter_b));

        // Check, if the paramters can be extracted as exprected.
        let parameters = parameters_from_validated_values(&ValidatedValue::Array(v_parameters)).unwrap();
        assert_eq!(2, parameters.len());
    }

}
