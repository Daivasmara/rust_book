// panic macro is used on unrecoverable errors
// it will print the error, unwind and clean up the stack, then quit
// unwind is the default procedure which means rust walks back up the stack
// and cleans up the data from each function it encounters
// cons: unwinding is alot of work, the alternative is to directly abort
// you can specify this behaviour in cargo.toml
// [profile.release]
// panic = 'abort'

fn main() {
    //panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99]; // this index doesnt exist therefore panic
           // we can backtrace using: RUST_BACKTRACE=1 cargo run
}
