Trying to learn some Rust for [Advent of Code 2021](https://adventofcode.com/2021/)

## Notes & learnings

### Day 1: Sonar Sweep
Simple challenge, but nice to get to know some iteration-related functions. The solution is probably not very idiomatic. I don't really understand how references or mutability work yet.

### Day 2: Dive!
Learned about include_str!, std::env::args(), enums, and matching on strings. Still, i'm using .unwrap() with no abandon, slapping it anywhere that requires the slightest error handling. Parsing logic could surely be improved a lot.

### Day 3: Binary Diagnostic
A bit-fiddling challenge! Not much learned honestly, besides that there are in-place array methods, like .retain().

### Day 4: Giant Squid
Ended up with a very inelegant solution, one of those that makes you wonder if Rust is a good choice to express these problems, or if it's me that still majorly sucks at writing it — most likely the latter.

Learned about mutating vector data while iterating it, and the pains that might mean for satisfying the borrow checker. Also used tuples to express what should have probably been a data structure with named fields.

### Day 5: Hydrothermal Venture
I like these map-based challenges :)

This one was an opportunity to learn about structs, even though i didn't end up using them. Using HashMap instead of a static array of 1000x1000 made the program slower, but i like the simpler and more generic code.

### Day 6: Lanternfish
This one got me quite stuck. The naive solution of using a vector just like the challenge description example worked fine for 80 days, but got impossible for 256 because of the exponential growth. Not even counting on Rust's performance would've saved me, as the memory consumption would be too high for my computer: ~1.7TB if using a u8-array for the fish population.

After banging my head with maths for a while trying to find a closed-form solution using 2^\<something\> given each fish initial timer —for a fish simulation without the initial 2-day reproduction delay it was easy enough— i gave up and looked for a hint online. Found the idea of counting all the fish with the same internal timer the same way (seems so obvious in hindsight!) on [@riffraff's blog](https://riffraff.info/2021/12/advent-of-code-2021-day-6/). Thanks Gabriele!

### Day 7: The Treachery of Whales
An easy one. The only trick used here was to express the sum 1+2+...+N as (N+1)*N/2.

### Day 8: Seven Segment Search
A challenging but very interesting one. I'm happy with the solution, although it could probably benefit from modeling the signal patterns as sets and then doing set intersection with the output patterns.

### Day 9: Smoke Basin
A 2D tiles one! I like these! The naive solution, with a vec-of-vecs, was easy enough to implement and worked fine, but it was nice to refactor the modeling of the sea floor as a `Map` of (x, y) tuples. This made accessing the map and checking boundaries much simpler :)

### Day 10: Syntax Scoring
A parsing bracket-balancing one. Learned about iter's `filter_map()` and `partition()` to try to separate the corrupted lines from the incomplete ones, but ended up using a good ol' for loop and pushing into separate vectors, as that seemed more readable to me.

I did however enjoy refactoring the two separate functions `get_corrupted_char(line)` and `get_unclosed_brackets(line)`, which had a lot of repeated parsing logic, into a single function that returns a `Result<Incomplete, Corrupted>`.

### Day 11: Dumbo Octopus
Yet another 2D puzzle, yay! The emergent behavior of the octopi synchronizing their flashes was a really nice surprise ^_^

This time i considered going directly for the `Map<(i32, i32), Value>` representation of the grid, but i'm glad i resisted that urge, since the static-size array solution ended up working very nicely. The initial code was super ugly in terms of nested loops and conditionals, but after refactoring the "unprocessed flashes" into a stack (a vector) instead of checking the whole map for values over 9, the messiness of the code was reduced substantially.

### Day 12: Passage Pathing
A path-finding challenge. It's nice that graphs can usually be modelled by maps-of-vectors. Recursion as usual worked well for this kind of graph-traversal problem. Rust's ergonomic closures made it pretty easy to parameterize the logic of when the submarine could visit a cave or not, to implement both parts with the same basic algorithm.

### Day 13: Transparent Origami
A very fun geometric puzzle. Got a dumb off-by-one error when printing the paper at the last step and was trying to debug folding logic that was never broken to being with 😖

### Day 14: Extended Polymerization
The naive solution of generating the polymer worked well for computing the 10 steps of part 1, but completely failed for the 40 steps of part 2, as that would require TBs of storage. I think that the idea of keeping count of all element pairs and thinking only in terms of what new pairs those generate was inspired by Day 6: Lanternfish, which also featured exponential growth :)

