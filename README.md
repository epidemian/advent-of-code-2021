Trying to learn some Rust for [Advent of Code 2021](https://adventofcode.com/2021/)

## Notes/learnings

- Day 1: Simple challenge, but nice to get to know some iteration-related functions. The solution is probably not very idiomatic. I don't really understand how references or mutability work still.
- Day 2: Learned about include_str!, std::env::args(), enums, and matching on strings. Still, i'm using .unwrap() with no abandon, slapping it anywhere that requires the slightest error handling. Parsing logic could surely be improved a lot.
- Day 3: A bit-fiddling challenge! Not much learned honestly, besides that there are in-place array methods, like .retain().
- Day 4: Reached a very inelegant solution, one of those that makes you wonder if Rust is a good choice to express these problems, or if it's me that still majorly sucks at writing it. Learned about mutating vector data while iterating it, and the pains that might mean for satisfying the burrow checker. Also used tuples to express what should have probably been a data structure with named fields.
