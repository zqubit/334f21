# Midterm Project, Part 6

This is the last part of midterm project, and you are going to finish the Bitcoin client. You need to maintain a state for the ledger that the blockchain creates and add all the necessary checks related to it. 

** Submit a report on Canvas. One submission for one team is enough. Remember to include both author names if you are a team of two. **

## Programming

### Transaction Checks
In last part, you include transactions into blocks. However, in order to prevent misbehavior such as double spending, you need to add the following checks:

#### Transaction signature check
- Check if the transaction is signed correctly by the public key(s).
- In UTXO model, check the public key(s) matches the owner(s)'s address of these inputs. (This step needs struct **State**, see below.)
- In account based model, check if the public key matches the owner's address of the withdrawing account. (This step needs struct **State**, see below.)

#### Spending check
- In UTXO model, check if the inputs to the transactions are not spent, i.e. exist in **State** (see below). Also check the values of inputs are not less than those of outputs.
- In account based model, check if the balance is enough and the suggested account nonce is equal to one plus the account nonce. This check also needs **State** (see below).

### State

Ledger state, or **State**, is a collection of all the required information to check transactions.  

- In UTXO model, **State** should contain all the unspent transaction outputs. The format of an unspent transaction output may contain *(transaction hash, output index, value, recipient)*. Output index refers to the index in transactions (remember transactions are multi-output.) Recipient refers to the recipient address of that output, and is used as the owner of that unspent transaction output.
- In account based model, **State** should contain all the accounts' information. It may contain *(account address, account nonce, balance)*.

To access data conveniently, we recommend use HashMap to store State. In UTXO model, we recommend `HashMap<(transaction hash, output index), (value, recipient)>`. In account based model, we recommend `HashMap<account address, (account nonce, balance)>`.

#### State update
When executing a block, i.e., executing transactions in that block, we need to update the state.
- In UTXO model, remove those *inputs*, and add *outputs* to the state.
- In account based model, change account's nonce and balance. Create new accounts if you need.

#### Initial state (ICO)
You can do initial coin offering (ICO) by inserting entries into **State** struct.
- In UTXO model, add unspent transaction outputs and specify the recipients to be the addresses you control.
- In account based model, create accounts whose addresses are under your control.

#### State per block
Since there is branching/forking in the blockchain, and the longest chain may change, you need to store one copy of **State** for each block. A copy of **State** for a block refers to the state after executing the block. We recommend using a HashMap-like storage, e.g., `HashMap<block hash, state>`. When you check transactions, you can get the corresponding state from it. When you update state, you do the update on a new state copy, and insert it.

Another way to deal with forking is to implement a reverse state transition corresponding to a transction, say that the longest chain changes from A->B->C->D to A->B->E->F->G, you can perform reverse state transition on blocks D and C and a forward state transition from blocks E, F, G.

#### Note

The above paragraph about **State** is one feasible way to implement it. You have the freedom to do it another way, as long as it enables transaction checks.

### Transaction generator
Transaction generator should generate transactions that pass the checks. It can read the blockchain and the state to ensure that. On different nodes/processes, transaction generator should control different key pairs.

### Transaction Mempool update
After implementing state transition, ensure that the transactions in the mempool are valid with respect to the new state, this is necessary since some transactions may classify as double-spends after the state update, you may need to remove those transactions.

#### Note
We do not ask you to implement transaction fees and mining rewards and the corresponding coinbase transactoin for this project

## Conclusion

Now that you finish the last part, you have a simplified Bitcoin client! With transaction generator simulating user's transactions, the system should run smoothly and securely.

## Submission
The final grading of the project will be based on your project report and demo video.

### Report
You need to submit a report that contains the following information.
1. Name and Netid of the team members
2. Design document
3. How you split your work

The design document is a short description of your codebase and the various design choices you made to ensure that your codebase is resiliant to various attacks like double spending attacks. It is recommended to include a simple design diagram to indicate the interactions between various modules of your codebase.

**Please limit the report within two pages.**

### Demo

You need to submit a two-minute video that demonstrates the execution of your code. (For example, if you use a Mac, you can use QuickTime Player to make a video recording of your computer screen.) For the demo you need to run your code on 3 nodes on the same computer and display the status at each node. Run the experiments at a suitable mining rate and a suitable transaction generation rate so that the video demonstrates the growth of the blockchains at the three nodes. Your video should be at a reasonable pace for the graders to follow without using slow motion. At the same time your video should provide enough information for the grader to judge what requirements you have completed.

More specifics (added on Nov. 27, 2021):

For each of the following bullet your video convincingly demonstrates, you receive 25% of the total credit, up to 100%.
1. Initializing all three nodes and setting them in motion; showing communications between them;
2. periodical traces of a working miner and mempool on each node (which should be succinct and not overwhelm the screen, e.g., printing a one-line trace when a new block is mined/received, or some selected transactions are received);
3. the growth of the blockchain on each node, and that all nodes agree to each other (e.g., showing the hash and height of the tip, and the total count of the blocks);
4. the evolution of the ledger state on each node, and that all nodes agree to each other (e.g., showing the balances of certain accounts if your implementation is account based);
5. occasional invalid transactions are properly rejected and do not crash the program (i.e. the generators should take a small chance to generate invalid transactions).
 
If your bitcoin client is not fully completed, you can still get partial credits by showing how the individual parts work, for example:

6. traces of the generator generating random UTXO or account-based transactions, signing them, adding them to the mempool, and broadcasting their hashes;
7. traces of the network exchanging messages;
8. traces of all sorts of validations when receiving a new block/transaction;
9. traces of one process working alone (instead of three processes working in harmony);
and so on.
