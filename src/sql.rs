use r2d2::{ManageConnection,PooledConnection};

/// The trait provides an interface to the naive database operations. It supports different
/// databases in one codes byse by a type parameter.
/// The trait is used in an aspect oriented way in this case. It allows to separate the concept of
/// the database operations from the business logic.
pub trait Crud<B: ManageConnection> {
    type Error;
    fn create(connection: &PooledConnection<B>) -> Result<(), Self::Error>;
    fn insert(&mut self, connection: &PooledConnection<B>) -> Result<(), Self::Error>;
    fn update(&self, connection: &PooledConnection<B>) -> Result<(), Self::Error>;
    fn delete(&self, connection: &PooledConnection<B>) -> Result<(), Self::Error>;
    fn select(connection: &PooledConnection<B>, key: i64) -> Result<Option<Self>, Self::Error> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    struct Entity {
        /// This is a stand in for the rowid. It indicates, if the object is already in the
        /// database.
        id: Option<i64>,
        attribute_1: String,
        attribute_2: i64,
    }
    impl Entity {
        fn new(attribute_1: &str, attribute_2: i64) -> Self {
            Entity { id: None, attribute_1: attribute_1.into(), attribute_2 }
        }
    }
    impl Crud<SqliteConnectionManager> for Entity {
        type Error = rusqlite::Error;

        fn create(connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
        {
            let sql = "CREATE TABLE Entity ( attribute_1 VARCHAR(255), attribute_2 INTEGER )";
            connection.execute(sql, [])?;
            Ok(())
        }

        fn insert(&mut self, connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
        {
            if let Some(_) = self.id {
                panic!("Cannot insert the same entity twice!");
            } else {
                let sql = "INSERT INTO Entity ( attribute_1, attribute_2 ) VALUES ( ?, ? )";
                let mut statement = connection.prepare(sql)?;
                self.id = Some(statement.insert(params![self.attribute_1, self.attribute_2])?);
                Ok(())
            }
        }

        fn update(&self, connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
        {
            if let Some(_) = self.id {
                let sql = "UPDATE Entity SET attribute_1 = ?, attribute_2 = ? WHERE rowid = ?";
                let mut statement = connection.prepare(sql)?;
                assert_eq!(1, statement.execute(params![self.attribute_1, self.attribute_2, self.id])?);
                Ok(())
            } else {
                panic!("Cannot update an entity, which is not already in the database!");
            }
        }

        fn delete(&self, connection: &PooledConnection<SqliteConnectionManager>) -> Result<(), Self::Error>
        {
            if let Some(_) = self.id {
                let sql = "DELETE FROM Entity WHERE rowid = ?";
                let mut statement = connection.prepare(sql)?;
                assert_eq!(1, statement.execute(params![self.id])?);
                Ok(())
            } else {
                panic!("Cannot delete an entity, which is not already in the database!");
            }
        }

        fn select(connection: &PooledConnection<SqliteConnectionManager>, key: i64) -> Result<Option<Self>, Self::Error> where Self: Sized
        {
            let sql = "SELECT rowid, attribute_1, attribute_2 FROM Entity WHERE rowid = ?";
            let mut statement = connection.prepare(sql)?;
            let mut entities = statement.query_map(params![key], |row| {
                let id = row.get(0)?;
                let attribute_1 = row.get(1)?;
                let attribute_2 = row.get(2)?;
                Ok(Some(Entity { id: Some(id), attribute_1, attribute_2 }))
            })?;

            // Returns only the first found!
            if let Some(result) = entities.next() {
                result
            }
            else {
                Ok(None)
            }
        }
    }

    fn create_db_connection() -> Pool<SqliteConnectionManager>
    {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .unwrap();
        pool
    }

    #[test]
    fn test_crud_on_entity() {
        let pool = create_db_connection();
        let connection = pool.get().unwrap();
        Entity::create(&connection).unwrap();
        let mut entity1 = Entity::new("Hello", 3);
        entity1.insert(&connection).unwrap();
        let mut entity2 = Entity::new("World", 1000000003);
        entity2.insert(&connection).unwrap();
        let selected_e1 = Entity::select(&connection, entity1.id.unwrap()).unwrap().unwrap();
        assert_eq!(entity1.attribute_1, selected_e1.attribute_1);
        assert_eq!(entity1.attribute_2, selected_e1.attribute_2);
        let selected_e2 = Entity::select(&connection, entity2.id.unwrap()).unwrap().unwrap();
        assert_eq!(entity2.attribute_1, selected_e2.attribute_1);
        assert_eq!(entity2.attribute_2, selected_e2.attribute_2);
        entity1.attribute_2 = 4;
        entity1.update(&connection).unwrap();
        let selected_e1 = Entity::select(&connection, entity1.id.unwrap()).unwrap().unwrap();
        assert_eq!(entity1.attribute_1, selected_e1.attribute_1);
        assert_eq!(entity1.attribute_2, selected_e1.attribute_2);
        entity1.delete(&connection).unwrap();
        if let Some(_) = Entity::select(&connection, entity1.id.unwrap()).unwrap() {
            panic!("Deleted entity shouldn't be found.");
        }
    }
}
