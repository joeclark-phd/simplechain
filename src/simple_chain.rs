use ethnum::U256;
use sled::{transaction::TransactionResult, Db};
use std::env::current_dir;

use crate::simple_block::{SimpleBlock, SimpleRecord, SimpleTransaction};

const DATABASE_NAME: &str = "data";
const BLOCKCHAIN_KEY: &str = "blockchain";
const LAST_BLOCK_KEY: &str = "last_block";
/// To "mine" a block, its hash (as a binary number) must be below 2^DIFFICULTY.
/// DIFFICULTY can be between 1 and 255, larger numbers meaning lower difficulty.
const DIFFICULTY: u8 = 243;

/// This struct maintains a single blockchain on the current computer
/// using a `sled` embedded database in the current working directory.
pub struct SimpleChain {
    db: Db,
    last_block_hash: Vec<u8>,
}

impl SimpleChain {

    pub fn add_block(&mut self, block: SimpleBlock) {
        let block_hash = block.get_hash();
        let blocks_tree = self.db.open_tree(BLOCKCHAIN_KEY).unwrap();
        let _: TransactionResult<(),()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash.as_slice(), block.serialize());
            let _ = tx_db.insert(LAST_BLOCK_KEY, block_hash.as_slice());
            Ok(())
        });
        self.last_block_hash = block_hash.to_vec();
    }

    pub fn set_last_block_hash(&mut self, last_block_hash: Vec<u8>) {
        self.last_block_hash = last_block_hash;
    }

    pub fn get_last_block(&self) -> SimpleBlock {
        let blocks_tree = self.db.open_tree(BLOCKCHAIN_KEY).unwrap();
        let block_bytes = blocks_tree.get(self.last_block_hash.clone()).unwrap().expect("Issue getting bytes.");
        SimpleBlock::deserialize(block_bytes.as_ref())
    }

}


/// A SimpleNode can generate a new SimpleBlockchain, accept transactions
/// for a verification queue, and mine new blocks while verifying transactions.
/// Since SimpleBlockchain only supports one blockchain on the current system,
/// the `initialize` method will either create a new one or (re)connect to the
/// existing one when called.
pub struct SimpleNode {
    chain: SimpleChain,
    /// The SHA3-256 digest of the node owner's account.
    /// Will be the recipient of the genesis block and mining payouts.
    owner: String,
    queue: Vec<SimpleRecord>,
}

impl SimpleNode {

    pub fn drop_and_reinitialize(owner: String) -> Self {
        let db = sled::open(current_dir().unwrap().join(DATABASE_NAME)).unwrap();
        let _ = db.drop_tree(BLOCKCHAIN_KEY);
        drop(db);
        Self::initialize(owner)
    }

    pub fn initialize(owner: String) -> Self {
        let db = sled::open(current_dir().unwrap().join(DATABASE_NAME)).unwrap();
        let blocks_tree = db.open_tree(BLOCKCHAIN_KEY).unwrap();
        let data = blocks_tree.get(LAST_BLOCK_KEY).unwrap();
        let mut blockchain: SimpleChain = SimpleChain {
                db,
                last_block_hash: vec![]
            };
        if data.is_none() {
            // no last block hash in the DB, so we need to create a new blockchain with a genesis block
            println!("No blockchain in database; now creating and mining a genesis block.");
            let genesis_txn = SimpleTransaction::new_mining(owner.clone());
            let genesis_record = SimpleRecord::new(genesis_txn.serialize());
            let mut genesis_block = SimpleBlock::new_genesis(genesis_record);
            // mine the genesis block so it's valid
            mine_block(&mut genesis_block);
            // store the block
            blockchain.add_block(genesis_block);
        } else {
            println!("Found existing blockchain in database.");
            // blockchain already exists in the db; just look up the last block hash to get a handle on the end of it
            blockchain.set_last_block_hash(data.unwrap().to_vec());
        }
        Self {
            chain: blockchain,
            owner,
            queue: vec![],
        }
    }

    pub fn queue_record(&mut self, record: SimpleRecord) {
        self.queue.push(record);
    }

    pub fn commence_mining(&mut self) {
        while self.queue.len() >= 5 {
            // Taking the LAST 5 records in queue for the new block.  In a real application
            // we would have a limit on block size and use that limit to determine how many records
            // can be stored in the block.
            let mut recs = self.queue.split_off(self.queue.len()-5);
            // add the mining transaction that pays the miner
            recs.push(SimpleRecord::new(SimpleTransaction::new_mining(self.owner.clone()).serialize()));
            //println!("records to mine: {:?}",recs);
            let last_block = self.chain.get_last_block();
            let mut new_block = SimpleBlock::new(
                last_block.get_hash().clone(),
                *last_block.get_height()+1,
                recs,
            );
            mine_block(&mut new_block);
            self.chain.add_block(new_block);
        }
        println!("All transactions in queue have been mined into the blockchain.  Stopping miner.");
    }

}


/// Increment the nonce and rehash the (proposed) block until the hash meets the difficulty criteria.
pub fn mine_block(block: &mut SimpleBlock) {
    let target: U256 = U256::new(1) << DIFFICULTY;
    loop {
        // println!("trying to mine block {} with nonce {}", block.get_height(), block.get_nonce());
        let blockhash_bytes: [u8; 32] = block.get_hash().as_slice().try_into().unwrap();
        // println!("blockhash_bytes: {}",hex::encode(blockhash_bytes));
        let blockhash_num = U256::from_ne_bytes(blockhash_bytes);
        if blockhash_num < target {
            println!("Mined block {} successfully after computing {} hashes...", block.get_height(), block.get_nonce());
            break;
        } else {
            block.increment_nonce();
        }
    }
}


