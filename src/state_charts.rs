use std::collections::BTreeMap;
use open_api_matcher::ValidatedValue;
use crate::error::StateChartError;

/// A system wide unique Id for a node.
pub type NodeId = String;

/// A system wide unique Id of a action.
pub type ActionId = String;

/// A system wide unique Id of a variable.
pub type VariableId = String;

/// A system wide unique Id of a event.
pub type EventId = String;

/// A system wide unique Id of a predicate.
pub type PredicateId = String;

/// The node is the heart of the state chart definition. A node can be a single state or a state
/// chart of its own.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Node {
    name: String,
    description: Option<String>,
    on_entry: Option<ActionCall>,
    on_exit: Option<ActionCall>,
    start_node: Box<Node>,
    out_transitions: Vec<Transition>,
    attributes: Vec<VariableDeclaration>,
    nodes: Vec<Node>,
}
/// Constructs a node from the validated value.
impl TryFrom<&ValidatedValue> for Node {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        if let ValidatedValue::Object(attributes) = value {
            let on_entry = match attributes.get("on-entry") {
                Some(vac) => {
                    let ac: ActionCall = vac.try_into()?;
                    Some(ac)
                },
                None => None,
            };
            let on_exit = match attributes.get("on-exit") {
                Some(vac) => {
                    let ac: ActionCall = vac.try_into()?;
                    Some(ac)
                },
                None => None,
            };
            let description = match attributes.get("description") {
                Some(vd) => {
                    let d: String = vd.try_into()?;
                    Some(d)
                },
                None => None,
            };
            Ok(Node {
                name: get_mandatory(&attributes, "name")?.try_into()?,
                description,
                on_entry,
                on_exit,
                start_node: Box::new(get_mandatory(&attributes, "start_node")?.try_into()?),
                out_transitions: transitions_from_validated_value(
                    get_mandatory(attributes, "out_transitions")?)?,
                attributes: attributes_from_validated_value(attributes.get("attributes"))?,
                nodes: nodes_from_validated_value(attributes.get("nodes"))?,
            })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ActionCall {
    name: ActionId,
    parameters: Vec<Parameter>,
}
impl TryFrom<&ValidatedValue> for ActionCall {
    type Error = StateChartError;

