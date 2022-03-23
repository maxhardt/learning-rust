# Learning Rust by solving Katas

This project documents my learning progress by solving Katas in Rust from codewars.com.

- What are Katas? Visit https://codewars.com
- What is Rust? Ehm... https://doc.rust-lang.org/stable/book/

## Structure

The project tries to follow best practices for structuring [Rust projects into `packages`, `crates` and `modules`](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html). The result is a `package` consisting of a single `library crate` called *katas*. This crate is divided into multiple `modules`, one module for each kata.

Tests are organized separately into [Unit Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests) for individual (helper) functions and [Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) addressing the final kata solution. Unit Tests are part of the `library crate` and defined next to the functions they are testing. Integration Tests are organized separately in `./tests`, each test importing the final kata solution from the katas `module` they are testing.

    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── lib.rs                  <-- single library crate
    │   ├── first_kata
    │   │   ├── README.md           <-- description of first kata
    │   │   └── mod.rs              <-- module and unit tests for first kata
    │   └── second_kata
    │       ├── README.md
    │       └── mod.rs
    └── tests
        ├── test_first_kata.rs      <-- integration tests for first kata
        └── test_second_kata.rs

## Setup and installation

Clone this repo and [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

## How to run?

Running both Unit and Integration tests:

```
$ cargo test

    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests (target/debug/deps/katas-d810538bc76d3a2d)

running 2 tests
test pagination::tests::test_max_digits_total ... ok
test pagination::tests::test_max_pages ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_pagination.rs (target/debug/deps/test_pagination-7e6066bd34d13830)

running 3 tests
test edges ... ok
test example ... ok
test random ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_prime.rs (target/debug/deps/test_prime-6502279b3cc8dbb4)

running 4 tests
test not_prime_tests ... ok
test basic_tests ... ok
test prime_tests ... ok
test random_tests ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s
```
