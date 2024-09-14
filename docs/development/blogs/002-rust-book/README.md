# The Rust Book

Okay, the concept for this work session is I start reading the [Rust Book](https://doc.rust-lang.org/book/) until I get to something I can use.

Chapter 1, Getting Started, get to the hello world I produced in the prior work session.

Chapter 2, Programming a Guessing Game, adds some building blocks I can start to use.

The guessing game prompt for a guess. I can prompt for a character name.

```rust
use std::io;

fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();

    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    println!("Greetings, {}.", character_name);
}
```

The guessing game uses random numbers. Random numbers are going to end up being useful in this text adventure, so I can slurp that in too, adding the dependency in `Cargo.toml`

```toml
[dependencies]
rand = "0.8.5"
```

Let's roll some dice.

```rust
use rand::Rng;

// ...

    let d_20 = rand::thread_rng().gen_range(1..=20);

    println!("You rolled {d_20}.");
```

```shell
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rusty-sword-main`
What is your character's name?
Bertha
Greetings, Bertha
.
You rolled 16.
```

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../001-hello-world/README.md): hello world
+ [Next post](../003-rust-book-2/): More working through the guessing game in the Rust Book
