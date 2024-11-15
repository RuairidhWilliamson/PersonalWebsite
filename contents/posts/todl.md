# TODL

January 2023

Rust, CLI

---

![TODL Example Usage](/assets/images/todl.png)

## Background

When I am programming I like to put in quick comments to remind myself to come back to an area of code later. These normally take the form of a simple comment. For example

```rust
  // TODO: Come back and fix this later
```

This is very useful since it allows me to focus on the bigger picture and not to get hyper focused on a small detail because I can mark it as something that I should look at later.

The disadvantage is that my code always has some number of TODO comments littered about in it with varying importance. They can be as harmless as a suggestion. That a part of the code should be refactored or switched to a different api when it is released or that the code is unsound and there is an important bug that needs fixing.

In order to address these issues I wanted a command line tool that would quickly list all the todo comments in my code and help me find what I need to work on.

The obvious solution to this is to right a grep command that returns matches that look like comment tags. This is ok, but I wanted something a bit more user friendly, with more descriptive information and more accuracy.

There are already some tools that do this: [todo-ci](https://crates.io/crates/todo-ci) and [cargo-todo](https://crates.io/crates/cargo-todo) being two rust tools I found. But they did not fit my style of comment tags and did not give me any descriptive information about the tag from git.

I wanted to learn about how to make rust CLI tools, so I decided to make my own.

## Implementation

My implementation involved searching files recursively and attempting to identify what kind of source code they are based on their file extension. Then assuming they file extension understand the comment syntax for that particular language and search the file for any comment tags.

I divided this problem into C style comments which use `//` or `/* */` and rust style comments which include the C style comments with the addition of `//!`, `/*! */`, `///`, `/** */`, `/*** */` and `todo!()`.

If you are unfamiliar with rust's todo macro: `todo!("I will fix this later")`. It is a great placeholder that will evaluate as a `panic!()` with a nice panic message. It is particularly useful when trying to get some rust code to compile, and you need the compiler to assume you have written the rest of the code. Since the `todo!()` macro never returns (it panics) the compiler will not complain about things like missing returns. For example:
```rust
  fn foo() -> Bar {
    todo!("create bar")
  }
```

But since we are telling the compiler to shut up when we use `todo!()` we better remove them later.

Once we have the list of comment tags we fetch git information about when they were added and who added them. This is then output to the user in a colourful table. To aid in finding comment tags the user cares about, the kind of comment tags are categorized into fix, improvement, information and custom. TODL takes into account gitignore unless specified otherwise and can sort the output by when the comment tag was last changed.

Not only does TODL have a CLI interface it also has a documented API that allows other tools to build on it.

Using cargo you can install TODL using `cargo install todl`.

[![asciicast](https://asciinema.org/a/617576.svg)](https://asciinema.org/a/617576)

## Thoughts
I am very pleased with TODL and I use it regularly.

In terms of performance I did some benchmarking on large repos and found it doesn't take more than a few seconds. Most of the time is spent fetching the git information about the matches.

If I was to rewrite it, I would change the search method to not be so specific to the source code type. Instead match more based on heuristics of what it can determine comments probably look like in the supplied language. This would avoid the problem of TODL only working for a small set of languages.

[![](https://img.shields.io/crates/v/todl)](https://crates.io/crates/todl)
[![](https://img.shields.io/docsrs/todl)](https://docs.rs/todl)

[View Project](https://github.com/RuairidhWilliamson/todl)
