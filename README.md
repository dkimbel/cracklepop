# CracklePop

## Background

For a 'normal' implementation of FizzBuzz in Rust, see [the 'Rust by Example' book](https://web.mit.edu/rust-lang_v1.26.0/arch/amd64_ubuntu1404/share/doc/rust/html/rust-by-example/flow_control/while.html).

What I've written here is a more elaborate version intended to be more extensible. What if tomorrow, we weren't going
to only need to support two "noises" like Crackle and Pop, but rather four? Or even ten? Or, what if our users wanted to
define their own new noises anytime, without our code changing at all?

My implementation solves this extensibility problem using a struct -- basically a class. Callers can create their own
instances of the `Noise` struct, providing a name like `"Crackle"` and a function that determines whether the noise
"matches" any given integer.

To see this in action, have a look at my 'snap crackle pop boom' integration test (`tests/integration_tests.rs`), which
adds two additional "noises" on top of Crackle and Pop: Snap (for numbers divisible by four) and Boom (for numbers
divisible by twelve).

## Exploring the code

When this CracklePop solution is executed, the code that actually runs is from `src/main.rs`. That's the 'toplevel' file
that iterates through all numbers between 1 and 100, looks up the matching "noises" like Crackle or Pop (if any), and
then prints the given number's result (its noises, or -- if there aren't any -- the number itself).

However, most of the actual code and datatypes are defined in `src/lib.rs`, the conventional file for Rust 'library'
code. This is where a 'Noise' is actually defined as a thing that can match a number and be converted to text for
printing.

## Running the code locally

Having [installed Rust](https://www.rust-lang.org/tools/install), cloned this repository, and changed into its
directory, you can run my CracklePop implementation with one command:
```
cargo run
```

## Running the tests locally

I've written tests to assert that all 100 lines of my CracklePop outputs are correct, and that a more complex exercise
that adds two additional noises would also be solved correctly. To run these tests locally, just use the normal cargo
command:
```
cargo test
```