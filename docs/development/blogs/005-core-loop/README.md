# Core loop

So, the problem with Rusty Sword so far is that it's not the least bit fun or interesting.

So what's the shortest path to some kind of game?

I'll want Rusty Sword to be more interesting than just whacking monsters with a rusty sword, but let's try that. How about a core loop of, you encounter a monster, you whack it with your rusty sword.

```shell
$ cargo run
   Compiling rusty-sword-main v0.1.0 (/Users/apetro/vcs/github/apetro/rusty-sword)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/rusty-sword-main`
What is your character's name?
Bob
Greetings, Bob.
You are in a dark, dusty dungeon.
You have encountered a giant spider.

You have defeated the giant spider.

You have encountered a giant spider.

You have defeated the giant spider.

You have encountered a giant spider.
```

Okay. Still boring, but it's a core loop.

How about you have hit points, and the spider bites you?

```shell
$ cargo run
   Compiling rusty-sword-main v0.1.0 (/Users/apetro/vcs/github/apetro/rusty-sword)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/rusty-sword-main`
What is your character's name?
Harriot
Greetings, Harriot.
You are in a dark, dusty dungeon.
You have encountered a giant spider.

The spider bites you. You have 9 health remaining.
You have defeated the giant spider.

You have encountered a giant spider.

The spider bites you. You have 8 health remaining.
You have defeated the giant spider.
```

Still, boring.

Okay, if you run out of health points your character dies.

```shell
You have encountered a giant spider.

The spider bites you. You have 0 health remaining.
You have succumbed to your wounds. The end.
```



Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../004-simpler-file-tree/README.md): Simplifying the file tree
