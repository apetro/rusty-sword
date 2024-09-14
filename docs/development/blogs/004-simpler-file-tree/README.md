# Simpler file tree

I read the whole [Rust Book](https://doc.rust-lang.org/book/).

Okay, so back in my very first post, I started right out making this more complicated than it's ready to be. I tried to plan ahead for multiple code trees.

Maybe someday Rusty Sword will need that complexity, but not today. So I'm going to unwind that.

So I move `/code/rusty-sword-main/src` to `/src`. And `/code/rusty-sword-main/Cargo.toml` to `/Cargo.toml`.

I delete `/code`.

I run `cargo run`, now in the root directory. And it works.

```shell
rusty-sword apetro$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rusty-sword-main`
What is your character's name?

```

I'm sick with COVID, so that's all I'm going to accomplish today.

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../003-rust-book-2/README.md): Muddling through learning something from the guessing game example in the Rust book
+ [Next post](../005-core-loop/README.md): Core loop
