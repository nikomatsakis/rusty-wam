#[test]
fn var_terms() {
    let t1 = term!(?X);
    assert_eq!(&format!("{:?}", t1), "?X");
}

#[test]
fn complex_terms() {
    let t = term!(a(?X, b(?Y), c(?X, ?Y), d));
    assert_eq!(&format!("{:?}", t), "a(?X,b(?Y),c(?X,?Y),d)");
}
