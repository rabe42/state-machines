use crate::error::StateChartError;
use crate::ids::NodeId;
use crate::sql::{Crud, KeyValue};
use open_api_matcher::ValidatedValue;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::collections::BTreeMap;

/// A system wide unique Id of a action.
pub type ActionId = String;

/// A system wide unique Id of a variable.
pub type VariableId = String;

/// A system wide unique Id of a event.
pub type EventId = String;

/// A system wide unique Id of a predicate.
pub type PredicateId = String;

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
            let parameters: Vec<Parameter> =
                parameters_from_validated_values(get_mandatory(attributes, "parameters")?)?;
            Ok(ActionCall { name, parameters })
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// The action call buys in on the implicid rowid, provided by SQLite.
/// While the action call is basically the name of the action and the list of parameters. The same
/// name and set of parameters can be used multiple times. In the relational world, this requires
/// an additional object id of the action call relation/object.
impl Crud<SqliteConnectionManager> for ActionCall {
    type Error = rusqlite::Error;
    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS ActionCall (
                name TEXT NOT NULL
            )";
        connection.execute(sql, [])?;
        let sql = "CREATE TABLE IF NOT EXISTS ACParameterList (
                action_call_id INTEGER NOT NULL,
                parameter_name TEXT,
                parameter_value TEXT,
                FOREIGN KEY(action_call_id) REFERENCES ActionCall(rowid)
            )";
        connection.execute(sql, [])?;

        Ok(())
    }
    fn insert(&mut self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        todo!()
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

impl Crud<SqliteConnectionManager> for Parameter {
    type Error = rusqlite::Error;

    /// Crates the tables, needed to store a parameter of a action call or a predicate.
    /// To simulate the enumeration behaviour the value type must be stored with it.
    /// It *MUST* have the value "string", "integer", "bool", "number", "none".
    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS Parameter (
                name TEXT NOT NULL,
                value_type TEXT NOT NULL,
                string_value TEXT,
                integer_value INTEGER,
                boolean_value INTEGER,
                number_value REAL
            )";
        connection.execute(sql, [])?;
        Ok(())
    }

    /// Inserts the value of the variable in a different column, depending on the value type.
    /// If the value type is none, a string value is assumed. This is realized by the get_column_name()
    /// and get_type() methods of the VariableValue.
    fn insert(&mut self, connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        let value_column = self.value.get_column_name();
        let sql = format!("INSERT INTO Parameter (name, value_type, {value_column}) VALUES (?, ?, ?)");
        let mut statement = connection.prepare(&sql)?;
        let rowid = statement.insert(params![self.name, self.value.get_type(), self.value.to_string() ])?;
                                                             // parameters here!
        Ok(KeyValue::Integer(rowid))
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        let sql = "SELECT name, value_type; string_value, integer_value, boolean_value, number_value
                   FROM Parameter WHERE rowid=?";
        let _statement = connection.prepare(sql)?;
        todo!()
    }
}

/// The transition from one node to another.
#[allow(dead_code)]
#[derive(Clone, Debug)]
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

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            let guard = get_mandatory(attributes, "guard")?.try_into()?;
            let to = get_mandatory(attributes, "to")?.try_into()?;
            let action = match attributes.get("action") {
                None => None,
                Some(v_action) => {
                    let ac: ActionCall = v_action.try_into()?;
                    Some(ac)
                }
            };
            Ok(Transition { guard, to, action })
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

impl Crud<SqliteConnectionManager> for Transition {
    type Error = rusqlite::Error;

    /// Crates the tables, needed to store a parameter of a action call or a predicate.
    /// To simulate the enumeration behaviour the value type must be stored with it.
    /// It *MUST* have the value "string", "integer", "bool", "number", "none".
    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS Transition (
                guard INTEGER NOT NULL,
                target TEXT NOT NULL,
                action INTEGER
            )";
        connection.execute(sql, [])?;

        Guard::create(connection)?;
        ActionCall::create(connection)?;

        Ok(())
    }
    fn insert(&mut self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        todo!()
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        todo!()
    }
}


/// The guard on a trasition holds the condition under which a transaction is activated.
/// It will be evaluated by the state machine runtime.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Guard {
    Event(EventId),
    Predicate(PredicateCall),
}

impl TryFrom<&ValidatedValue> for Guard {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::String(event_id) = value {
            Ok(Guard::Event(event_id.into()))
        } else if let ValidatedValue::Object(_) = value {
            let predicate_call = value.try_into()?;
            Ok(Guard::Predicate(predicate_call))
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

impl Crud<SqliteConnectionManager> for Guard {
    type Error = rusqlite::Error;

    /// Crates the tables, needed to store a parameter of a action call or a predicate.
    /// To simulate the enumeration behaviour the value type must be stored with it.
    /// It *MUST* have the value "string", "integer", "bool", "number", "none".
    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS Guard (
                guard_type TEXT NOT NULL,
                event TEXT,
                predicate_call INTEGER
            )";
        connection.execute(sql, [])?;

        PredicateCall::create(connection)?;

        Ok(())
    }
    fn insert(&mut self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        todo!()
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        todo!()
    }
}


/// The call of a predicate may be a guard. The predicate of all transactions of the current state
/// will be evaluated when ever a variable value was modified.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PredicateCall {
    name: PredicateId,
    parameters: Vec<Parameter>,
}
impl TryFrom<&ValidatedValue> for PredicateCall {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            let name: PredicateId = get_mandatory(attributes, "name")?.try_into()?;
            let parameters =
                parameters_from_validated_values(get_mandatory(attributes, "parameters")?)?;
            Ok(PredicateCall { name, parameters })
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

impl Crud<SqliteConnectionManager> for PredicateCall {
    type Error = rusqlite::Error;

    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS PredicateCall (
                name TEXT NOT NULL,
                parameter_list INTEGER
            )";
        connection.execute(sql, [])?;
        let sql = "CREATE TABLE IF NOT EXISTS PredicateCallParameterList (
                predicate_call_id INTEGER NOT NULL,
                parameter_id INTEGER NOT NULL
            )";
        connection.execute(sql, [])?;

