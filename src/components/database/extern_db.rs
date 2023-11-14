use super::scylladb::Scylladb;
use super::RawEntity;
use crate::common::BlockPtr;
use crate::errors::DatabaseError;
use crate::runtime::asc::native_types::store::StoreValueKind;
use async_trait::async_trait;
use std::collections::HashMap;

pub(super) enum ExternDB {
    Scylla(Scylladb),
}

#[async_trait]
pub(super) trait ExternDBTrait: Sized {
    async fn create_entity_table(
        &self,
        entity_type: &str,
        schema: HashMap<String, StoreValueKind>,
    ) -> Result<(), DatabaseError>;
    async fn load_entity(
        &self,
        block_ptr: BlockPtr,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Option<RawEntity>, DatabaseError>;
    async fn load_entity_latest(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Option<RawEntity>, DatabaseError>;
    async fn create_entity(
        &self,
        block_ptr: BlockPtr,
        entity_type: &str,
        data: RawEntity,
    ) -> Result<(), DatabaseError>;
    async fn create_entities(
        &self,
        block_ptr: BlockPtr,
        values: Vec<(String, RawEntity)>,
    ) -> Result<(), DatabaseError>;
    async fn soft_delete_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<(), DatabaseError>;
    async fn hard_delete_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<(), DatabaseError>;

    /// Revert all entity creations from given block ptr up to latest by hard-deleting them
    async fn revert_create_entity(&self, from_block: u64) -> Result<(), DatabaseError>;

    /// Revert all entity deletion from given block ptr up to latest by nullifing `is_deleted` fields
    async fn revert_delete_entity(&self, from_block: u64) -> Result<(), DatabaseError>;
}