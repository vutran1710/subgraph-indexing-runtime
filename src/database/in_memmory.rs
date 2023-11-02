use super::abstract_types;
use super::DatabaseTrait;
use super::RawEntity;
use crate::errors::DatabaseError;
use std::collections::HashMap;

pub type InMemoryDataStore =
    HashMap<String, HashMap<String, HashMap<String, abstract_types::Value>>>;

impl DatabaseTrait for InMemoryDataStore {
    fn handle_load(
        &self,
        entity_type: String,
        entity_id: String,
    ) -> Result<Option<RawEntity>, DatabaseError> {
        let store = self;
        let table = store.get(&entity_type);

        if table.is_none() {
            return Ok(None);
        }

        let table = table.unwrap();
        let entity = table.get(&entity_id);

        if entity.is_none() {
            return Ok(None);
        }

        let entity = entity.unwrap().to_owned();
        Ok(Some(entity))
    }

    fn handle_create(&mut self, entity_type: String, data: RawEntity) -> Result<(), DatabaseError> {
        let store = self;
        if !store.contains_key(&entity_type) {
            store.insert(entity_type.clone(), HashMap::new());
        }

        let table = store.get_mut(&entity_type).unwrap();
        if let abstract_types::Value::String(entity_id) =
            data.get("id").ok_or(DatabaseError::MissingID)?
        {
            table.insert(entity_id.to_owned(), data);
            Ok(())
        } else {
            unimplemented!()
        }
    }

    fn handle_update(
        &mut self,
        entity_type: String,
        entity_id: String,
        data: RawEntity,
    ) -> Result<(), DatabaseError> {
        let store = self;
        if !store.contains_key(&entity_type) {
            store.insert(entity_type.clone(), HashMap::new());
        }
        assert!(data.contains_key("id"));

        let table = store.get_mut(&entity_type).unwrap();
        table.remove_entry(&entity_id);
        table.insert(entity_id, data);

        Ok(())
    }
}