        Parameter::create(connection)?;

        Ok(())
    }
    fn insert(&mut self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        todo!()
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        todo!()
    }
}

/// Declares a variable inside of a state chart state.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    name: String,
    r#type: String,
    value: VariableValue,
}
impl VariableDeclaration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &VariableValue {
        &self.value
    }
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
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

impl Crud<SqliteConnectionManager> for VariableDeclaration {
    type Error = rusqlite::Error;

    /// Crates the tables, needed to store a parameter of a action call or a predicate.
    /// To simulate the enumeration behaviour the value type must be stored with it.
    /// It *MUST* have the value "string", "integer", "bool", "number", "none".
    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        let sql = "CREATE TABLE IF NOT EXISTS VariableDeclaration (
                name TEXT NOT NULL,
                variable_type TEXT NOT NULL,
                string_value TEXT,
                integer_value INTEGER,
                number_value REAL,
                bool_value INTEGER
            )";
        connection.execute(sql, [])?;

        Ok(())
    }
    fn insert(&mut self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<KeyValue, Self::Error>
    {
        todo!()
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: KeyValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        todo!()
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
impl VariableValue {

    /// Provides access to the type of the variable value, which is intended to be used as the
    /// column of the value in the database.
    fn get_column_name(&self) -> &'static str {
        match self {
            Self::String(_) => "string_value",
            Self::Integer(_) => "integer_value",
            Self::Number(_) => "number_value",
            Self::Boolean(_) => "boolean_value",
            Self::None => "string_value",
        }
    }

    fn get_type(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::None => "string",
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Integer(i) => format!("{}", i),
            Self::Number(n) => format!("{}", n),
            Self::Boolean(b) => if *b { String::from("1") } else { String::from("0") },
            Self::None => String::from(""),
        }
    }
}
impl Default for VariableValue {
    fn default() -> Self {
        Self::None
    }
}
impl TryFrom<&ValidatedValue> for VariableValue {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
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


/// Derives a vector of parameters from an array of validated values.
fn parameters_from_validated_values(
    values: &ValidatedValue,
) -> Result<Vec<Parameter>, StateChartError> {
    if let ValidatedValue::Array(values) = values {
        let mut result = Vec::new();
        for v_param in values {
            let param: Parameter = v_param.try_into()?;
            result.push(param);
        }
        Ok(result)
    } else {
        Err(StateChartError::UnexpectedType)
    }
}

/// Retrieves a mandatory attribute from a standard map.
pub fn get_mandatory<'a>(
    attributes: &'a BTreeMap<String, ValidatedValue>,
    name: &str,
) -> Result<&'a ValidatedValue, StateChartError> {
    if let Some(attribute) = attributes.get(name) {
        Ok(attribute)
    } else {
        Err(StateChartError::MandatoryAttributeMissing(name.into()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use r2d2::Pool;

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
        let parameters =
            parameters_from_validated_values(&ValidatedValue::Array(v_parameters)).unwrap();
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

    fn create_db_connection() -> PooledConnection<SqliteConnectionManager> {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = Pool::builder().max_size(10).build(manager).unwrap();
        pool.get().unwrap()
    }

    #[test]
    fn test_parametert_crud() {
        let connection = create_db_connection();
        Parameter::create(&connection).unwrap();
        let mut p1 = Parameter { name: "p1".into(), value: VariableValue::String("a string".into()) };
        p1.insert(&connection).unwrap();
        let mut p2 = Parameter { name: "p2".into(), value: VariableValue::Integer(3623456) };
        p2.insert(&connection).unwrap();
        let mut p3 = Parameter { name: "p3".into(), value: VariableValue::Number(3623456.123456) };
        p3.insert(&connection).unwrap();
        let mut p4 = Parameter { name: "p4".into(), value: VariableValue::Boolean(true) };
        p4.insert(&connection).unwrap();
        let mut p5 = Parameter { name: "p5".into(), value: VariableValue::None };
        p5.insert(&connection).unwrap();
    }

    #[test]
    fn test_predicate_call_crud() {
        let connection = create_db_connection();
        PredicateCall::create(&connection).unwrap();
    }

    #[test]
    fn test_action_call_crud() {
        let connection = create_db_connection();
        ActionCall::create(&connection).unwrap();
    }

    #[test]
    fn test_variable_declaration_crud() {
        let connection = create_db_connection();
        VariableDeclaration::create(&connection).unwrap();
    }
}
