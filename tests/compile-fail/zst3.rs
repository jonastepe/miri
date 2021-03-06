// FIXME: Due to https://github.com/rust-lang/rust/issues/43457 we have to disable validation
// compile-flags: -Zmir-emit-validate=0

// error-pattern: the evaluated program panicked

#[derive(Debug)]
struct A;

fn main() {
    // can't use assert_eq, b/c that will try to print the pointer addresses with full MIR enabled
    assert!(&A as *const A == &A as *const A);
}
