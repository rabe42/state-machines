use open_api_matcher::ValidatedValue;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use log::debug;

use crate::error::StateChartError;
use crate::sql::Crud;
use crate::ids::NodeId;
use crate::state_charts::{ActionCall, Transition, VariableDeclaration, get_mandatory};

/// The node is the heart of the state chart definition. A node can be a single state or a state
/// chart of its own.
/// The node will be saved in all details to the database. The objective is to make it easier to
/// address the nodes in the context of the state machines operations.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Node {
    id: NodeId,
    description: Option<String>,
    on_entry: Option<ActionCall>,
    on_exit: Option<ActionCall>,
    start_node: Option<NodeId>,
    out_transitions: Vec<Transition>,
    attributes: Vec<VariableDeclaration>,
    nodes: Vec<Node>,
}
impl Node {
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Provides access to the optional start node of the state chart.
    #[allow(dead_code)]
    pub fn start_node(&self) -> Option<&NodeId> {
        self.start_node.as_ref()
    }
}
impl Crud<SqliteConnectionManager, NodeId> for Node {
    type Error = rusqlite::Error;

    /// This id differs from the id of the other CRUD objects. While the id is normally provided
    /// only by the database, this id is provided by the user and already available from the start,
    /// which means, it is not optional at all. This implies some dificulties in providing a
    /// reference, linked to the lifetime of the receiver.
    fn get_id(&self) -> Option<&NodeId>
    {
        Some(&self.id)
    }

    fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        debug!("[node::Crud::create()]");
        let sql = "CREATE TABLE IF NOT EXISTS Node (
                id TEXT NOT NULL UNIQUE,
                description TEXT,
                on_entry TEXT,
                on_exit TEXT,
                start_node TEXT,
                out_transitions TEXT,
                attributes TEXT,
                nodes TEXT
            )";
        connection.execute(sql, [])?;
        let sql = "CREATE TABLE IF NOT EXISTS NodeAttributes (
                node_id TEXT NOT NULL,
                attribute_id TEXT NOT NULL
            )";
        connection.execute(sql, [])?;
        let sql = "CREATE TABLE IF NOT EXISTS SubNodes (
                parent_node TEXT NOT NULL,
                child_node TEXT NOT NULL
            )";
        connection.execute(sql, [])?;

        // Create the tables for the dependent data types.
        Transition::create(connection)?;
        VariableDeclaration::create(connection)?;

        Ok(())
    }
    fn insert(&mut self, connection: &PooledConnection<SqliteConnectionManager>) -> Result<&NodeId, Self::Error>
    {
        debug!("[node::Crud::insert()]");
        let sql = "INSERT INTO Node (
                   id, description, on_entry, on_exit, start_node, out_transitions, attributes, nodes 
                ) VALUES ( 
                    ?, ?, ?, ?, ?, ?, ?, ?
                )";
        let mut statement = connection.prepare(sql)?;
        let _rowid = statement.insert(params![]);
        Ok(&self.id)
    }
    fn update(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        debug!("[node::Crud::update()]");
        todo!()
    }
    fn delete(&self, _connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
    {
        debug!("[node::Crud::delete()]");
        todo!()
    }
    fn select(_connection: &PooledConnection<SqliteConnectionManager>, _key_value: &NodeId) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized
    {
        debug!("[node::Crud::select()]");
        todo!()
    }
}
/// Constructs a node from the validated value.
impl TryFrom<&ValidatedValue> for Node {
    type Error = StateChartError;

