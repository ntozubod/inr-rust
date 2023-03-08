# inr_rust
Slow migration of INR to rust

## 20230308 0.0.1

Upgrade the rust version from Hello World to a simple app that takes
arguments and reads a file and does simple processing.
It has a couple of tests.

This is a copy of the minigrep program from chapter 12 of The Rust Programming
Language (2nd Edition for Rust 2021) by Steve Klabnik and Carol Nichols and
contributions from the Rust Community (published 2023).
(see https://doc.rust-lang.org/book/)

Start from a solid base and evolve by gradually adding functionality.

## 20221029 2.1.1b

Move the mainline from inr_c/Lex.c into a separate source file.
The new main.c/main.rs file has minimal functionality.

## 20221028 2.1.1a

Added the subdirectory 'inr_c/rusty' containing script and results of
transpiling 'inr_c/src' into unsafe rust.
