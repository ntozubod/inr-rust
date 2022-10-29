# inr-rust
Slow migration of INR to rust

## 20221029 2.1.1b

Move the mainline from inr_c/Lex.c into a separate source file.
The new main.c/main.rs file has minimal functionality.

## 20221028 2.1.1a

Added the subdirectory 'inr_c/rusty' containing script and results of
transpiling 'inr_c/src' into unsafe rust.
