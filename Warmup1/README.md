# Warmup 1

Hi, welcome! This is your first assignment of this course. The goal of this assignment is to let you get familiar with the **Rust** programming language. We will use Rust throughout this course so it is a good idea to start to learn it as early as possible.

## Introduction

We expect that you are familiar with at least one programming language so that you have some experience with programming. If you don't know Rust language, it is totally okay since in this assignment, you'll self-teach Rust language by reading the documents of:

- the Rust language itself;
- Cargo, the rust package manager and building tool;
- Rust standard libraries;
- Rust crates.

Then you will write a simple project in Rust and run it. If you are already familiar with Rust, this simple project will take less than 30 minutes!

**Notice that this is an individual assignment. You should finish it on your own.**

## Reading 
Please read [Rust by example](https://doc.rust-lang.org/rust-by-example/) to learn Rust grammar. We suggest you to read the entire document to get an overview of Rust. For this assignment, Chapters 1-4 and 16.1 in this document are highly recommended.

Please read [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/) to learn Cargo, the Rust package manager and building tool. After reading chapter 1, you'll be able to install Rust and Cargo, and run a Rust project.

For [Rust standard crate](https://doc.rust-lang.org/stable/std/), we recommend you to learn two very important structs: **String** and **Vec**.

You can learn about other public crates here: [https://docs.rs/](https://docs.rs/). A *crate* just means a library or a package, and can be managed by Cargo. You will learn how to use the following crate:
- [ring](https://docs.rs/ring/0.16.9/ring/), a cryptographic crate. Specifically, you need to learn how to do SHA256 hash.
- [serde](https://docs.rs/serde/1.0.104/serde/) and [bincode](https://docs.rs/bincode/1.2.1/bincode/), serialization crates. Specifically, you need to learn how to encode an object into bytes.

For these crates, their github page or homepage may also be helpful. Feel free to read them.

## Programming 
You will create a Rust project using Cargo. We do not provide template code for this assignment and you need to create a prjoect by yourself. Also you don't need to clone this repo for this assignment. You need to add the following crates to your project dependencies:
- ring
- bincode
- serde
- hex
- log (for formatted logging, optional)

You need to write the main function of the project that does the following:

1. create a **String** that contains your name.
2. convert it to bytes.
3. use SHA256 hash function in *ring* crate to compute the hash value of your name.
4. use *hex* crate to encode hash value to hex format.
5. define a struct named **NameHash** and create an instance of this struct that contains both your name and the hex foramt hash.
6. derive Debug trait on **NameHash**.
7. derive Serialize and Deserialize trait on **NameHash** (see *serde* crate on *docs.rs*, also see [this](https://serde.rs/derive.html)).
8. serialize the **NameHash** instance into bytes using *bincode* crate.
9. deserialize bytes back to the instance using *bincode* crate.
10. print on screen the serialized bytes and the deserialized instance using Debug format (hint: use "{:?}" instead of "{}").

The expected output of command `cargo run` would look like the following, where the first line is the serialized bytes and the second line is the instance:<a name="output">
```
   Compiling hello_world v0.1.0 (/home/user/Documents/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/hello_world`
[10, 0, 0, 0, 0, 0, 0, 0, 74, 111, 104, 110, 32, 83, 109, 105, 116, 104, 64, 0, 0, 0, 0, 0, 0, 0, 101, 102, 54, 49, 97, 53, 55, 57, 99, 57, 48, 55, 98, 98, 101, 100, 54, 55, 52, 99, 48, 100, 98, 99, 98, 99, 102, 55, 102, 55, 97, 102, 56, 102, 56, 53, 49, 53, 51, 56, 101, 101, 102, 55, 98, 56, 101, 53, 56, 99, 53, 98, 101, 101, 48, 98, 56, 99, 102, 100, 97, 99, 52, 97]
NameHash { name: "John Smith", hash: "ef61a579c907bbed674c0dbcbcf7f7af8f851538eef7b8e58c5bee0b8cfdac4a" }
```
</a>

### Advance Notice
This assignment has some basic Rust programming tasks. Each task is closely related to future projects in the course. 

- The goal of step 3 is to let you use SHA256 function, and you're going to use it throughout the course assignments.
- The goal of step 5 is to let you know how to define a struct. When you implement a Bitcoin client, almost everything is a struct, e.g. Block, Transaction, Unspent Transaction Output (UTXO), etc.
- The goal of steps 7, 8, 9 is to let you know how to encode any object into bytes. In network socket communcation, the messages are first encoded, then transmitted, and at last decoded at the receiver. In your  Bitcoin client, structs such as Block or Transaction need to be encoded and decoded.

### Submission
All you need to submit is the output of command `cargo run` on Canvas. You don't need to submit codes.

Run command `cargo run` in terminal for your project and copy the output on the screen. An example of output is in the [previous paragraph](#output).