    fn try_from(value: &ValidatedValue) -> Result<Self, Self::Error> {
        if let ValidatedValue::Object(attributes) = value {
            let on_entry = match attributes.get("on-entry") {
                Some(vac) => {
                    let ac: ActionCall = vac.try_into()?;
                    Some(ac)
                }
                None => None,
            };
            let on_exit = match attributes.get("on-exit") {
                Some(vac) => {
                    let ac: ActionCall = vac.try_into()?;
                    Some(ac)
                }
                None => None,
            };
            let description = match attributes.get("description") {
                Some(vd) => {
                    let d: String = vd.try_into()?;
                    Some(d)
                }
                None => None,
            };
            let start_node: Option<NodeId> = match attributes.get("start-node") {
                Some(sn) => Some(sn.try_into()?),
                None => None,
            };
            let out_transitions = match attributes.get("out-transitions") {
                Some(ot) => transitions_from_validated_value(ot)?,
                None => Vec::new(),
            };
            Ok(Node {
                id: get_mandatory(attributes, "id")?.try_into()?,
                description,
                on_entry,
                on_exit,
                start_node,
                out_transitions,
                attributes: attributes_from_validated_value(attributes.get("attributes"))?,
                nodes: nodes_from_validated_value(attributes.get("nodes"))?,
            })
        } else {
            Err(StateChartError::UnexpectedType)
        }
    }
}

/// Retrieves the transitions of a node from the transition.
fn transitions_from_validated_value(
    value: &ValidatedValue,
) -> Result<Vec<Transition>, StateChartError> {
    if let ValidatedValue::Array(transitions) = value {
        let mut result = Vec::new();
        for v_transition in transitions {
            let transition = v_transition.try_into()?;
            result.push(transition)
        }
        Ok(result)
    } else {
        Err(StateChartError::UnexpectedType)
    }
}

/// Retrieves the attributes/variables from the array.
fn attributes_from_validated_value(
    value: Option<&ValidatedValue>,
) -> Result<Vec<VariableDeclaration>, StateChartError> {
    if let Some(value) = value {
        if let ValidatedValue::Array(attribs) = value {
            let mut result: Vec<VariableDeclaration> = Vec::new();
            for attribute in attribs {
                let vd: VariableDeclaration = attribute.try_into()?;
                result.push(vd);
            }
            Ok(result)
        } else {
            Err(StateChartError::UnexpectedType)
        }
    } else {
        Ok(Vec::new())
    }
}

fn nodes_from_validated_value(
    value: Option<&ValidatedValue>,
) -> Result<Vec<Node>, StateChartError> {
    if let Some(v_value) = value {
        if let ValidatedValue::Array(nodes) = v_value {
            let mut result: Vec<Node> = Vec::new();
            for v_node in nodes {
                let node = v_node.try_into()?;
                result.push(node);
            }
            Ok(result)
        } else {
            Err(StateChartError::UnexpectedType)
        }
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use crate::state_charts::VariableValue;
    use open_api_matcher::OpenApi;
    use r2d2::Pool;

    #[test]
    fn test_read_sc() {
        let open_api_file = std::fs::File::open("StateMachines.yml").unwrap();
        let open_api = OpenApi::new(&open_api_file).unwrap();
        let sc = std::fs::read_to_string("tests/simple-task.json").unwrap();
        let sc_schema = open_api.get_schema("#/components/schemas/Node").unwrap();
        let vvsc = ValidatedValue::new(&sc, &sc_schema, &open_api).unwrap();
        let node: Node = (&vvsc).try_into().unwrap();
        assert_eq!(NodeId::new("Simple-Task"), node.id);
        assert_eq!(NodeId::new("Simple-Task/New"), node.start_node.unwrap());
        assert_eq!(3, node.nodes.len());
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
        assert_eq!(attributes[0].name(), "var1");
        assert_eq!(attributes[0].value(), &VariableValue::Integer(1));
        // let _vd1 = VariableDeclaration::new("var1", "integer", VariableValue::Integer(1));
    }

    fn create_db_connection() -> PooledConnection<SqliteConnectionManager> {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = Pool::builder().max_size(10).build(manager).unwrap();
        pool.get().unwrap()
    }

    #[test]
    fn test_node_crud() {
        let connection = create_db_connection();
        Node::create(&connection).unwrap();
    }
}
