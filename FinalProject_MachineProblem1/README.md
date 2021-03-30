# Final Project Machine Problem 1
In this MP, you are going to implement private attack and selfish mining based on your midterm project codebase, respectively. Do experiments about the two attacks and finish a report.

## Recall: Private attack

In private attack, we assume there is a single adversarial user. The adversarial user is trying to re-write the last *k* blocks, that is, break the *k*-common prefix property. To do so, it must create a private fork of length equal to or longer than the honest longest chain. Notice that the adversary must also wait until the honest chain is long enough (>=*k*) to reveal its private chain. Please refer lecture 6 for more on private attack. 

## Recall: Selfish mining

In selfish mining, we assume there is a single adversarial user. The adversarial user always mines on the block at the tip of the longest chain, whether the chain is private or public. Upon successful mining, the adversary maintains the block in private to release it at an appropriate time. When an honest miner publishes a block the adversary will release private blocks to override the honest block (if it has private blocks). Please refer lecture 7 for more on selfish mining.


## Experiment


The easiest way to experiment the attacks is to run one honest node (longest-chain node) and one adversarial node on your local machine. The network delay would be very small, though. So you can imagine this is simulating the attack under zero delay (e.g., this setting is analyzed in lecture 6).

You can simulate different fractions of adversarial hash power $\beta$ by changing the parameter *lambda* in miner's API. 

For simplicity, you can also change the honest node implementation such that they break ties in the favor of adversarial blocks.

For private attack, start from the genesis block, and the adversarial node should keep a private chain extending the genesis block, and wait for its length to be *>=k*, and reveal it to the honest node when the private chain is equal to or longer than the honest chain and the honest chain is also of length *>=k*. At the honest node, you need to record the time that the honest chain becomes *>=k*, and the time that the private chain is received (and replace the honest chain). Compute the time interval between these two times. (If the adversary never succeeds in attacking, you can also record this case.)

For selfish mining, the adversary keeps a temporary private chain, if it successfully mines the private chain extending the genesis block before honest miner succeeds in extending the genesis block. Then when the honest miner mines a block extending the genesis block, it reveals one private block at the same level to override that block. The selfish mining keeps going along, and you can choose the duration of your experiment. At the honest node, you need to record the chain quality at the end of experiment (the fraction honest blocks in the longest chain).

We don't require any implementation of UTXO or ledger state that is related to the attacks. The focus of this assignment is the attacks on the longest chain protocol.

If you have another attack other than private attack in terms of breaking the common prefix, or you have another attack other than selfish mining to reduce chain quality, then you can implement your attack and describe it in detail in the report.

## Submission
This is a project for teams of two. The grading will be based on the project report.

### Report
You need to submit a report that contains the following information.
1. Name and Netid of the team members

For the two attacks, respectively:
2. Description of the implementation of attacks
3. Briefly description of your experiment setting and steps
4. Results of: attack time (private attack) and chain quality (selfish mining) under different values of $\beta$. For chain quality, also compare your simulation results with the number in the notes of lecture 7
5. How you split your work

**Please limit the report within two pages.**


## Advance Notice

This is the first MP of final projects that extends your Bitcoin codebase and there will be another MP which is orthogonal to this one. So creating a different git branch for this assignment is a good idea.

