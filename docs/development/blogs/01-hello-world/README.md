# Hello world

Okay, what does it take to get enough Rust going to say "Hello world"?

I took a quick peek at the [Rust language website](https://www.rust-lang.org/learn/get-started). Something something "rustup". Apparently I already have that installed from a prior go at messing with Rust.

```shell
$ which rustup
/Users/apetro/.cargo/bin/rustup
```

So I updated it to latest.

```shell
$ rustup update
info: syncing channel updates for 'stable-aarch64-apple-darwin'
info: latest update on 2024-06-13, rust version 1.79.0 (129f3b996 2024-06-10)
...

  stable-aarch64-apple-darwin updated - rustc 1.79.0 (129f3b996 2024-06-10) (from rustc 1.71.1 (eb26296b5 2023-08-03))

...
```

Cool?

Cargo is apparently also a thing

```shell
$ cargo --version
cargo 1.79.0 (ffa9cf99a 2024-06-03)
```

Okay. I vaguely think I'll end up with multiple packages, so I added a `/code` directory. I figure I'll end up with a main package for running the user interactive game and then a bunch of supporting packages. So within `/code` I ran

```shell
$ cargo new rusty-sword-main
    Creating binary (application) `rusty-sword-main` package
...
```

And sure enough, I can now get it to print hello world.

```shell
$ cd rusty-sword-main/
$ cargo run
   Compiling rusty-sword-main v0.1.0 (/Users/apetro/vcs/github/apetro/rusty-sword/code/rusty-sword-main)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.03s
     Running `target/debug/rusty-sword-main`
Hello, world!
```

Okay. Still no idea what I'm doing.

But. I built something and it ran. That's a start.
