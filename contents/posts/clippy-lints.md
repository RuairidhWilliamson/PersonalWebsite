# Useful Clippy Lints

November 2024

Rust

---

![](/assets/images/ferris.png)

## Introduction
Rust has many tools that make up its ecosystem.
The main components are [rustc](https://github.com/rust-lang/rust) and [cargo](https://github.com/rust-lang/cargo) but [rustfmt](https://github.com/rust-lang/rustfmt), [clippy](https://github.com/rust-lang/rust-clippy) and [rust-analyzer](https://github.com/rust-lang/rust-analyzer) are often used too.
Clippy, not to be confused with the Windows help mascot, is a linter.
Rustc also performs linting but clippy's lints are more extensive and more opinionated.
Clippy is distributed with [rustup](https://rustup.rs) along with the other rust components.
It also follows the same release cycle where a new stable version is released every 6 weeks.

## Default Lints
Clippy has a lot of lints, [clippy-lints](https://rust-lang.github.io/rust-clippy/stable/index.html).
736 at the time of writing.
445 of these are set to warn/deny by default.
Leaving 291 which are allow by default.
This is still a lot of lints that aren't being used.
The question is which of them should a rust developer enable.
I have made a list of what I have found to be useful and not overly strict lints that I use in my rust projects.

## Lints
Clippy divides its lints into groups.
Lint groups allow the user to enable a lot of lints at once.
In the lint list here I have enabled `pedantic` and `nursery` lint groups.

`restriction` is for lints that impose restrictions and should not all be enabled.
Some of them are only applicable in specific contexts and some are contradictory.
I have chosen a few that I find useful.
I have also provided 3 commented out that if your project suits would be good to enable.
Specifically `unwrap_used` can be very helpful to ensure your error handling is sound.
If you really want to panic you can use `.expect()`.

The lint group `nursery` is for lints that are unreliable or have known issues.
It is a bad idea to enable this whole group.
It would be better to pick out specific lints that are useful and just enable them.
However enabling this entire group hasn't caused me too many problems and there are only a few lints that I have had issues with.

The lint group `pedantic` is for lints that are very opinionated and is used against clippy's own source code.
Enabling this adds some lints that I find are too restrictive.
This is why some of these lints are set to allow.

The rust lints and cargo lints are self explanatory.
`cargo_common_metadata` is useful if you are publishing a crate on [crates.io](https://crates.io/).

I recommend adding and removing lints to your taste but this should be a useful starting place or a way to discover lints.

Here is the list. You can put this in your `Cargo.toml` and it will just work.
```TOML
[lints.rust]
unsafe_code = "warn"
unused_crate_dependencies = "warn"

[lints.clippy]
# Cargo
# cargo_common_metadata = "warn"
wildcard_dependencies = "warn"

# Restriction
allow_attributes = "warn"
clone_on_ref_ptr = "warn"
create_dir = "warn"
dbg_macro = "warn"
exit = "warn"
string_to_string = "warn"
undocumented_unsafe_blocks = "warn"
unused_result_ok = "warn"
unused_trait_names = "warn"
# unwrap_used = "warn"
# print_stderr = "warn"
# print_stdout = "warn"

# Nursery
nursery = { level = "warn", priority = -1 }
missing_const_for_fn = "allow"
significant_drop_tightening = "allow"
suboptimal_flops = "allow"
option_if_let_else = "allow"

# Pedantic
pedantic = { level = "warn", priority = -1 }
cast_precision_loss = "allow"
default_trait_access = "allow"
missing_errors_doc = "allow"
module_name_repetitions = "allow"
must_use_candidate = "allow"
```

Also a good idea to add a `clippy.toml` file with this.
```TOML
allow-print-in-tests = true
allow-unwrap-in-tests = true
```
