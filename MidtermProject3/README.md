# Midterm Project, Part 3

In this part of midterm project, you are going to implement the **network** module of Bitcoin client. The network module is in charge of communicating with other nodes/clients. It forms the peer-to-peer (p2p) network and uses gossip protocol to exchange data, including blocks (and transactions, in future parts).
We will be combining all the work so far to make a functioning data blockchain!

## Repository management and submission

1. We suggest you to continue to work on your repo of midterm project. 
2. Submit a report in pdf on Canvas. Please don't submit code.

## Code provided
The following files are related to this assignment.
- *src/network/message.rs* - defines the types of messages.
- *src/network/worker.rs* - defines the behavior after receiving messages.

In other files of *src/network/*, we provide a multithread tcp socket server. The default number of thread is 4 and you can change it by parameter `--p2p-workers`. To see how the network server works, you can start two processes of your program by running these two commands respectively
```
cargo run -- -vvv --p2p 127.0.0.1:6000 --api 127.0.0.1:7000
cargo run -- -vvv --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000
```

`--p2p` parameter means that the first process will listen on 127.0.0.1:6000 and the second process will listen on 127.0.0.1:6001.

`-c` parameter means that the second process will try to connect to 127.0.0.1:6000, which is the address of the first process.

On the first process, you can see this log, indicating that the first process accepts connection from the second process.
> New incoming connection from ...

We also provide an API to do ping/pong. You can run
`http://127.0.0.1:7000/network/ping` to send a ping message from the first process to the second process. You will also see a debug log about the ping/pong message.

Ping/pong messages are defined in *src/network/message.rs*, and the behavior after receiving messages are defined in *src/network/worker.rs*. Please read them since you are going to write your own messages.

Notice: the connection is bidirectional, so after process 2 connects to process 1, you don't need to make process 1 create another connection to process 2.

## Programming

You will define a few message types and the behavior when they are received.

### Message types

You need three message types. You can define them in *src/network/message.rs*.

1. NewBlockHashes(Vec\<H256\>), similar to *inv* in lectures
2. GetBlocks(Vec\<H256\>), similar to *getdata* in lectures
3. Blocks(Vec\<Block\>), similar to *block* in lectures

Your enum `Message` should look like this:
```rust
pub enum Message {
    Ping(String),
    Pong(String),
    NewBlockHashes(Vec<H256>),
    GetBlocks(Vec<H256>),
    Blocks(Vec<Block>),
}
```

### Gossip protocol

You need to define the gossip protocol, i.e., the behavior when messages are received, in *src/network/worker.rs*.

First, you need to add a thread safe wrapper of Blockchain into **Context** struct in *src/network/worker.rs*, and then change the related functions accordingly. It is similar to [previous part](../MidtermProject2). Notice that the server we provide is a multithread one, so please be careful with thread safety. Also note that _all workers and the miner inside one node (i.e. process) will share the same blockchain_.

Your struct `Context` should look like this:
```rust
pub struct Context {
    msg_chan: channel::Receiver<(Vec<u8>, peer::Handle)>,
    num_worker: usize,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
    // mempool: Arc<Mutex<Mempool>>,  // not yet required
    // (Optional: you may define other state variables here)
}
```

Then, you can define the gossip protocol introduced in the lecture (inside the `worker_loop` function).

#### 1. NewBlockHashes:
This message will be either originated from the miner when it successfully mines a block, or received from another peer relaying the blocks.

Upon receiving **NewBlockHashes**, if the hashes are not already in blockchain, you need to ask for them by sending **GetBlocks**.

#### 2. GetBlocks:
Upon receiving **GetBlocks**, if the hashes are in blockchain, you can get these blocks and send them by **Blocks** message.

#### 3. Blocks:
Upon receiving **Blocks**:
- Check if each block is already in the blockchain. If so, skip that block; otherwise, check if that block is valid before inserting it into blockchain. We will discuss the validity checks in the following subsections.
- Finally, you need to make a broadcast of **NewBlockHashes** message when receiving new blocks in **Blocks** message. **NewBlockHashes** message should contain hashes of blocks newly received and accepted.

#### 3.1. PoW validity check

Add code to check the PoW validity of a block by checking if:

- `block.hash() <= difficulty`. (Note that difficulty is a misnomer here since a higher 'difficulty' here means that the block is easier to mine).
- Difficulty in the block header is consistent with your view. We have a fixed mining difficulty for this project, thus, this would just involve checking if the difficulty equals the genesis block's difficulty.

If the check fails, it indicates that the block is corrupted or dishonest. You should ignore the block instead of adding it to your blockchain.

#### 3.2. Parent check

- Check if the block's parent exists in your local copy of your blockchain, if the parent exists, insert the block to your blockchain.
- If this check fails, you need to add the block in an 'orphan buffer'. The buffer stores the blocks whose parent is not seen yet. Also, you need to send **GetBlocks** message, containing this parent hash.

#### 3.3. Orphan block handler

Check if the newly processed block is a parent to any blocks in the orphan buffer. If that is the case, remove the blocks from the orphan buffer and insert them one by one. Note that inserting a block can make other orphan blocks, i.e. its children, ready to be inserted consequently. This step should be repeated until no more orphan blocks can be processed (e.g., using a recursive function, or a loop or whatever).

### Combine with miner

We've defined **NewBlockHashes**, so when the miner successfully generates a new block, just broadcast that message.

### Implementation Hints
- Both the miner and the workers can use `self.server.broadcast(...)` to broadcast a message.
- In the `worker_loop`, use `peer.write(...)` to respond to the peer who sent the message (note how `Message::Ping` is responded).
- Inside the worker loop, Use `self.blockchain.lock().unwrap()` to obtain the lock of blockchain before accessing it (similar to [previous part](../MidtermProject2)).
- The orphan buffer should be shared by multiple worker threads. The easiest way to do this is probably to make it a part of the `Blockchain` struct.
- One way to implement the orphan buffer is to use a `HashMap<H256, Vec<Block>>` that maps a parent hash to a list of orphan blocks that have that parent.

## Experiments

After you finish the programming, you will have a working data blockchain that can mine blocks (implemented in part 2) and exchange blocks via gossip protocol. We call this blockchain a data blockchain since we are not adding any meaningful transactions or transaction validation at this stage yet. The experiment section requires you to run three processes of your program, each with a distinct address/port. Process 2 connects to process 1 and process 3 connects to process 2. Please do NOT connect process 3 to process 1. This setting mimics a network with 3 nodes, although your processes are on the same machine.

Below is an example of commands that meet the above requirement:
```
cargo run --release -- -vvv --p2p 127.0.0.1:6000 --api 127.0.0.1:7000
cargo run --release -- -vvv --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000
cargo run --release -- -vvv --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001
```

For mining, you need to set lambda and difficulty. You can choose proper lambda and difficulty to make mining rate in a reasonable range, e.g. around 1 block per second.

Before the experiment, please make sure the *timestamp* field in your block reflects the real time when this block is created.

Start mining in three processes at the same time. Then, after running for some time, stop miner. You should check (either by human or by code) whether the three processes have the same set of blocks and the same longest chain.

The following data need to be collected:

- You need to implement a counter for miner, and a counter for blockchain. You need to report the number of blocks that is mined by each process, and the number of blocks in blockchain in each process.
- You need to compute the block delay in your code, for received blocks (instead of mined blocks). It is computed by `now - timestamp` where `now` is the time the block is received by the network module, and `timestamp` is the field inside header, which reflects the real time of block creation.
- We also require you to measure the size of your blocks. It can be done by serializing it and checking the serialized vector's length.

Note: remember to start the program **in release version**, which runs much faster than debug version.



## Report

Please submit a report in pdf. Please use double spacing between paragraphs and use 11 pt font size. Also please keep it within one or two pages.

In the first paragraph, please state clearly your experiment settings. It should include the difficulty, the lambda parameter, and the duration of your experiment. Also state the size of blocks. If size varies, report average size.

The second paragraph is a table of number of blocks. It should look like the following. The table may come with a one-sentence caption.

|Process|1  |2  |3  |
|---|---|---|---|
|# Blocks mined|   |   |   |
|# Blocks in blockchain|   |   |   |

The third paragraph should give a 2-sentence analysis of the results in the table.
- Firstly check whether numbers of blocks in blockchain is consistent among 3 process, if not, try to explain.
- Secondly, check whether the sum of blocks mined by 3 processes equals to the number of blocks in blockchain, if not, try to explain.


The fourth paragraph is a table of block delay. It should look like the following. The table may come with a one-sentence caption.

|Process|1  |2  |3  |
|---|---|---|---|
|Average of Block Delay|   |   |   |

The fifth paragraph should give a 3-sentence analysis of the results in the table.
- Firstly check whether the block delay magitude is reasonable, if not, try to explain. Here reasonable means that it should not be more than several seconds, since the communication between processes is on one machine, and the block size is not large.
- Secondly, check whether the delay in process 1 and 3 is larger than that in process 2. And explain it by considering the topology (hint: no connection between process 3 and process 1).
- Thirdly, in process 1 and 3, observe whether the delay values have two distinguishable clusters. And explain it by considering the different route of blocks sent by process 2 and process 1/3.

## Advance notice
1. Communication of transactions will be covered in the next part.
2. In the next part, you are going to make the data meaningful, i.e., expressive for cryptocurrency operations.
3. In this project, we don't consider handling spamming attacks. Orphan buffer may be spammed by blocks from an adversary (not a big issue with real PoW), but we don't require you to solve this problem.
