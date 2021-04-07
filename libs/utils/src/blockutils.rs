use protos::*;

pub fn new_block(seq_num: u64, previous_hash: Vec<u8>) -> Block {
    Block {
        header: Some(BlockHeader {
            number: seq_num,
            previous_hash,
            data_hash: vec![],
        }),
        data: Some(BlockData { data: vec![] }),
        metadata: Some(BlockMetadata {
            metadata: uninit_metadata(),
        }),
    }
}

pub fn init_tx_validation_flags(block: &mut Block) {
    let metadata: Vec<u8> = block
        .data
        .as_ref()
        .map(|data| vec![TxValidationCode::Valid as u8; data.data.len()])
        .unwrap_or(vec![]);

    let index = BlockMetadataIndex::TransactionsFilter as usize;

    let mut uninit = uninit_metadata();
    uninit.insert(index, metadata);

    block.metadata = Some(BlockMetadata { metadata: uninit });
}

fn uninit_metadata() -> Vec<Vec<u8>> {
    let len = block_metadata_index_name().len();
    let mut metadata = Vec::with_capacity(len);
    for i in 0..len {
        metadata.push(vec![])
    }

    metadata
}

#[cfg(test)]
mod tests {
    use crate::blockutils::{init_tx_validation_flags, new_block};
    use protos::{Block, BlockData, BlockMetadataIndex};

    fn v(block: &mut Block) {
        init_tx_validation_flags(block)
    }

    #[test]
    fn it_works() {
        let mut block = new_block(12, vec![]);
        block.data = Some(BlockData {
            data: vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        });
        println!("{:?}", block);
        v(&mut block);
        println!("{:?}", block);

        let meta = block.metadata.as_mut().unwrap();
        let v = meta
            .metadata
            .get_mut(BlockMetadataIndex::TransactionsFilter as usize)
            .unwrap();
        v[0] = 2;
        v[1] = 4;
        v[2] = 5;

        println!("{:?}", block);
    }
}
