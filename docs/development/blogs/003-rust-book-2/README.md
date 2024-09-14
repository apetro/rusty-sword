# More Rust Book

Okay, the concept for this work session is I keep reading the [Rust Book](https://doc.rust-lang.org/book/) until I get to something more I can use.

In the prior post I followed the guessing game in Chapter 2 enough to roll a d20. Baby steps.

The Rust Book goes on with the guessing game example, demonstrating some error handling, parsing, comparing, and looping. All good stuff. All not really stuff I'm ready to use yet. I suppose I could get a core input loop going.

```rust
    let mut action = String::new();
    loop {
        println!("What does your character do?");
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        println!("{character_name} does {action}.");
        println!();
    }
```

This demonstrates that I need to be doing some trimming on those inputs.

Adding

```rust
    let character_name = character_name.trim();
```

does it, but I don't know if that's idiomatic.

```shell
$ cargo run
   Compiling rusty-sword-main v0.1.0 (/Users/apetro/vcs/github/apetro/rusty-sword/code/rusty-sword-main)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/rusty-sword-main`
What is your character's name?
Bob
Greetings, Bob.
You rolled 19.
What does your character do?
fly
Bob does fly.

What does your character do?
throw
Bob does fly
throw.

What does your character do?
^C
```

Wait, what? That read_line contatenates rather than replaces,
so I need to move the `let mut action = String::new();`
into the loop to reset it each time through.

That's about all I'm getting out of this guessing game for now.

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../002-rust-book/README.md): Diving into the Rust Book
+ [Next post](../004-simpler-file-tree/README.md): simplifying the file tree
