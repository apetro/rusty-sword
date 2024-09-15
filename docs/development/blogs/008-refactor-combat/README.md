# Refactoring combat

So, combat should be a function.

And exploring a dungeon should be a function.

Which allows a couple kinds of dungeons.

I don't understand the borrow checker and borrowing and when to use an `&` and not so I'm just flailing away at adding `&` until it compiles. Which is not great. I'll need to re-read that chapter about the borrow checker.

Also I'm probably using String where I would be better off using &str.

But. It compiles.

```shell
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rusty-sword`
What is your character's name?
Bob
Greetings, Bob.

Would you like to clear out the SPIDERS nest, confront the BRIDGE troll, view your INVENTORY, or RETIRE from adventuring?
SPIDERS
A dark nest of scary spiders. It's a tough job clearing them out, but someone has got to do it.

Location: nest of spiders
Would you like to HUNT a monster, LEAVE nest of spiders, or view your INVENTORY?
HUNT
You have encountered a extra large spider.
The extra large spider bites you. You have 9 health remaining.
You have defeated the extra large spider.
The extra large spider evaporates in a cloud of foul-smelling rainbow smoke.
When the smoke dissipates, a lesser spirit coin remains, which you grab.


Location: nest of spiders
Would you like to HUNT a monster, LEAVE nest of spiders, or view your INVENTORY?
LEAVE
Discretion is the better part of valor. You leave the nest.

Would you like to clear out the SPIDERS nest, confront the BRIDGE troll, view your INVENTORY, or RETIRE from adventuring?
BRIDGE
A bridge under which lives a grumpy troll

Location: troll bridge
Would you like to HUNT a monster, LEAVE troll bridge, or view your INVENTORY?
HUNT
You have encountered a Bridge Troll.
The Bridge Troll whomps you. You have 8 health remaining.
You swing your rusty sword at the Bridge Troll, but miss.
The Bridge Troll whomps you. You have 7 health remaining.
You swing your rusty sword at the Bridge Troll, but miss.
The Bridge Troll whomps you. You have 6 health remaining.
You swing your rusty sword at the Bridge Troll, but miss.
The Bridge Troll whomps you. You have 5 health remaining.
You have defeated the Bridge Troll.
The Bridge Troll evaporates in a cloud of foul-smelling rainbow smoke.
When the smoke dissipates, a lesser spirit coin remains, which you grab.


Location: troll bridge
Would you like to HUNT a monster, LEAVE troll bridge, or view your INVENTORY?
LEAVE
Discretion is the better part of valor. You leave the nest.

Would you like to clear out the SPIDERS nest, confront the BRIDGE troll, view your INVENTORY, or RETIRE from adventuring?
INVENTORY
You are wielding a rusty sword in your dominant hand.
You have 2 lesser spirit coins.

Would you like to clear out the SPIDERS nest, confront the BRIDGE troll, view your INVENTORY, or RETIRE from adventuring?
RETIRE
Bob ended the game with 2 lesser spirit coins.
The end.
$
```

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../006-escape/README.md): Escaping the spider nest
