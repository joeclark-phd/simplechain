# simplechain

Just an experiment in creating a blockchain that can serve as a sort of tamper-proof database through the use of a proof-of-work mechanism.  The basic data type, called Transaction, could be anything that can be serialized into bytes.  A Block will hold any number of Transactions up to a target block size.  A Miner will continually do proof-of-work to create a chain of hashes to secure the data.  (An attacker would have to do a comparable amount of work in order to forge a blockchain downstream from any change they made.)

# Creating a new blockchain

See `/examples/genesis.rs`.  To create a new blockchain, you need to create a "node" that can mine and add to the chain.  Method `initialize` will re-connect to an existing
one if it's already been started on disk, or you can use `drop_and_reinitialize` to ensure that you start a new one.  The hash you give it will represent the owner's wallet
account and mining rewards will be given to that wallet.

    // Create a new blockchain and mine the genesis block:
    let node = SimpleNode::drop_and_reinitialize(digest(b"joeclark-phd"));

# Mining difficulty

Mining difficulty is set in the DIFFICULTY constant of `simple_blockchain.rs`:

    /// To "mine" a block, its hash (as a binary number) must be below 2^DIFFICULTY.
    /// DIFFICULTY can be between 1 and 255, larger numbers meaning lower difficulty.
    const DIFFICULTY: u8 = 243;

With a difficulty of 243, mining a block takes up to a few seconds on my computer.