use crate::kvledger::history;
use crate::simulator::TxSimulator;

use crate::txmgr::TxMgr;
use crate::QueryExecutor;
use blockdb::BlockStore;
use error::*;
use protos::*;

pub struct KVLedger<S: BlockStore, T: TxMgr> {
    ledger_id: String,
    block_store: S,
    history_db: String,
    tx_mgmt: T,
}

impl<S: BlockStore, T: TxMgr> KVLedger<S, T> {
    pub fn new(ledger_id: &str, store: S, tx_mgmt: T) -> Self {
        KVLedger {
            ledger_id: String::from(ledger_id),
            block_store: store,
            history_db: String::from("history db"),
            tx_mgmt,
        }
    }
}

impl<S: BlockStore, T: TxMgr> crate::Ledger for KVLedger<S, T> {
    type HQE = history::KVHistoryQueryExecutor;
    type TS = T::T;

    fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.block_store.get_blockchain_info()
    }

    fn get_block_by_number(&self, block_number: u64) -> Result<Option<Block>> {
        self.block_store.retrieve_block_by_number(block_number)
    }

    fn get_blocks_iterator(
        &self,
        start_block_number: u64,
    ) -> Result<Box<dyn Iterator<Item = Result<Option<Block>>>>> {
        self.block_store.retrieve_blocks(start_block_number)
    }

    fn get_transaction_by_id(&self, tx_id: &str) -> Result<Option<ProcessedTransaction>> {
        let tx = self.block_store.retrieve_tx_by_id(tx_id)?;
        if tx.is_none() {
            return Ok(None);
        }

        let bytes = utils::proto::marshal(&tx.unwrap())?;

        let code = self.block_store.retrieve_tx_validation_code_by_txid(tx_id)?;
        let pt = ProcessedTransaction{ transaction_envelope: Some(Envelope{payload: bytes, signature: vec![]}), validation_code: code as i32};

        Ok(Some(pt))
    }

    fn get_block_by_hash(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.block_store.retrieve_block_by_hash(block_hash)
    }

    fn get_block_by_tx_id(&self, tx_id: &str) -> Result<Option<Block>> {
        self.block_store.retrieve_block_by_txid(tx_id)
    }

    fn get_tx_validation_code_by_tx_id(&self, tx_id: &str) -> Result<TxValidationCode> {
        self.block_store.retrieve_tx_validation_code_by_txid(tx_id)
    }

    fn new_tx_simulator(&self, tx_id: &str) -> Result<Self::TS> {
        let sim = self.tx_mgmt.new_tx_simulator(tx_id)?;
        Ok(sim)
    }

    fn new_query_executor(&self) -> Result<Box<dyn QueryExecutor>> {
        unimplemented!()
    }

    fn new_history_query_executor(&self) -> Result<Self::HQE> {
        unimplemented!()
    }

    fn commit_legacy(&self, block: Block) -> Result<()> {
        // self.tx_mgmt.validate_and_prepare(&block)
        unimplemented!()
    }
}
