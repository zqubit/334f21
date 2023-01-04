# Midterm Project, Part 5

This part of the Midterm project will deal with including transactions in the codebase. Integrate the transaction structure inside the block content, add network functionality to transaction propagation and adding a transaction mempool to be used by the miner to include transaction content in the block being mined.

## Repository management and submission

1. We suggest you to continue to work on your repo of midterm project. 
2. No submission required.

## Code provided
No additional code will be provided for this assignment.

## Programming

### Transaction network messages

Add the following new messages:
1. NewTransactionHashes(Vec<H256>), similar to NewBlockHashes
2. GetTransactions(Vec<H256>), similar to GetBlocks
3. Transactions(Vec<Transaciton>), similar to Blocks

### Transaction format

You are free to choose any format for transaction structure. We recommend using a transaction structure that is either compatable with the UTXO model in Bitcoin or the account based model in Ethereum. 

- UTXO model transaction: input contains the hash of previous transaction and the index; output contains a recipient address and a value. It can support multiple inputs/outputs in a transaction. You can refer to [Bitcoin](https://en.bitcoin.it/wiki/Transaction) transaction but don't need to adopt its complex scripting language.
- Account based model transaction: it should contain a recipient address, a value, and a account-nonce. It only supports single sender and single receiver. This should be simpler to implement than UTXO model.

Note: address in Bitcoin and Ethereum is a 20-byte array, a.k.a. H160. You should define a struct H160 in *src/crypto/*. The conversion from public key to H160 is different in Bitcoin and Ethereum. We suggest to use the following simple conversion:

> first hash the public key, then take the last 20 bytes.

Now it's time to add **Signature** to transaction. You need to convert the **Signature** struct of *ring* to bytes since it is not serializable. You also need to do the same for public key. Then append the public key and the signature to transaction by
- either create a struct **SignedTransaction** that contains the transaction, the public key, and the signature,
- or define a field/fields in transaction that stores the public key and the signature.

### Checks
When receiving and processing a new transaction in *src/network/worker.rs*, please add the following checks.

#### Transaction signature check

Check if the transaction is signed correctly by the public key(s). 

(Not necessary at this stage.) In UTXO model, also check the public key(s) matches the owner(s)'s address of these inputs. In account based model, check if the public key matches the owner's address of the withdrawing account.

#### Double spend checks

(Not necessary at this stage.) In UTXO model, check if the inputs to the transactions are not double spent. In account based model, check if the balance is enough and the suggested nonce is equal to one plus the account nonce.

#### Add those checks when processing blocks

When receiving and processing a block, also check transactions inside it.

### Transaction Mempool

Create a transaction **Mempool** structure to store all the received valid transactions which have not been included in the blockchain yet.
If a new transaction passes the above checks, add it to the mempool.
**Mempool** will also be used by miner to include transactions in the blocks being mined. The miner will add transactions in the mempool to the block till it reaches the block size limit. You are free to choose the size limit on the blocks. On processing a new block(which is not an orphan or stale), remove corresponding transacitons from the mempool.

Similar to **Blockchain**, you need the thread safe wrapper `Arc<Mutex<>>`.
### Transaction generator

To demonstrate transaction is working well with the client, you need to add transactions into your running client. The transactions can be a simple payment in account based model, or a transaction with just one input and one output in UTXO model. You are free to choose the sender and recipient.

In order to do that, you need to write a transaction generator. You can
- either create a new thread, and generate a transaction periodically,
- or write an API in *src/api/* and call this API externally.

The transaction generator should have network server handle and wrapper of mempool. When a transaction is generated, add the transactions to mempool and broadcast the hash to the network.

**Since you are not storing state (will be covered in the next part), you can create transactions with any random input value.**

## Experiment

Make sure the code is working properly with transactions with valid signatures. Also test it by including invalid signatures.

## Advance notice
1. In the next part, we will need to add state validity to the transaction which corresponds to the double spend checks.
2. We will do ICO in the next part.
3. We will have a demo for this project after next part.
