use simplechain::{simple_blockchain::SimpleNode, utilities::digest};



fn main() {

    // Create a new blockchain and mine the genesis block:
    let node = SimpleNode::drop_and_reinitialize(digest(b"joeclark-phd"));

}