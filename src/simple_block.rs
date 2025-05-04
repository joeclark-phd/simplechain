use bincode::{config, Decode, Encode};

use crate::utilities::sha3_256_hash;

const MINING_PAYOUT: i32 = 25;

/// A toy struct representing a transaction, which will be encapsulated by a SimpleRecord.
/// SimpleRecord is the main "data row" element of this blockchain, and anything that can 
/// be encoded as a vector of bytes (Vec<u8>) can represent its "data".
#[derive(Encode, Decode)]
pub struct SimpleTransaction {
    from: Option<String>,
    to: String,
    amount: i32,
}
impl SimpleTransaction {

    pub fn new(from: String, to: String, amount: i32) -> Self {
       Self { from: Some(from), to, amount, }
    }

    pub fn new_mining(to: String) -> Self {
        Self { 
            from: None,
            to,
            amount: MINING_PAYOUT,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).unwrap()
    }

}


/// A record that can be (part of) a SimpleBlock.  It doesn't matter what the underlying type
/// of "data" is, as long as it can be serialized into bytes.
/// 
/// NOTE: Not knowing what the data is, there could be duplicates.  If we want hashes to be unique,
/// we could add a 'timestamp' field to SimpleRecord.
#[derive(Encode, Debug, Decode)]
pub struct SimpleRecord {
    id: Vec<u8>,
    data: Vec<u8>,
}
impl SimpleRecord {

    pub fn new(data: Vec<u8>) -> Self {
        let mut rec = Self { id: vec![], data };
        rec.id = rec.hash();
        rec 
    }

    pub fn serialize(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).unwrap()
    }

    pub fn hash(&self) -> Vec<u8> {
        sha3_256_hash(self.data.as_slice())
    }

}

#[derive(Encode, Decode)]
pub struct SimpleBlock {
    id: Vec<u8>,
    prev: Option<Vec<u8>>,
    height: u32,
    records: Vec<SimpleRecord>,
    record_hash: Vec<u8>,
    nonce: u64
}
impl SimpleBlock {

    pub fn new(prev: Vec<u8>, height: u32, records: Vec<SimpleRecord>) -> Self {
        let mut block = Self {
            id: vec![],
            prev: Some(prev),
            height,
            records,
            record_hash: vec![],
            nonce: 0
        };
        block.record_hash = block.hash_records();
        block.rehash();
        block
    }

    pub fn new_genesis( genesis_record: SimpleRecord ) -> Self {
        let mut block = Self {
            id: vec![],
            prev: None,
            height: 0,
            records: vec!(genesis_record),
            record_hash: vec![],
            nonce: 0
        };
        block.record_hash = block.hash_records();
        block.rehash();
        block
    }

    pub fn serialize(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        let config = config::standard();
        bincode::decode_from_slice(bytes, config).unwrap().0
    }

    fn serialize_records(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(&self.records, config).unwrap()
    }

    /// Returns the size in bytes of the 'records' vector.
    pub fn get_records_size(&self) -> usize {
        self.serialize_records().len()
    }

    /// Hash the records vector.
    /// 
    /// NOTE: There's no reason (yet) in this experimental project to hash the records. 
    /// That'd be useful in the real world for "light nodes" that store the block headers,
    /// but not the full block of records.
    /// 
    /// NOTE: This will give a different hash depending on the order in which the records are
    /// listed.  It might be better to go to something like a Merkle tree instead of a vector
    /// so that we always get the same structure and same hash with the same set of records.
    fn hash_records(&self) -> Vec<u8> {
        sha3_256_hash(self.serialize_records().as_slice())
    }

    /// Updates the hash (a hash of the prev, height, record_hash, and nonce fields).
    /// NOTE: the records vector itself is not included in the hash, since some "light nodes"
    /// may not carry it.
    pub fn rehash(&mut self) {
        let blockcopy = SimpleBlock {
            id: vec![],
            prev: self.prev.clone(),
            height: self.height.clone(),
            records: vec![],
            record_hash: self.record_hash.clone(),
            nonce: self.nonce.clone()
        };
        self.id = sha3_256_hash(blockcopy.serialize().as_slice());
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.rehash();
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
        self.rehash();
    }

    pub fn get_height(&self) -> &u32 {
        &self.height
    }

    pub fn get_nonce(&self) -> &u64 {
        &self.nonce
    }

    pub fn get_hash(&self) -> &Vec<u8> {
        &self.id
    }

}









#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn rec_id_is_hash_of_rec_data() {
        let txn1 = SimpleTransaction::new("Joseph".to_string(), "Benjamin".to_string(), 1001);
        let rec1 = SimpleRecord::new(txn1.serialize());
        assert_eq!(rec1.hash(), rec1.id);
    }

    #[test]
    fn can_get_records_size_from_block() {
        let txn1 = SimpleTransaction::new("Joseph".to_string(), "Benjamin".to_string(), 1001);
        let rec1 = SimpleRecord::new(txn1.serialize());
        let block1 = SimpleBlock::new(vec![], 42, vec![rec1]);
        let rec2 = SimpleRecord::new(txn1.serialize());
        let rec3 = SimpleRecord::new(txn1.serialize());
        let block2 = SimpleBlock::new(vec![], 42, vec![rec2,rec3]);
        println!("block1 size: {}",block1.get_records_size());
        println!("block2 size: {}",block2.get_records_size());
        assert_ne!(block1.get_records_size(), block2.get_records_size());
    }

    #[test]
    fn changing_nonce_changes_block_hash() {
        let txn1 = SimpleTransaction::new("Joseph".to_string(), "Benjamin".to_string(), 1001);
        let rec1 = SimpleRecord::new(txn1.serialize());
        let mut block1 = SimpleBlock::new(vec![], 42, vec![rec1]);
        let hash1 = block1.get_hash().clone();
        block1.set_nonce(1);
        let hash2 = block1.get_hash().clone();
        assert_ne!(hash1, hash2);
    }

}
