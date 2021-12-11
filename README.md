Trying to learn some Rust for [Advent of Code 2021](https://adventofcode.com/2021/)

## Notes & learnings

### Day 1
Simple challenge, but nice to get to know some iteration-related functions. The solution is probably not very idiomatic. I don't really understand how references or mutability work yet.

### Day 2
Learned about include_str!, std::env::args(), enums, and matching on strings. Still, i'm using .unwrap() with no abandon, slapping it anywhere that requires the slightest error handling. Parsing logic could surely be improved a lot.

### Day 3
A bit-fiddling challenge! Not much learned honestly, besides that there are in-place array methods, like .retain().

### Day 4
Ended up with a very inelegant solution, one of those that makes you wonder if Rust is a good choice to express these problems, or if it's me that still majorly sucks at writing it — most likely the latter.

Learned about mutating vector data while iterating it, and the pains that might mean for satisfying the burrow checker. Also used tuples to express what should have probably been a data structure with named fields.

### Day 5
I like these map-based challenges :)

This one was an opportunity to learn about structs, even though i didn't end up using them. Using HashMap instead of a static array of 1000x1000 made the program slower, but i like the simpler and more generic code.

### Day 6
This one got me quite stuck. The naive solution of using a vector just like the challenge description example worked fine for 80 days, but got impossible for 256 because of the exponential growth. Not even counting on Rust's performance would've saved me, as the memory consumption would be too high for my computer: ~1.7TB if using a u8-array for the fish population.

After banging my head with maths for a while trying to find a closed-form solution using 2^\<something\> given each fish initial timer —for a fish simulation without the initial 2-day reproduction delay it was easy enough— i gave up and looked for a hint online. Found the idea of counting all the fish with the same internal timer the same way (seems so obvious in hindsight!) on [@riffraff's blog](https://riffraff.info/2021/12/advent-of-code-2021-day-6/). Thanks Gabriele!

### Day 7
An easy one. The only trick used here was to express the sum 1+2+...+N as (N+1)*N/2.
