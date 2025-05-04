use simplechain::{simple_chain::SimpleNode, utilities::digest};



fn main() {

    // Create a new blockchain and mine the genesis block:
    let _node = SimpleNode::drop_and_reinitialize(digest(b"joeclark-phd"));

}