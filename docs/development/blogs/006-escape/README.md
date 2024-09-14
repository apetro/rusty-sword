# Escape

Okay, let's give our hero the ability to return from the giant spiders nest to town.

```shell
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rusty-sword-main`
What is your character's name?
Hunter
Greetings, Hunter.
You are in a dark nest of giant spiders. It's a tough job clearing them out, but someone has got to do it.

Would you like to HUNT a spider or LEAVE the nest?
HUNT
You have encountered a giant spider.
You swing your rusty sword at the spider, but miss.
You have defeated the giant spider.
The spider evaporates in a cloud of foul-smelling rainbow smoke.
Incredibly, when the smoke clears it reveals an iron spirit coin where once there was a monster.
You take the coin. You now have 1 lesser spirit coins.


Would you like to HUNT a spider or LEAVE the nest?
LEAVE
Discretion is the better part of valor. You leave the nest.
The end.
```

Alright. It's still not fun, but there's starting to be some kind of game here of how many coins you can collect before fleeing. Not a great game.

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../005-core-loop/README.md): Core game loop
