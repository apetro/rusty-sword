# Modules

I can't handle the spaghetti code in one giant file anymore. It's only 285 lines, but still. We've got dice. We've got monsters. We've got dungeons. We've got the main program. I want those to be four separate files.

So I re-read the pages on modules in the Rust Book.

Then I wrap up the structs and functions about dungeons in

`pub mod dungeon {`

and then add a bunch of `use` to the new module and to the crate root to get everything to find everything else again, because it's intertwined spaghetti code.

`use crate::Monster;`

Maybe the monster stuff should be a child module of the dungeon stuff.

Anyway, it compiles with some kind of warnings about types not being the same amount of public as other things.

Ignoring those warnings for now, I factor the dungeon module out into its own file `dungeon.rs`. 76 lines. Bringing `main.rs` down to 221 lines.

Alright, let's go modular. A module for inventory.

Navigation:

+ [Documentation home](../../../README.md)
+ [Development documentation home](../../README.md)
+ [Blogs home](../README.md)
+ [Previous post](../008-refactor-combat/README.md): Factoring out functions
