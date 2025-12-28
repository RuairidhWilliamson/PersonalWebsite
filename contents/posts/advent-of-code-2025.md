# Advent of Code 2025

December 2025

Rust

---

![Advent Of Code](/assets/images/advent_of_code_2025.png)

[Advent Of Code](https://adventofcode.com/) is an annual set of programming problems that are released each day in December. This year there were 12 problems. I implemented my solutions in Rust.

[My Solutions Git Repository](https://github.com/RuairidhWilliamson/aoc/tree/2025)

Below you can see the execution times for each of my solutions:

|        | Part 1         | Part 2         |
| ------ | -------------- | -------------- |
| Day 1  | `85.602µs`     | `87.772µs`     |
| Day 2  | `4.57µs`       | `19.26µs`      |
| Day 3  | `36.331µs`     | `67.802µs`     |
| Day 4  | `88.042µs`     | `1.776048ms`   |
| Day 5  | `85.961µs`     | `37.581µs`     |
| Day 6  | `28.38µs`      | `28.67µs`      |
| Day 7  | `40.69µs`      | `20.32µs`      |
| Day 8  | `60.81036ms`   | `62.474765ms`  |
| Day 9  | `179.964µs`    | `4.078527095s` |
| Day 10 | `3.73721ms`    | `15.32971328s` |
| Day 11 | `143.543µs`    | `625.043µs`    |
| Day 12 | `1.773988511s` |                |

I tried to optimise each solution to run as quickly as possible. Some of these were easier to optimise than others. I was able to most of the first few days running in less than `1ms`. The later days proved more challenging. Especially Day 10 which I had the most trouble with.

## Useful Utilities

I built a few reusable utilities this year.

### Grid

Advent of Code always has some problems that need a 2D grid to solve. I implemented this as a generic wrapper around a `Vec` with a width and height. Exposing methods to get and set cells. Adding methods to iterate, display the entire grid and swap rows was useful. For extra performance, adding unsafe get_unchecked and set_unchecked was useful too.

[Grid Implementation](https://github.com/RuairidhWilliamson/aoc/blob/2025/src/grid.rs)

### ASCII Grid

ASCII Grid is a way to take an ASCII string and use it as a grid. It assumes the rows are separated by `\n`. It then gives access to each ASCII character as a cell by its coordinates. It is nice because it means the string allocation can often be borrowed. This is not as feature full as the Grid but it is quicker and easier to use when the input is a grid of ASCII characters.

[ASCII Grid Implementation](https://github.com/RuairidhWilliamson/aoc/blob/2025/src/ascii_grid.rs)

### Integer Iterator

Integer iterator iterates through the entire space of a vector of non-negative integers `[usize; N]`. For example for `N = 3`.

```
[0, 0, 0]
[1, 0, 0]
[0, 1, 0]
[0, 0, 1]
[2, 0, 0]
[1, 1, 0]
...
```

It guarantees that the sum of its output is monotonic. Meaning the sum stays the same or increases at each iterations step. It will never decrease.

This is useful for searching over multi dimensional integer spaces. The other benefit is that the iterator is memory efficient. It takes up a fixed space and `memcpy` can be avoided using the exposed methods instead of the iterator.

I wrote an implementation backed by `Vec` on the heap and `[usize; N]` on the stack.

[Integer Iterator Implementation](https://github.com/RuairidhWilliamson/aoc/blob/2025/src/integer_iter.rs)

## Optimisation Strategies

These are some general optimisation strategies in no particular order without going into specific details of the problems.

### Release Mode

The first and easiest optimisation strategy is to compile in the release profile. This is the equivalent of `-O2` or `-O3` in C/C++ parlance. Just enabling release mode gives a huge speed up as the compiler makes a lot of good optimisations.

### Cargo Profile Tweaks

In a similar vein, tweaking the release profile can yield some small performance gains. Adding this to `Cargo.toml` might improve runtime performance slightly at the cost of compilation speed.

```toml
[profile.release]
codegen-units = 1
lto = true
panic = "abort"
strip = true
```

### Target Native CPU

Another easy optimisation is to enable native CPU target. This lets `rustc` know that we are only going to run this binary on this CPU so it can make use of the full set of instructions that my CPU supports. By default `rustc` will build for a more compatible set of instructions to allow the built binary to be shared.

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

### Cargo Flamegraph

[Cargo Flamegraph](https://github.com/flamegraph-rs/flamegraph) is an excellent tool for determining where your code is spending time executing. It generates an SVG showing the stacktrace where the width represents how often that call was in the stacktrace. It can be misleading because it does not show call frequency only where time was spent. A function being called 1000s of times looks the same as a function being called once and spending a long time in that single call.

When using Cargo Flamegraph it would often reveal that the most time consuming code is `HashMap` look ups or growing vectors. Refactoring code with this knowledge often lead to large speedups.

### Removing Bound Checks

Rust cares a lot about safety so has bound checks on most things. This is great and doesn't incur much performance overhead. However when optimising a very tight loop removing bounds checks that are guaranteed to succeed can give performance wins. This rightly requires unsafe code. It was particularly valuable to add methods to do unsafe operations in my Grid struct. Since often it was easy to prove the index was within the grid.

### Algorithm Changes

Advent of Code is mostly about what algorithm you are going to employ to solve each problem. So reconsidering a generic depth first search and using something more specific to the problem is often a good idea.

### Multithreading

[Rayon](https://github.com/rayon-rs/rayon) is an excellent library for running iterators across multiple threads. Because Rust has strong guarantees around sharing state rayon can guarantee that the program is correct to multi thread. Depending on the iterator and algorithm this could speed up from 2 to 8 times.

### Reduce Memory Usage

Reducing the size of data can improve performance too. For example, changing a `u64` to a `u8` if it only needs 256 values can add up if that value is repeated a lot. This makes it more likely data will be stored in the same cache line and helpful when having trouble with using up all the system memory.
