# Getting Rusty
## The Anatomy of a Rust Program
```rust
fn main() {
}
```
* The main function - always the first program that runs in every executable Rust program. 
* Rust requires curly brackets around all function bodies.
* println - used when calling a Rust macro 
* println - used when calling a function
* ; - end of an expression

### Compilation and Execution
```rust
rustc main.rs
```
* Before running a rust program, you must compile using a rust compiler - rustc. 
* Similar to gcc and clang in C and C++
* After compiling successfully, Rust outputs a binary executable.
* To run the compiled program: 
```rust
./main
```


## Cargo
Cargo - system and package manager.
* cargo new - to create a project
* cargo build - to build a project using 
* cargo run - build and run a project in one step using
* cargo check - build a project without producing a binary to check for errors 
  * cargo build --release  --- to build for release. It creates an executable in target/release instead of target/debug.
