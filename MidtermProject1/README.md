# Midterm Project, Part 1

Through this midterm project, you are going to build a simplified Bitcoin client. The goal of the client is not to run in Bitcoin mainnet or any public testnet. Instead, the goal is to run it inside your computer and let you have fun with it. You have plenty of freedom of designing and implementing this project.

The midterm project should be based on your code of warmup 2. 
This is the first part of midterm project. You are going to finish the **Blockchain** struct.

## Repository management and submission

1. We suggest you to continue to work on your repo of warmup 2.
2. You can run tests (by command `cargo test`) provided in the code to check the validity of their implementation. However, passing these tests doesn't guarantee getting full grades.
3. After step 2, please push to your github repo, and click `Code`->`Download ZIP` on github to download a zip file.
4. Rename it to your netids as `netid.zip`. Upload the zip file on Canvas. Please check your file size and it should be less than 1MB or 2MB.
5. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The code we provide for midterm project is the same as warmup 2. The following files are related to this assignment.
1. *src/block.rs* - We have provided some referential code [here](https://github.com/zqubit/334f21/blob/master/MidtermProject1/block.rs). If you decide to base your work on the referential code, please read the file above to get familiar with the struct **Block** and fill up the function `default_difficulty()`. You may also opt to design and implement the **Block** struct in your own way.
2. *src/blockchain.rs* - Please finish **Blockchain** struct and some functions related to it in this file. In [this file](https://github.com/zqubit/334f21/blob/master/MidtermProject1/blockchain.rs), we have provided some hints and a couple of extra test cases. Also, it is your freedom to work on, partially adapt, or ignore the referential code above.

If you base your work on the referential code, be sure to copy the code into corresponding files in `src/`.

## Programming

### Block

We have provided the definition of the struct **Block** in the referential code. It is similar to that in Bitcoin, including:
1. parent - a hash pointer to parent block, using **H256** that we provide.
2. nonce - a random integer that will be used in proof-of-work mining.
3. difficulty - the mining difficulty, i.e., the threshold in proof-of-work check. **H256** is used since we provide the comparison function, with which you can simply write `if hash <= difficulty`. **For this assignment, you can set the difficulty to whatever you like in the function `default_difficulty()`**. You will need to adjust the difficulty value in the future.
4. timestamp - the timestamp when this block is generated. This is used to decide the delay of a block in future part of this project.
5. merkle\_root - the Merkle root of data (see below in 6.).

The above fields are also known as **Header**.

6. data/content - the actual transactions carried by this block. It just contains a **Vec** of **Transaction**. Transaction struct is the one you wrote in previous assignment.

The referential code has also provided the following functionalities that you can readily use:
- Notice that to create the Merkle root of **Transaction**, we need the trait **Hashable** for **Transaction**. The way to implement that trait is first serialize it into bytes, then call SHA256 to hash the bytes. We have provided the corresponding code snippet inside the double-slash comments.
- We have also provided the trait **Hashable** for **Block**. The way to hash a block is to hash **Header** rather than **Content**. We first implement **Hashable** for **Header** (Similarly, first serialize it into bytes, then call SHA256 to hash the bytes). When we hash a **Block**, we directly call the hash function of **Header**.
- To help test and debug, we have provided the function *generate_random_block*. This function takes an argument named *parent*. The generated block should contain that *parent*. And the *nonce* should be a random integer. As for content, we simply create some arbitrary transaction(s), e.g. `let transactions: Vec<Transaction> = vec![Default::default()];`. Then, we use the **MerkleTree** struct to calculate the Merkle root of these transactions. The fields such as difficulty and timestamp were arbitrarily chosen since they are unimportant in this part.

### Blockchain

You need to finish a struct named **Blockchain**, which contains the necessary information of a direct acyclic graph (DAG) (in fact, _a tree_ of blocks, in this project) and provides functions related to the longest chain rule. The following functions are required:
1. new() - create a new blockchain that only contains the information of the genesis block. (Define genesis block by yourself. For this assignment, you can define it arbitrarily; the header and the content of the genesis block does not matter.)
2. insert() - insert a block into the blockchain. (For this assignment, you can assume the block always has a valid parent.)
3. tip() - return the last block's hash in the longest chain.
4. all_blocks_in_longest_chain() - return all blocks' hashes, from the genesis to the tip. This function will not be tested in this part, and will be used in debugging in the future.

#### Storage choice

We suggest that you use a **HashMap** in standard crate to store blocks. You can use hash as key and the block as value. It can look up for blocks by hash very conveniently.

You can also store the tip, and update it after inserting a block. If, say, your current tip is hash(B1), and you insert a new block B2. You need to update tip to hash(B2) if and only if the length of chain B2 is *strictly greater* than that of B1.

Since we are following the longest chain rule, you may also store the length/height of each block. And use the height to determine the longest chain. E.g., genensis block has height 0.

You may worry that the client will run out of memory. You can implement with persistent storage such as database, but this is not the point of this project and we suggest to just use in-memory storage.

## Grading

If your programming is working, you will pass the test named *insert_one* and other ones in the referential code (By running `cargo test`.)

We will use other private tests to grade your submission.
The tests will insert several blocks into a new blockchain, and check whether the tip is correct. The tests contain forking/branching scenarios to check the correctness of your longest chain rule.

We will *NOT* call insert function with invalid blocks. Specifically, we will not insert a block whose parent is not already inserted.

## Advance Notice
1. In the future, the **Blockchain** struct will be shared between threads, such as miner thread and network thread. If you want to learn about thread safety in Rust, you can read [this](https://doc.rust-lang.org/book/ch16-01-threads.html). In this project, we will use _Arc_ and _Mutex_ to manage the shared variables, like the blockchain and mempool (in Part 5). In addition, this codebase extensively use _channels_ to send control signals across threads.
2. Our goal is to decouple blockchain status from ledger status, and focus on the former. As a result, we don't involve transaction execution or ledger update or UTXO in this part. They will be handled in future parts.
3. We don't involve proof-of-work check or mining, but we need to prepare for them. So we require fields nonce and difficulty inside a block. You can start to think about how to mine or check blocks.
4. Blockchain struct will be used in multiple places in the future. For example, when you implement a miner, you insert a mined block into blockchain; when you want to mine on the longest chain, you need to get the tip as the block's parent; when you receive a block from p2p network, you insert it.
