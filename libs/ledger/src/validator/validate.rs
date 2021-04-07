use error::*;

use crate::rwset::builder::TxRwSet;
use crate::rwset::key::{self, PubAndHashUpdates};
use crate::statedb::{self, Height, UpdateBatch, VersionedDB};
use protos::*;

use std::convert::TryFrom;




pub struct Validator<V: VersionedDB> {
    vdb: V,
}

impl<V: VersionedDB> Validator<V> {
    pub fn new(vdb: V) -> Self {
        Validator { vdb }
    }

    pub fn validate_and_prepare_batch(&self, block: &mut Block) -> Result<(UpdateBatch, Height)> {
        utils::blockutils::init_tx_validation_flags(block);

        let header = block.header.as_ref().ok_or(anyhow!("block header is null"))?;
        let data = block.data.as_ref().ok_or(anyhow!("block data is null"))?;
        let metadata = block.metadata.as_mut().ok_or(anyhow!("block metadata is null"))?;
        let txs_filter = metadata.metadata.get_mut(BlockMetadataIndex::TransactionsFilter as usize)
            .ok_or(anyhow!("metadata TransactionsFilter not set"))?;

        let mut updates = PubAndHashUpdates::new();

        for (index, proto_msg) in data.data.iter().enumerate() {
            let tx: Transaction = utils::proto::unmarshal(proto_msg)?;
            let proposal = tx
                .signed_proposal
                .ok_or_else(|| anyhow!("proposal is null"))?;
            let tx_header = utils::proto::unmarshal::<Proposal>(&proposal.proposal_bytes)?
                .header
                .ok_or_else(|| anyhow!("transaction header is null"))?;


            let resp = match tx.response.get(0) {
                Some(v) => v,
                None => {
                    txs_filter[index] = TxValidationCode::NilTxaction as u8;
                    continue;
                }
            };


            let payload: ProposalResponsePayload = match utils::proto::unmarshal(&resp.payload) {
                Ok(v) => v,
                Err(e) => {
                    error!("unmarshal tx response payload error: {:}", e);
                    txs_filter[index] =  TxValidationCode::InvalidOtherReason as u8;
                    continue;
                }
            };


            let tx_read_write_set: TxReadWriteSet = match utils::proto::unmarshal(&payload.results) {
                Ok(v) => v,
                Err(e) => {
                    error!("unmarshal  tx read write set error: {:}", e);
                    txs_filter[index] = TxValidationCode::InvalidOtherReason as u8;
                    continue;
                }
            };

            let tx_rw_set = match TxRwSet::try_from(tx_read_write_set) {
                Ok(v) => v,
                Err(e) => {
                    error!("try from txRwSet error: {:}", e);
                    txs_filter[index] = TxValidationCode::InvalidWriteset as u8;
                    continue;
                }
            };

            if self.validate_writeset(&tx_rw_set).is_err() {
                txs_filter[index] = TxValidationCode::InvalidWriteset as u8;
                continue;
            }

            let validation_code = self.validate_tx(&tx_rw_set, &mut updates)?;

            if validation_code == TxValidationCode::Valid {
                debug!("Block [{:?}] Transaction index [{:?}] TxId [{:?}] marked as valid by state validator.  [{:?}]", header.number, index, tx_header.tx_id, validation_code);
                let _ = updates
                    .apply_write_set(tx_rw_set, Height::new(header.number, index as u64));
            } else {
                warn!("Block [{:?}] Transaction index [{:?}] TxId [{:?}] marked as invalid by state validator. Reason code [{:?}]",
                      header.number, index, tx_header.tx_id, validation_code);
            }
            txs_filter[index] =  validation_code as u8;
        }
        return Ok((
            UpdateBatch::from(updates),
            Height::new(header.number, (data.data.len() - 1) as u64),
        ));
    }

    fn validate_writeset(&self, tx_rw_set: &TxRwSet) -> Result<()> {
        for rw_set in &tx_rw_set.ns_rw_sets {
            //Validation of write set
            for kv_write in &rw_set.kv_rw_set.writes {
                self.vdb
                    .validate_key_value(&kv_write.key, &kv_write.value)?;
            }
        }

        Ok(())
    }

    fn validate_tx(
        &self,
        tx_rw_set: &TxRwSet,
        updates: &mut PubAndHashUpdates,
    ) -> Result<TxValidationCode> {
        for rw_set in &tx_rw_set.ns_rw_sets {
            let ns = rw_set.namespace.clone();

            // Validation of read set
            for kv_read in &rw_set.kv_rw_set.reads {
                if updates.pub_updates.exists(&ns, &kv_read.key) {
                    return Ok(TxValidationCode::MvccReadConflict);
                }

                let committed_version = self.vdb.get_version(&ns, &kv_read.key)?;

                let ver = kv_read.version.clone().map(Height::from);
                debug!(
                    "comparing versions for key [{:?}]: committed version={:?} and read version={:?}",
                    kv_read.key.clone(),
                    committed_version.clone(),
                    ver
                );
                if !statedb::are_same(committed_version, ver) {
                    debug!("Version mismatch for key [{:?}:{:?}]. committed version = [{:?}], version in read set [{:?}]",
                           ns, kv_read.key, committed_version, kv_read.version);
                    return Ok(TxValidationCode::MvccReadConflict);
                }
            }

            // Validate range queries for phantom items
            for rgi in &rw_set.kv_rw_set.range_queries_info {
                debug!(
                    "validate range query: ns={:?}, RangeQueryInfo={:?}",
                    ns.clone(),
                    rgi
                )
                // TODO:
            }

            // Validate hashes for private reads
            for coll_hashed_rw_set in &rw_set.coll_hashed_rw_sets {
                let coll = coll_hashed_rw_set.collection_name.clone();

                for kv_read_hash in &coll_hashed_rw_set.hashed_rw_set.hashed_reads {
                    let hash = utils::base64::encode(&kv_read_hash.key_hash);

                    if updates.hash_updates.contains_key(&ns) {
                        let ns_batch = updates.hash_updates.get(&ns).unwrap();
                        if ns_batch.exists(&coll, &hash) {
                            return Ok(TxValidationCode::MvccReadConflict);
                        }
                    }

                    let committed_version = self
                        .vdb
                        .get_version(&key::derive_hashed_data_ns(&ns, &coll), &hash)?;

                    let ver = kv_read_hash.version.clone().map(Height::from);
                    if !statedb::are_same(committed_version, ver) {
                        debug!("Version mismatch for key hash [{:?}:{:?}:{:?}]. committed version = [{:?}], version in hashed read set [{:?}]",
                               ns,
                               coll_hashed_rw_set.collection_name,
                               hash,
                               committed_version,
                               kv_read_hash.version);
                        return Ok(TxValidationCode::MvccReadConflict);
                    }
                }
            }
        }

        Ok(TxValidationCode::Valid)
    }
}
