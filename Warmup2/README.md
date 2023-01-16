Until the assignment is posted on Canvas, this description is subject to change.

# Warmup 2

In this assignment, you will implement some crypto-primitives and basic data structures. You will need the code that we provide in this repo. Please follow the instructions.

**Notice that this is an individual assignment. You should finish it on your own.**

## Repository management and submission:
1. Fork this public repo (https://github.com/zqubit/334f21.git) as a *private* repo to avoid your code being seen by others.  **We recommend you refer to [this post](https://medium.com/@bilalbayasut/github-how-to-make-a-fork-of-public-repository-private-6ee8cacaf9d3) when creating your private repo.**  WE WILL DEDUCT POINTS FROM YOUR GRADES IF YOU DO NOT KEEP YOUR REPO PRIVATE.  When in doubt, double check with the teaching assistants.  Note: We are also going to use the same repo for subsequent tasks, which means you don't need to fork this repo again in the future.
2. Students can run tests (by command `cargo test`) provided in the code to check the validity of their implementation. However, passing these tests doesn't guarantee getting full grades. 
3. After finishing this warmup, before submitting your code, please remove your tests if you write any. Please leave `mod tests` at its original code, and this is for TAs to conveniently do the grading.
4. After step 3, please push to your private github repo, and click `Code` -> `Download ZIP` from the repo's webpage to download a zip file.
5. Rename it to your netid as `netid.zip`. Upload the zip file on Canvas. Please check your file size and it should be less than 1 MB or 2 MB.
6. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
We have provided incomplete code for implementing some crypto-primitives and data structures like merkle tree, signature and transactions. The following files are related to this assignment and you should read them.
1. _src/crypto/hash.rs_ - Provides __H256__ struct(32 byte array),  __Hashable__ trait, with its implementation for H256. 
2. _src/crypto/keypair.rs_ - function to randomly generate keypair.

You don't need to write anything in the above two files.

3. _src/crypto/merkle.rs_ - struct definition of **MerkleTree** struct and the related function declaration
4. _src/transaction.rs_ - struct definition of **Transaction** struct and function declaration for __sign()__ and __verify()__ .

You will write your code in the above two files.

As for other files in the repo, you don't have to worry about them in this assignment. They may appear in future assignments.

## Programming
After you fork this repo, the first thing we suggest is to run command `cargo test` to see whether the code is compiling on your machine. (If compiling has error, please check the version of cargo to be the latest stable.) If the compiling is successful, you will see something like this:
```
running X tests
test XXX ... FAILED
test XXX ... FAILED
```
It's expected that tests fail with the code we provide. After you finish this assignment, some of the tests will pass (specifically, the 4 tests below should pass):
- `crypto::merkle::tests::proof`
- `crypto::merkle::tests::root`
- `crypto::merkle::tests::verifying`
- `transaction::tests::sign_verify`

You need to implement the missing parts in the code. They include the following.

### Transaction and signature
This part is in file _src/transaction.rs_.
1. You need to fill in the **Transaction** struct. Up to now we don’t expect the cryptocurrency and payment to be functional, so you can put any content in transactions. A simple choice is to put some **Input** and **Output** inside transactions and you can define **Input** and **Output** by yourself.
2. You need to fill in the **sign** and **verify** function. These two function should sign and verify the digital signature of the **Transaction** struct. Please use **ring** crate (actually the crate is already used in the heading of this file). The code we provide contains some `unimplemented!()` and you can delete it and write your own code.
3. A tricky part about transaction and signature is how you put them together. One possible way is to create another struct called **SignedTransaction**. The other way is to declare a field in transaction called *signature*, which will be empty if there is no signature. Feel free to design your own way. (We don’t require you to define a struct to carry signature in this assignment.)
4. For testing, you need to fill in the function **generate_random_transaction()** which will generate a random transaction on each call. It should generate two different transactions on two calls. We require this since we are going to use this function many times in our test and grading. Just a suggestion: don’t generate a very large transaction, since it will slow down our test platform. Again, there is `unimplemented!()` and you can delete it.
5. We provide a small test function named **sign_verify()**. After you finished steps 1-4, you can run `cargo test` and you can see the result of this function in the output. It will look like the following.
```
test transaction::tests::sign_verify ... ok
```
To test your code, you are free to write more tests.

### Merkle Tree
This part is in file *src/crypto/merkle.rs*. You need to complete the merkle tree struct and some functions. We covered merkle tree briefly in the lecture. You can also find a good article about it [here](https://nakamoto.com/merkle-trees/). Specifically, the functions you need to implement are:
1. *new()* - this function takes a slice of Hashable data as input, and create the merkle tree. 
2. *root()* - given a merkle tree, return the root. The computation of the root is inside *new()*, this function should just return the root.
3. *proof()* - given a merkle tree, and also given the index, this function returns the proof in the form of a vector of hashes.
4. *verify()* - given a root, a hash of datum, a proof (a vector of hashes), an index of that datum (same index in *proof()* function), and a leaf_size (the length of leaves/data in *new()* function), returns whether the proof is correct.

*We have also provided some **starter code** in Warmup2/merkle.rs*, where the `new()` function is implemented except two helper functions `hash_children` and `duplicate_last_node`. You will probably like to refer to the starter code if you are unfamiliar with Rust, but it is up to you whether to adopt the starter code or not.

We provide some small test functions in this file and you can run `cargo test`. In these test functions, we also provide a brief explanation about the expected computation.

*new()* function can take any Hashable data, but for simpilicity we will test merkle tree over **H256**, whose Hashable trait is already provided inside *src/crypto/hash.rs*.

A tricky part about *new()* is when the input length is not a power of 2, you will need some more steps to create the merkle tree as follows.
> Whenever a level of the tree has odd number of nodes, duplicate the last node to make the number even.

## Implementation Hints
**It is highly recommended that you use an IDE when programming on this project.** Typing and ownership management in Rust can be tricky, and an IDE is helpful in detecting problems and providing suggestions while you code, especially as the code size grows.
 
Implementation hints on `transaction.rs`:
- For this assignment, you can define the struct `Transaction` arbitrarily as long as it's non-empty. The content in that struct does not matter.
- The `sign` and `verify` functions you are going to implement should just be a couple of lines of code built upon the functions provided in the `ring` crate. [Here]( https://docs.rs/ring/latest/ring/signature/index.html#signing-and-verifying-with-ed25519) is an example showing the usage of these functions. Pay attention to the `sign` and `verify` functions in that example. Below are more detailed instructions:
  - For `sign`: Firstly, serialize the Transaction `t` using the `bincode` crate. Pass a _reference_ of the resulting byte vector to `key`'s [`sign` method](https://docs.rs/ring/latest/ring/signature/struct.Ed25519KeyPair.html#method.sign) to obtain the signature.
  - For `verify`: As in `sign`, the Transaction `t` need to be firstly serialized. Refer to the last few lines in the example above to create an `UnparsedPublicKey` object and apply [the `verify` method](https://docs.rs/ring/latest/ring/signature/struct.UnparsedPublicKey.html#method.verify) on it. This should return a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html). Apply the `is_ok()` method to turn it into a `bool`.
- The simpliest way of generating random numbers in Rust is to use the generic function `random` in the `rand` crate. For example, calling `rand::random::<u32>()` will produce a random 32-bit unsigned integer.
 
Implementation hints on `crypto/merkle.rs`:
- There are more than one ways to implement a Merkle tree. For example:
  1. The most intuitive way is to implement it as a recursive binary tree. In Rust, both the left subtree and the right subtree of a tree (of type `T`) should have a wrapped type like `Option<Box<T>>`. Here, `Box` is required because the subtrees are dynamically sized, and `Option` is needed because the subtrees can be empty. (**We have provided some starter code for this method: see `merkle.rs`**. If you are new to Rust, it is highly recommended to read the starter code and then build upon it.)
  2. Binary trees can also be implemented using non-recursive data structures, for example using an array (refer to https://en.wikipedia.org/wiki/Binary_tree#Arrays). In Rust, `Vec<T>` is an array of changeable size containing elements of type `T`.
- To concatenate two slices `a` and `b`, use `[a, b].concat()`. Several types in Rust can be automatically converted to slice, _not_ including H256. You can use `.ref()` to explicitly convert `H256` into a byte slice. The concatenated result no longer fits in an `H256`, so we need a more general way for hashing. See the hint below.
- Recall that we can use the function `ring::digest::digest` with the SHA256 algorithm to calculate the hash of an arbitrary byte slice. To convert the resulting hash (of type `Digest`) into the type `H256`, the simpliest way is to call ...`.into()`.
- (Related to the starter code) Assuming a reference to non-leaf `node` has type `&MerkleTreeNode`, the simplest way to get a reference to its (e.g., left) child is via `node.left.as_ref().unwrap()`. If it's unknown whether a `node` has a left/right child or not, it is recommend to use the [`match`](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) syntax: e.g., `match &node.left { ... }`
- In the `proof` and `verify` methods, you probably need [loops](https://doc.rust-lang.org/reference/expressions/loop-expr.html) for traversing down the Merkle tree. The Merkle proof/verification algorithm can be implemented with very basic Rust syntax (except the problems as elaborated above).

## Advance Notice
- At the end of this entire project, you will implement a functional cryptocurrency client. We don't require you have a functional transaction struct in this assignment, but please start to think what transaction struct should be. Also please start to think about UTXO since it is closely related to transaction.
- This code base provides other files that will help you build a blockchain client. If you want to run the main program and see what is going on, you can run `cargo run -- -vv`. Currently the main program just stucks at a loop.
