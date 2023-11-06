use crate::chain::ethereum::block::EthereumBlockData;
use crate::chain::ethereum::event::EthereumEventData;
use crate::chain::ethereum::transaction::EthereumTransactionData;
use crate::database::abstract_types::Value;
use std::collections::HashMap;
use web3::types::Log;

#[derive(Debug)]
pub enum SourceInputMessage {
    JSON(serde_json::Value),
    Protobuf,
}

#[derive(Debug)]
pub enum TransformedDataMessage {
    Ethereum {
        block: EthereumBlockData,
        transactions: Vec<EthereumTransactionData>,
        logs: Vec<Log>,
    },
}

#[derive(Debug)]
pub struct EthereumFilteredEvent {
    datasource: String,
    handler: String,
    event: EthereumEventData,
}

#[derive(Debug)]
pub enum FilteredDataMessage {
    Ethereum {
        events: Vec<EthereumFilteredEvent>,
        block: EthereumBlockData,
    },
}

pub type EntityType = String;

pub type EntityID = String;

#[derive(Debug)]
pub enum StoreOperationMessage {
    Create((EntityType, HashMap<String, Value>)),
    Load((EntityType, EntityID)),
    Update((EntityType, EntityID, HashMap<String, Value>)),
    Delete((EntityType, EntityID)),
}

#[derive(Debug)]
pub enum StoreRequestResult {
    Create(String),
    Load(Option<HashMap<String, Value>>),
    Delete,
    Update,
}
