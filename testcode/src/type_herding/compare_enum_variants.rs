enum Test {
    A(usize),
    B(usize),
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn main() {
    // as the variants are the same, these are "variant_eq"
    let a = Test::A(1);
    let b = Test::A(2);

    assert!(variant_eq(&a, &b));

    // here, the variants aren't the same, but the inner values are
    // this is not "variant_eq"
    let c = Test::A(1);
    let d = Test::B(1);

    assert!(! variant_eq(&c, &d));
}