### Day 15: Chiton
Oof, this one was challenging. I initially considered trying to find a path using only right and down moves, as the example case only showcased moves in those directions. This greatly simplified the "path finding" algorithm, and luckily it worked for part 1. But not for part 2. Oh no. Path 2 required a more general solution, and damn it if i were to remember how Dijkstra's algorithm worked. I'm not even sure i ever knew it at any point in my life.

So i just hacked my way through trying to patch the first solution with some "reassessment" of path costs. It worked in the end, but i doubt this algorithm is calculating path cost for general cases, and it most likely is terribly inefficient.

Update: i ended up implementing Dijkstra's algorithm. Not generalized; a very ad-hoc for this specific problem. It's a bit more code than the previous solution, but i feel it's quite more understandable. And it runs faster too :)

### Day 16: Packet Decoder
Not a difficult challenge, but quite a bit of code needed. I used some OO, and the result ended up looking pretty legit :)

### Day 17: Trick Shot
Not much learned really, besides the fact that Rust's type inference is lovely. Kind of a brute force solution for a relatively simple problem. Not proud of not having found a more clever approach, but it totally works.

### Day 18: Snailfish
Wow, so far this has been the hairiest to implement in Rust *by far*. The biggest challenge was to do proper memory management of a tree-like structure that needs to be manipulated. After a lot of fighting with the borrow checker, the [solution i reached](src/day18_oo.rs) was quite OO-ish and looked fairly alright from the point of view of data modelling and delegation, but i definitely didn't like the mess of `Rc`s it was, and how complicated exploding and splitting snailfish numbers became.

So i ended up [trying again](src/day18.rs), this time representing each snailfish number as simply a list of tokens, which made implementing exploding much easier, as the logic of "adding the pair's left value to the first regular number to the left of the exploding pair (if any)" is quite trivial when the structure is linear instead of nested. This modelling also avoided any memory management shenanigans and borrow-checking-related pains.

The only downside of the second approach was that calculating the snailfish number's magnitude required a bit more bookkeeping and somewhat involved logic, while doing so in the OO tree-like approach was as straightforward as it could be: just classic object delegation.

### Day 20: Trench Map
I skipped day 19 to complete this simpler cellular automata sort of puzzle. The infiniteness of the grid was simulated by simply pre-extending the image by the amount of necessary steps. The cellular automata rules given by the input had a "trick" of making all the dark pixels on the infinitely-extended image turn to light and then back to dark on the next step.


### Day 21: Dirac Dice
Part 1 was quite easy with a naive solution. Part 2 was one of *those* maths-heavy ones. Managed to get a recursive solution that doesn't take a humongous amount of time to compute. Apparently there are much faster solutions, but at least i can understand mine :)


### Day 22: Reactor Reboot
Part 1 was solvable by "brute force" using a naive implementation of a 3D grid representing all reactor cubes. Part 2 required some cleverness on the cuboids' representation, as representing and processing the entire reactor grid became unfeasible.

It was a rather nice exercise in decomposition and data modelling. The most enjoyable part was changing the cuboid representation from two (x, y, z) tuples to three (start, end) ranges, which allowed a much more natural delegation and decomposition of both algorithms and parsing :)

### Day 23: Amphipod
I had no idea where to start implementing this one, but the moves of the amphipods seemed easy to understand, so after some trial and error i managed to solve part 1 [by hand](src/day23.txt).

After that, and seeing that part 2 was about, i now have an idea of what to do for solving this with code: model the state of the amphipod burrow and implement a shortest path algorithm to find the minimal cost to move between the initial state and the goal state where all amphipods are in their destination rooms.

Update: i implemented the above strategy. It took a while to get all conditions right, and i used an external pathfinding library. I might get rif of the latter if i feel inspired to code a generic Dijkstra.

Update 2: implementing a generic Dijkstra's algorithm was interesting. I learned about type parameter trait constraints, which is a bit abstract, but makes sense after some getting used to it. And also learned about Rust rules for importing modules, which are not super intuitive IMO.

The generic algorithm ended up being slower than the one in the [`pathfinding`](https://docs.rs/pathfinding/latest/pathfinding/) crate, which is to be expected. And it's also slower then the ad-hoc algorithm i did for day 15, which is also to be expected, since that one used a 2D vector for storing distances, while the generic algorithm must use a more generic data structure — a HashMap in this case. But even though it's slower, i decided to use it for both days' solutions, since it was a nice learning experience, and that also leaves the project dependency-free, and all code is in one place :)

### Day 25: Sea Cucumber
Nice to have a simple and straightforward ending puzzle. Went all-in with a mutable 2D vec :)
