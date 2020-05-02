## 01 - Getting Started
Rust is an _ahead-of-time_ compiled language. Meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. The trade-off in this languge design is you need to run 2 commands, one to compile and another to run the program.

```sh
rusctc main.rs # compile and produce 'main' executable
./main # run the compiled program
```

### Cargo
Cargo is Rust's build system and package manager. It handles tasks such as building the code, downloading the library, and building those libraries (libraries that the code needs called _dependencies_)

```sh
cargo new hello_world # generate new project
```
> add `--vcs=git` to forcefully generate git files within existing git repository

```sh
cargo build # compile for development
cargo run # compile and run
cargo check # quick check to make sure it compiles without producing an executable
```
> compiled development executable location: `./target/debug/hello_world`

```sh
cargo build --release # compile for release, with optimizations to make program run faster
                      # the drawback is that it takes longer to compile
```
> compiled release executable location: `./target/release/hello_world`
