# Midterm Project, Part 3

In this part of midterm project, you are going to implement the **network** module of Bitcoin client. The network module is in charge of communicating with other nodes/clients. It forms the peer-to-peer (p2p) network and uses gossip protocol to exchange data, including blocks and transactions. (Transactions will not be covered in this part.)

## Repository management and submission

1. We suggest you to continue to work on your repo of midterm project. 
2. Submit a report in pdf on Canvas. Please don't submit code. One submission for one team is enough.

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

### Gossip protocol

You need to define the gossip protocol, i.e., the behavior when messages are received, in *src/network/worker.rs*.

First, you need to add thread safe wrapper of Blockchain into **Context** struct in *src/network/worker.rs*. It is similar to [previous part](../MidtermProject2). Notice that the server we provide is a multithread one, so please be careful with thread safety.

Then, you can define the gossip protocol introduced in the lecture.
1. For **NewBlockHashes**, if the hashes are not already in blockchain, you need to ask for them by sending **GetBlocks**.
2. For **GetBlocks**, if the hashes are in blockchain, you can get these blocks and send them by **Blocks** message.
3. For **Blocks**, insert the blocks into blockchain if not already in it.
4. Optional. If a block's parent is missing, put this block into a buffer and send **Getblocks** message. The buffer stores the blocks whose parent is not seen yet. When the parent is received, that block can be popped out from buffer and inserted into blockchain.

Hint: `peer.write()` may be useful to send a message.

### Combine with miner

We've defined **NewBlockHashes**, so when miner succesfully generates a new block, just broadcast that message. Hint: in miner, `self.server.broadcast()` may be useful.

## Experiment

After you finish the programming, you will have a program that can mine blocks (implemented in part 2) and exchange blocks via gossip protocol. The experiment section requires you to run three processes of your program, each with a distinct address/port. Process 2 connects to process 1 and process 3 connects to process 2. Please do NOT connect process 3 to process 1. This setting mimics a network with 3 nodes, although your processes are on the same machine.

For mining, you need to set lambda and difficulty. You can choose proper lambda and difficulty to make mining rate in a reasonable range, e.g. around 1 block per second.

Start mining in three processes at the same time. Then, after running for some time, stop miner. You should check (by human) whether the three processes have the same set of blocks and the same longest chain.

You need to implement a counter for miner, and a counter for blockchain. You need to report the number of blocks that is mined by each process, and the number of blocks in blockchain in each process.

Note: remember to start the program **in release version**, which runs much faster than debug version.

## Report

Please submit a report in pdf. Please use double spacing between paragraphs and use 11 pt font size. Also please keep it within one page.

Firstly, the report should have both teammate's name and netid. Then you need to write the following paragraphs.

In the first paragraph, please state clearly your experiment settings. It should include the difficulty, the lambda parameter, and the duration of your experiment.

The second paragraph is a table of number of blocks. It should look like the following. The table may come with a one-sentence caption.

|Process|1  |2  |3  |
|---|---|---|---|
|# Blocks mined|   |   |   |
|# Blocks in blockchain|   |   |   |

The third paragraph should give a 2-sentence analysis of the results in the table. Firstly check whether numbers of blocks in blockchain is consistent among 3 process, if not, try to explain. Secondly, check whether the sum of blocks mined by 3 processes equals to the number of blocks in blockchain, if not, try to explain.

In the last paragraph, please use one sentence to describe how you divide your work.

## Advance notice
1. When a block is received, it should be validated/checked first. We will cover this in the future.
2. Communication of transactions will be covered in the next part.
2. The buffer is optional in this part, and will be covered in the next part.
