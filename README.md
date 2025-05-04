# simplechain

Just an experiment in creating a blockchain that can serve as a sort of tamper-proof database through the use of a proof-of-work mechanism.  The basic data type, called SimpleRecord, could be anything that can be serialized into bytes.  A SimpleBlock will hold any number of SimpleRecords up to a target block size.  A mining Node will continually do proof-of-work to create a SimpleChain of hashes to secure the data.  (An attacker would have to do a comparable amount of work in order to forge a blockchain downstream from any change they made.)

## Example

To see the blockchain miner in action, clone this repo and enter:

    cargo run --example mining

## Creating a new blockchain

See `/examples/genesis.rs`.  To create a new blockchain, you need to create a "node" that can mine and add to the chain.  Method `initialize` will re-connect to an existing
one if it's already been started on disk, or you can use `drop_and_reinitialize` to ensure that you start a new one.  The hash you give it will represent the owner's wallet
account and mining rewards will be given to that wallet.

    // Create a new blockchain and mine the genesis block:
    let node = SimpleNode::drop_and_reinitialize(digest(b"joeclark-phd"));

## Automated mining

See `/examples/mining.rs`.  First you need to add a number of records to the node's transaction queue.  In the example, I generate 500 randomly generated transactions. 
Transactions are wrapped in the `SimpleRecord` struct.  My idea with this abstraction was that this code could be a "generic" blockchain that could store any serializable
data, not necessarily just financial transactions.

    let transaction = SimpleTransaction::new(digest(from_name.as_bytes()),digest(to_name.as_bytes()),amount);
    let record = SimpleRecord::new(transaction.serialize());
    node.queue_record(record);

Be sure to queue at least 5 records, because the miner mines blocks with five transactions each from the queue, until the queue is empty (or has fewer than 5, anyway).
Commence mining with this appropriately-named method:

    node.commence_mining();

## Mining difficulty

Mining difficulty is set in the DIFFICULTY constant of `simple_blockchain.rs`:

    /// To "mine" a block, its hash (as a binary number) must be below 2^DIFFICULTY.
    /// DIFFICULTY can be between 1 and 255, larger numbers meaning lower difficulty.
    const DIFFICULTY: u8 = 243;

With a difficulty of 243, mining a block takes up to a few seconds on my computer.

## What this project does not do

This project has been to develop a blockchain and the "miner" that builds it up into a tamper-resistant data store.  If I were to expand this into a tool that other people could use (for a cryptocurrency, let's say), the next things I would need to implement would be:

- Exploring/querying the blockchain to retrieve transaction info,
- Verification of transactions against "wallet" balances before mining,
- Ways for peer nodes to communicate, catching each other up on history and notifying each other of new nodes,
- An API for outside programs to submit new transactions to the queue,