    /// Creates an ActionCall from the provided validated value.
    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            let name: ActionId = get_mandatory(attributes, "name")?.try_into()?;
            let parameters: Vec<Parameter> = parameters_from_validated_values(get_mandatory(attributes, "parameters")?)?;
            Ok(ActionCall { name, parameters })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
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

/// The transition from one node to another.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Transition {
    /// The check, which tests, if the transaction is activated.
    guard: Guard,
    /// The node, we reach after the transaction is activated.
    to: NodeId,
    /// The action called, if the transition is activated.
    action: Option<ActionCall>,
}
impl TryFrom<&ValidatedValue> for Transition {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        if let ValidatedValue::Object(attributes) = value {
            let guard = get_mandatory(attributes, "guard")?.try_into()?;
            let to = get_mandatory(attributes, "to")?.try_into()?;
            let action = match attributes.get("action") {
                None => None,
                Some(v_action) => {
                    let ac: ActionCall = v_action.try_into()?;
                    Some(ac)
                },
            };
            Ok(Transition { guard, to, action })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// The guard on a trasition holds the condition under which a transaction is activated.
/// It will be evaluated by the state machine runtime.
#[allow(dead_code)]
#[derive(Clone)]
pub enum Guard {
    Event(EventId),
    Predicate(PredicateCall),
}

impl TryFrom<&ValidatedValue> for Guard {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        if let ValidatedValue::String(event_id) = value {
            Ok(Guard::Event(event_id.into()))
        }
        else if let ValidatedValue::Object(_) = value {
            let predicate_call = value.try_into()?;
            Ok(Guard::Predicate(predicate_call))
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// The call of a predicate may be a guard. The predicate of all transactions of the current state
/// will be evaluated when ever a variable value was modified.
#[allow(dead_code)]
#[derive(Clone)]
pub struct PredicateCall {
    name: PredicateId,
    parameters: Vec<Parameter>,
}
impl TryFrom<&ValidatedValue> for PredicateCall
{
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error>
    {
        if let ValidatedValue::Object(attributes) = value {
            let name: PredicateId = get_mandatory(attributes, "name")?.try_into()?;
            let parameters = parameters_from_validated_values(get_mandatory(attributes, "parameters")?)?;
            Ok(PredicateCall { name, parameters })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// Declares a variable inside of a state chart state.
#[allow(dead_code)]
#[derive(Clone)]
pub struct VariableDeclaration {
    name: String,
    r#type: String,
    value: VariableValue,
}
impl TryFrom<&ValidatedValue> for VariableDeclaration {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            Ok(VariableDeclaration {
                name: get_mandatory(attributes, "name")?.try_into()?,
                r#type: get_mandatory(attributes, "type")?.try_into()?,
                value: get_mandatory(attributes, "value")?.try_into()?,
            })
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// The variable value holds the value of a variable attribute or parameter.
#[derive(Debug, PartialEq, Clone)]
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

/// Retrieves the transitions of a node from the transition.
fn transitions_from_validated_value(value: &ValidatedValue)
    -> Result<Vec<Transition>, StateChartError>
{
    if let ValidatedValue::Array(transitions) = value {
        let mut result = Vec::new();
        for v_transition in transitions {
            let transition = v_transition.try_into()?;
            result.push(transition)
        }
        Ok(result)
    }
    else {
        Err(StateChartError::UnexpectedType)
    }
}

/// Retrieves the attributes/variables from the array.
fn attributes_from_validated_value(value: Option<&ValidatedValue>)
    -> Result<Vec<VariableDeclaration>, StateChartError>
{
    if let Some(value) = value {
        if let ValidatedValue::Array(attribs) = value {
            let mut result: Vec<VariableDeclaration> = Vec::new();
            for attribute in attribs {
                let vd: VariableDeclaration = attribute.try_into()?;
                result.push(vd);
            }
            Ok(result)
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
    else {
        Ok(Vec::new())
    }
}

fn nodes_from_validated_value(value: Option<&ValidatedValue>)
    -> Result<Vec<Node>, StateChartError>
{
    if let Some(v_value) = value {
        if let ValidatedValue::Array(nodes) = v_value {
            let mut result: Vec<Node> = Vec::new();
            for v_node in nodes {
                let node = v_node.try_into()?;
                result.push(node);
            }
            Ok(result)
        }
        else {
            Err(StateChartError::UnexpectedType)
        }
    }
    else {
        Ok(Vec::new())
    }
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
    use open_api_matcher::OpenApi;

    #[test]
    fn test_get_mandatory() {
        let mut attributes = BTreeMap::new();
        get_mandatory(&attributes, "name").unwrap_err();
        attributes.insert("name".into(), ValidatedValue::String("Lucky".into()));
        get_mandatory(&attributes, "name").unwrap();
    }

    #[test]
    fn test_parameter_try_into() {
        let mut parameter_a: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_a.insert("name".into(), ValidatedValue::String("A".into()));
        parameter_a.insert("value".into(), ValidatedValue::Bool(true));

        let _parameter: Parameter = (&ValidatedValue::Object(parameter_a)).try_into().unwrap();
    }

    #[test]
    fn test_guard_try_into() {
        let event_id = ValidatedValue::String("sme:///open".into());
        let _guard: Guard = (&event_id).try_into().unwrap();

        let mut pc_attributes: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        pc_attributes.insert("name".into(), ValidatedValue::String("A".into()));
        pc_attributes.insert("parameters".into(), ValidatedValue::Array(Vec::new()));
        let pc = ValidatedValue::Object(pc_attributes);
        let _guard: Guard = (&pc).try_into().unwrap();
    }

    #[test]
    fn test_parameters_from_vv() {
        let mut parameter_a: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_a.insert("name".into(), ValidatedValue::String("A".into()));
        parameter_a.insert("value".into(), ValidatedValue::Bool(true));
        let mut parameter_b: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        parameter_b.insert("name".into(), ValidatedValue::String("B".into()));
        parameter_b.insert("value".into(), ValidatedValue::Bool(false));
        let mut v_parameters: Vec<ValidatedValue> = Vec::new();
        v_parameters.push(ValidatedValue::Object(parameter_a));
        v_parameters.push(ValidatedValue::Object(parameter_b));

        // Check, if the paramters can be extracted as exprected.
        let parameters = parameters_from_validated_values(&ValidatedValue::Array(v_parameters)).unwrap();
        assert_eq!(2, parameters.len());
    }

    #[test]
    fn test_variable_declaration_try_into() {
        let mut vd1_bt: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        vd1_bt.insert("name".into(), ValidatedValue::String("var1".into()));
        vd1_bt.insert("type".into(), ValidatedValue::String("integer".into()));
        vd1_bt.insert("value".into(), ValidatedValue::Integer(1));
        let vd1 = ValidatedValue::Object(vd1_bt);
        let _vd: VariableDeclaration = (&vd1).try_into().unwrap();
    }

    #[test]
    fn test_attributes_from_vv() {
        // Create a list of ValidatedValues with objects with the attributes of the
        // VariableDeclaration
        let mut vd1_bt: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        vd1_bt.insert("name".into(), ValidatedValue::String("var1".into()));
        vd1_bt.insert("type".into(), ValidatedValue::String("integer".into()));
        vd1_bt.insert("value".into(), ValidatedValue::Integer(1));
        let vd1 = ValidatedValue::Object(vd1_bt);
        let mut vd2_bt: BTreeMap<String, ValidatedValue> = BTreeMap::new();
        vd2_bt.insert("name".into(), ValidatedValue::String("var2".into()));
        vd2_bt.insert("type".into(), ValidatedValue::String("boolean".into()));
        vd2_bt.insert("value".into(), ValidatedValue::Bool(true));
        let vd2 = ValidatedValue::Object(vd2_bt);

        let v = vec![vd1, vd2];
        let attributes = attributes_from_validated_value(Some(&ValidatedValue::Array(v))).unwrap();
        assert_eq!(attributes.len(), 2);
        assert_eq!(attributes[0].name, "var1");
        assert_eq!(attributes[0].value, VariableValue::Integer(1));
        // let _vd1 = VariableDeclaration::new("var1", "integer", VariableValue::Integer(1));
    }

    #[test]
    fn test_read_sc() {
        let open_api_file = std::fs::File::open("StateMachines.yml").unwrap();
        let open_api = OpenApi::new(&open_api_file).unwrap();
        let sc = std::fs::read_to_string("tests/simple-task.json").unwrap();
        let sc_schema = open_api.get_schema("#/components/schemas/Node").unwrap();
        let _vvsc = ValidatedValue::new(&sc, &sc_schema, &open_api).unwrap();
    }
}
