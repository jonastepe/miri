// FIXME: Disable validation until we figure out how to handle recursive statics.
// compile-flags: -Zmir-emit-validate=0

struct S(&'static S);
static S1: S = S(&S2);
static S2: S = S(&S1);

fn main() {
    let p: *const S = S2.0;
    let q: *const S = &S1;
    assert_eq!(p, q);
}
