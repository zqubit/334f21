# Midterm Project, Part 4

In this part of the midterm project, we will be combine last 3 week's work to make a functioning data blockchain. Most of this week's work will be combining mining, network and blockchain module. You will need to add PoW validation and a block buffer to handle orphan blocks.

## Repository management and submission

1. We suggest you to continue to work on your repo of midterm project. 
2. Submit a report in pdf on Canvas. Please don't submit code. One submission for one team is enough.

## Code provided
No additional code will be provided for this assignment since the work is mostly related to adding compatibility across modules.

## Programming

### Relay blocks

Here we ask you to extend the gossip protocol so that the propagation of blocks can be faster. In *src/network/worker.rs*, you need to make a broadcast of a **NewBlockHashes** message when receiveing new blocks in **Blocks** message. **NewBlockHashes** message should contain hashes of blocks newly received.

### Checks
When processing a new block in *src/network/worker.rs*, please add the following checks.

#### PoW validity check

Add code to check the PoW validity of a block by checking if:

1. PoW check: check if `block.hash() <= difficulty`. (Note that difficulty is a misnomer here since a higher 'difficulty' here means that the block is easier to mine).
2. Difficulty in the block header is consistent with your view. We have a fixed mining difficulty for this project, thus, this would just involve checking if difficulty equals the parent block's difficulty. (This step should be done after parent check.)

#### Parent check

1. Check if the block's parent exists in your local copy of your blockchain, if the parent exists, add the block to your blockchain.
2. If this check fails, you need to add the block in an 'orphan buffer'. You may need to create a struct for this.
3. Updated. If this check fails, also send **GetBlocks** message, containing this parent hash. (This is the same as part 3 instructs.)

#### Orphan block handler

Check if the new processed block is a parent to any block in the orphan buffer, if that is the case, remove the block from orphan buffer and process the block. This step should be done iteratively. I.e., a block makes a former orphan block be processed, and the latter makes another former orphan block be processed, and so on.

## Experiment

After you finish the programming, you will have a working data blockchain. We call this blockchain a data blockchain since we are not adding any meaningful transactions or transaction validation at this stage yet. If you like, you can put data into transactions, who will be carried by blocks and be on-chain eventually.

Before the experiment, please make sure the *timestamp* field in your block reflects the real time when this block is created.

We require you to do an experiment whose setting is similar to last part. The experiment requires you to run three processes of your program, each with a distinct address/port. Process 2 connects to process 1 and process 3 connects to process 2. Please do NOT connect process 3 to process 1.

For mining, you need to set lambda and difficulty. You can choose proper lambda and difficulty to make mining rate in a reasonable range.

Start mining in three processes at the same time. Then, after running for some time, stop miner. You should check (by human) whether the three processes have the same set of blocks and the same longest chain.

You need to compute the block delay in your code, for received blocks (instead of mined blocks). It is computed by `now - timestamp` where `now` is the time the block is received by the network module, and `timestamp` is the field inside header, which reflects the real time of block creation.

We also require you to measure the size of your blocks. It can be done by serializing it and checking the serialized vector's length.

Note: we suggest to start the program in release version, which runs much faster than debug version.

## Report

Please submit a report in pdf. Please use double spacing between paragraphs and use 16 pt font size. Also please keep it within one page.

Firstly, the report should have both teammate's name and netid. Then you need to write the following paragraphs.

In the first paragraph, please state clearly your experiment settings. It should include the difficulty, the lambda parameter, and the duration of your experiment. Also state the size of blocks. If size varies, report average size.

The second paragraph is a table of block delay. It should look like the following. The table may come with a one-sentence caption.

|Process|1  |2  |3  |
|---|---|---|---|
|Avg Block Delay|   |   |   |

The third paragraph should give a 3-sentence analysis of the results in the table.
- Firstly check whether the block delay magitude is reasonable, if not, try to explain. Here reasonable means that it should not be more than several seconds, since the communication between processes is on one machine, and the block size is not large.
- Secondly, check whether the delay in process 1 and 3 is larger than that in process 2. And explain it by considering the topology (hint: no connection between process 3 and process 1).
- Thirdly, in process 1 and 3, observe whether the delay values have two distinguishable clusters. And explain it by considering the different route of blocks sent by process 2 and process 1/3.

In the last paragraph, please use one sentence to describe how you divide your work.

## Advance notice
1. In the next part, you are going to make the data meaningful, i.e., expressive for cryptocurrency operations.
2. In this project, we don't consider handling spamming attacks. Orphan buffer may be spammed by blocks from an adversary (not a big issue with real PoW), but we don't require you to solve this problem.
