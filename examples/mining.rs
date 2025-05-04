use markov_namegen::{CharacterChainGenerator, RandomTextGenerator};
use rand::Rng;
use simplechain::simple_block::{SimpleRecord, SimpleTransaction};
use simplechain::{simple_chain::SimpleNode, utilities::digest};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    // Create a new blockchain and mine the genesis block:
    let mut node = SimpleNode::drop_and_reinitialize(digest(b"joeclark-phd"));

    // Using my markov_namegen crate, prepare to generate a bunch of random "to" and "from" names:
    let file = File::open("resources/american_surnames.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut namegen = CharacterChainGenerator::builder()
        .train(lines)
        .build();

    // Inject a bunch of transactions(records) into the node's queue:
    let mut rng = rand::rng();
    for _ in 0..500 {
        let from_name = namegen.generate_one();
        let to_name = namegen.generate_one();
        let amount: i32 = rng.random_range(1..1000);
        println!("Queing transaction: from: {}, to: {}, amount: {}", from_name, to_name, amount);
        let transaction = SimpleTransaction::new(digest(from_name.as_bytes()),digest(to_name.as_bytes()),amount);
        let record = SimpleRecord::new(transaction.serialize());
        node.queue_record(record);
    }

    // Begin mining blocks:
    node.commence_mining();

}