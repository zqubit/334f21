# Midterm Project, Part 2

In this part of midterm project, you are going to implement the **mining** module of Bitcoin client. The mining module, or miner, will produce blocks that solve proof-of-work puzzle.

## Special Notifications

One of the dependencies (namely, `net2`) had a bug until version `0.2.36`. This bug may lead to TCP port binding failures, which would show the following error message in the terminal when you run `cargo run`:
> `ERROR - P2P server error: Invalid argument (os error 22)`

To prevent the above error, make sure your `Cargo.toml` is up to date ([see the latest file here](https://github.com/zqubit/334f21/blob/master/Cargo.toml#L24)). This ensures that you have the working version of `net2`.

## Repository management and submission

1. We suggest you to continue to work on your repo of midterm project. 
2. Submit a report in pdf on Canvas. Please don't submit code.

## Code provided
The following files are related to this assignment.
- *src/miner.rs* - where the mining process takes place.
- *src/api/mod.rs* - an API with which you can interact with your program when it is running.
- *src/main.rs* - the main function of the program. In this part, you need to read and change the code that creates a miner.

To see how the code in these files works, you can run `cargo run -- -vv` and you will see these logs in the terminal
> Miner initialized into paused mode
> 
> API server listening at 127.0.0.1:7000

This means the miner is not started yet, however, you can use API to start it. In a browser (or *curl* command), go to
http://127.0.0.1:7000/miner/start?lambda=1000000

Then you will see this log in the terminal
> Miner starting in continuous mode with lambda 1000000

This means the miner is started and keeps working in the *main mining loop*. We also provide a parameter *lambda* and use it in sleep function inside the main mining loop, since we don't want the CPU to run crazily. Here lambda is 1000000 (micro seconds), so in each iteration of the main mining loop, it will sleep for that long.

`-vv` in `cargo run -- -vv` means the level of logging is 2 (info). With `-vvv` the level is 3 (debug) and you can get more log in the terminal.

## Programming

You have seen that the miner is working in the *main mining loop*, so the programming goal for this part is to prepare the miner and implement the main mining loop.

### Preparation for miner

You need to add required components to **Context** struct in *src/miner.rs*

Specifically, the miner needs the following,
1. Blockchain. Miner calls *blockchain.tip()* and set it as the parent of the block being mined. After a block is generated, it needs to insert the block into blockchain.
2. Network server. This component is already there in the code we provide. After a block is generated, it needs to send the block hash to peers. (Not required in this part.)
3. (Not required in this part) Memory pool. Miner takes transactions from the memory pool and set them as the content.

Hence, in this part, you only need to add blockchain into miner **Context** struct. It is running in another thread (cf. `thread::Builder::new` in line 67), hence we need the thread safe wrapper of blockchain. Please follow these steps,
1. Read the document of [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) and [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) in std crate of Rust.
2. Add `Arc<Mutex<Blockchain>>` to the definition of miner **Context** struct.
3. Add `blockchain: &Arc<Mutex<Blockchain>>` to the argument of *new()* function. Inside *new()* function, use `Arc::clone(blockchain)` to get a clone and pass it to **Context**.

The completed **Context** struct should look like the following:

```rust
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
...
pub struct Context {
    /// Channel for receiving control signal
    control_chan: Receiver<ControlSignal>,
    operating_state: OperatingState,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
    // mempool: Arc<Mutex<Mempool>>,  // not yet required
    // (Optional: you may define other state variables here)
}
```

The function `miner::new` should have the following signature:
```rust
pub fn new(server: &ServerHandle, blockchain: &Arc<Mutex<Blockchain>>) -> (Context, Handle)
```

At last, you need to go to *src/main.rs*, and change the code related to `miner::new`. You need to first create a new **Blockchain**, then turn it into `Arc<Mutex<Blockchain>>` (e.g., `let blockchain = Arc::new(Mutex::new(Blockchain::new()));`), then pass it into function `miner::new`.


### Main mining loop

The main mining loop is the loop that is trying random nonces to solve the proof-of-work puzzle. We have provided the loop with some API code. The actual mining may start from line 114, in which we have "TODO: actual mining" comment.

Before accessing the blockchain struct, we need to firstly grab its lock:
```rust
let mut blockchain = self.blockchain.lock().unwrap();
```
The lock will be _automatically released_ when the corresponding variable's lifetime ends. If you are not sure when the lifetime ends or want explicit control in when to release the lock, you can do the following:
```rust
drop(blockchain);  // Release the lock explicitly like this
```

Next, to build a block, you need to gather a block's fields. In a block header, the fields are gathered as follows,
1. parent - use *blockchain.tip()*
2. timestamp - use `SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()` from `std::time`. This expression is pretty self-explanatory, except `UNIX_EPOCH` refers to 1970-01-01 00:00:00 UTC, and `millis` is short for _milliseconds_.
You can refer [this document](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html) for more information.
3. difficulty - in real life, it is computed from parent and ancestor blocks with some adaptive rule. In this project, we use the simple rule: a static/constant difficulty. This rule just means the difficulty of this block should be the same with that of parent block. You should be able to get parent block's difficulty from blockchain.
4. merkle root - compute it by creating a merkle tree from the content.
5. nonce - generate a random nonce (use *rand* crate) in every iteration, or increment nonce (say, increment by 1) in every iteration. P.S. do you think there is any difference in terms of the probability of solving the puzzle?

As for the block content, you can put arbitrary content, since in this step we don't have memory pool yet. You can put an empty vector, or some random transactions.

After you have all these fields and build a block, just check whether the proof-of-work hash puzzle is satisfied by
```
block.hash() <= difficulty
```
Notice that the above code is conceptually the same as *H(nonce|block) < threshold* in lectures.

If it is satisfied, the block is successfully generated. Congratulations! Just insert it into blockchain, and keep on mining for another block.

## Experiment

After you finish the programming, you will have a program that can mine blocks. The experiment section requires you to run the program with different threshold/difficulty and measure the mining rate.

First, you need to set a difficulty. Since we use static difficulty, it's sufficient to set that of the genesis block. (Recall that the genesis block is created when calling *Blockchain::new()*.) Please run experiments with **at least 3 different difficulty values**. 

Then, start the program **in release version** by running
```
cargo run --release -- -vv
``` 
and call API via browser or curl command: 
```
http://127.0.0.1:7000/miner/start?lambda=0
```
(Setting `lambda=0` makes the miner run at full speed; you may use other values for `lambda` if appropriate.)

After some time, stop miner (or the program), count the number of blocks and calculate the mining rate (block per second). Please run experiments such that the mining rate is not too large or too low. 0.01 to 1000 blocks per second is a reasonable range. (If too low, you have to wait for too long. If too high, you may run out of memory.)

You also need to write the function to get the number of blocks if you don't have one. You can do it in your way. It can be in *src/blockchain.rs*, *src/miner.rs*, and/or *src/api/mod.rs*, etc. 

## Report

Please submit a report in pdf. Please use double spacing between paragraphs and use 11 pt font size. Also please keep it within one page.

In the first paragraph, please state clearly your experiment settings. It should include the difficulty, the lambda parameter, and the duration of your experiment.

The second paragraph is a table of difficulty vs mining rate. There should be at least 3 different difficulty values. The table should have a one-sentence caption.

The third paragraph should give a 1-2 sentence analysis of the results in the table. Especially if you encounter any unexpected result please point it out.

## Advance notice
1. Miner also needs memory pool (and UTXO state perhaps). We will cover them in the future.
2. We will cover network module in the next part. In that part, the miner just needs to follow the protocol when a block is mined. E.g., broadcast the block's hash.
3. You can also see whether your blockchain/longest chain is working after many blocks are mined. Is the longest chain growing? Is the miner generating incorrect blocks such as orphan blocks?
