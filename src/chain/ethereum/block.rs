use super::asc::*;
use web3::types::Block;
use web3::types::H160;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::asc::base::AscIndexId;
use crate::asc::base::AscPtr;
use crate::asc::base::IndexForAscTypeId;
use crate::impl_asc_type_struct;

#[repr(C)]
pub(crate) struct AscEthereumBlock {
    pub hash: AscPtr<AscH256>,
    pub parent_hash: AscPtr<AscH256>,
    pub uncles_hash: AscPtr<AscH256>,
    pub author: AscPtr<AscH160>,
    pub state_root: AscPtr<AscH256>,
    pub transactions_root: AscPtr<AscH256>,
    pub receipts_root: AscPtr<AscH256>,
    pub number: AscPtr<AscBigInt>,
    pub gas_used: AscPtr<AscBigInt>,
    pub gas_limit: AscPtr<AscBigInt>,
    pub timestamp: AscPtr<AscBigInt>,
    pub difficulty: AscPtr<AscBigInt>,
    pub total_difficulty: AscPtr<AscBigInt>,
    pub size: AscPtr<AscBigInt>,
    pub base_fee_per_block: AscPtr<AscBigInt>,
}

impl AscIndexId for AscEthereumBlock {
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::EthereumBlock;
}

impl_asc_type_struct!(
    AscEthereumBlock;
    hash => AscPtr<AscH256>,
    parent_hash => AscPtr<AscH256>,
    uncles_hash => AscPtr<AscH256>,
    author => AscPtr<AscH160>,
    state_root => AscPtr<AscH256>,
    transactions_root => AscPtr<AscH256>,
    receipts_root => AscPtr<AscH256>,
    number => AscPtr<AscBigInt>,
    gas_used => AscPtr<AscBigInt>,
    gas_limit => AscPtr<AscBigInt>,
    timestamp => AscPtr<AscBigInt>,
    difficulty => AscPtr<AscBigInt>,
    total_difficulty => AscPtr<AscBigInt>,
    size => AscPtr<AscBigInt>,
    base_fee_per_block => AscPtr<AscBigInt>
);

/*
/// Convert bblock data from query store to Asc Block
impl ToAscObj<AscBlock> for BlockFromQueryStore {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscBlock, AscError> {
        Ok(AscBlock {
            hash: asc_new(heap, &self.hash, gas)?,
            parent_hash: asc_new(heap, &self.parent_hash, gas)?,
            uncles_hash: asc_new(heap, &self.uncles_hash, gas)?,
            author: asc_new(heap, &self.author, gas)?,
            state_root: asc_new(heap, &self.state_root, gas)?,
            transactions_root: asc_new(heap, &self.transactions_root, gas)?,
            receipts_root: asc_new(heap, &self.receipts_root, gas)?,
            number: asc_new(heap, &BigInt::from(self.number), gas)?,
            gas_used: asc_new(heap, &BigInt::from_unsigned_u256(&self.gas_used), gas)?,
            gas_limit: asc_new(heap, &BigInt::from_unsigned_u256(&self.gas_limit), gas)?,
            timestamp: asc_new(heap, &BigInt::from_unsigned_u256(&self.timestamp), gas)?,
            difficulty: asc_new(heap, &BigInt::from_unsigned_u256(&self.difficulty), gas)?,
            total_difficulty: asc_new(
                heap,
                &BigInt::from_unsigned_u256(&self.total_difficulty),
                gas,
            )?,
            size: self
                .size
                .map(|size| asc_new(heap, &BigInt::from_unsigned_u256(&size), gas))
                .unwrap_or(Ok(AscPtr::null()))?,
            base_fee_per_block: self
                .base_fee_per_gas
                .map(|base_fee| asc_new(heap, &BigInt::from_unsigned_u256(&base_fee), gas))
                .unwrap_or(Ok(AscPtr::null()))?,
        })
    }
}
*/

#[derive(Clone, Debug, Default)]
pub struct EthereumBlockData {
    pub hash: H256,
    pub parent_hash: H256,
    pub uncles_hash: H256,
    pub author: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub number: U64,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub timestamp: U256,
    pub difficulty: U256,
    pub total_difficulty: U256,
    pub size: Option<U256>,
    pub base_fee_per_gas: Option<U256>,
}

impl<'a, T> From<&'a Block<T>> for EthereumBlockData {
    fn from(block: &'a Block<T>) -> EthereumBlockData {
        EthereumBlockData {
            hash: block.hash.unwrap(),
            parent_hash: block.parent_hash,
            uncles_hash: block.uncles_hash,
            author: block.author,
            state_root: block.state_root,
            transactions_root: block.transactions_root,
            receipts_root: block.receipts_root,
            number: block.number.unwrap(),
            gas_used: block.gas_used,
            gas_limit: block.gas_limit,
            timestamp: block.timestamp,
            difficulty: block.difficulty,
            total_difficulty: block.total_difficulty.unwrap_or_default(),
            size: block.size,
            base_fee_per_gas: block.base_fee_per_gas,
        }
    }
}