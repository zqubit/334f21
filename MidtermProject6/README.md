# Midterm Project, Part 6

This is the last part of midterm project, and you are going to finish the Bitcoin client. You need to maintain a state for the ledger that the blockchain creates and add all the necessary checks related to it. 

**Due date: 5:00PM, Mar 25, 2021. Please submit a report on Compass2g and record a demo video. One submission for one team is enough. The details of the demo video will be released in Piazza soon.**

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

Another way to deal with forking is to implement a reverse state transition corresponding to a transction, say that the longest chain changes from A->B->C->D to A->B->E->F->G, you can perform reverse state transition on blocks D and C and a forward state transition from blocks E, F, G. You may use diagrams on page 9 and 13 on lecture slides as a reference(these diagrams are not comprehensive and may miss some functionalities depending on your implementation).

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
The final grading of the project will be based on demo video and the project report.

### Report
You need to submit a report that contains the following information.
1. Name and Netid of the team members
2. Design document
3. How you split your work

The design document is a short description of your codebase and the various design choices you made to ensure that your codebase is resiliant to various attacks like double spending attacks. It is recommended to include a simple design diagram to indicate the interactions between various modules of your codebase.

**Please limit the report within two pages.**

### Demo

You will demo the running codebase in the video. The demo will consists of running the codebase on 3 nodes in the same machine and checking the status at every node. Run the experiments at different mining rates and different transactoin generation rates respectively. The details of the demo will be released in Piazza soon.

