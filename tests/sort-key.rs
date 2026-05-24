macro_rules! val {
    ($(($name:ident : $val:literal))+) => {
        concat!($(stringify!($name), stringify!($val),)+)
    };
}

#[test]
fn basic() {
    assert_eq!("a3a4b2b1", sort_macro::sort_in!([.0] val!((a:3) (b:2) (b:1) (a:4))));
}
