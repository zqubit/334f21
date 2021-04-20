# Final Project Machine Problem 2
The goal of this project is to implement checkpointing in the longest chain protocol, which is introduced in lecture 17.

**Due date: 5:00PM, May 6, 2021.**

## An honest node signs checkpoint blocks
Let’s assume there is an honest node whose public key PK is known to all nodes. This honest node runs a full-node client and signs a block’s hash every epoch. This block is called a checkpoint block. An epoch is an inter-checkpoint interval, and you can assume an epoch is a fixed number of blocks, say, 10 blocks, where block number 10, 20, … should be signed by this honest node.

You can make a new data structure/message type that contains PK’s signature so that you don’t need to change your block structure. The PK node can broadcast its signature through the new message type.

## Checkpoints in the longest chain
The consensus rule for every node is simply changed to the longest chain extending the latest checkpoint.

## Recall: Private attack
To demonstrate that the longest chain with checkpoints is resilient to private (51%) attack, you need to use your private attack client in the previous project. More on the experiment section.

## Experiment

Similar to the previous project, you can run nodes (clients) on your local machine. But in this project, we require you to run at least 3 nodes (more on experiment steps). And you can simulate different fractions of adversarial hash power *beta* by changing the parameter *lambda* in miner's API. 

### Experiment steps
Please follow these steps
1. Run 2 honest nodes (connected to each other), one of which holds the public key PK and signs checkpoint blocks. Also, run 1 private attack node, but don’t start mining.
2. Start mining in honest nodes. Let the honest chain grow and be checkpointed for a few checkpoints.
3. Stop the honest mining, but don’t shut down the honest nodes (by miner API, pass in a very large *lambda*).
4. Start the private attack mining, and let the private chain grow longer than the honest chain.
5. Thanks to checkpoints, the private attack never succeeds (you can check that in the honest nodes).



## Submission
This is a project for teams of two. The grading will be based on the project report.

### Report
You need to submit a report that contains the following information.
1. Name and Netid of the team members
2. Brief description of the implementation of the checkpoint
3. Brief description of your experiment setting and steps
4. Experiment result, which is the security of the longest chain with checkpoints. You can report the length of the honest chain with checkpoints, the length of the private chain, and whether the honest chain is replaced by the private chain. 
5. How you split your work

**Please limit the report within two pages.**

