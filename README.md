Sort macro input tokens

Example
-------------------------------------------------------------------------------
After sorting, it is easy to input keys in an unordered manner
```rust
macro_rules! foo {
    (@branch (a $a:literal) (b $b:literal)) => {
        concat!(stringify!($a), stringify!($b))
    };
    ($($name:ident : $val:literal)+) => {
        sort_macro::sort_in!({@branch} foo!($( ($name $val) )+))
    };
}
fn main() {
    assert_eq!(foo!(b:3 a:2), "23");
    assert_eq!(foo!(a:2 b:3), "23");
}
```

Grammar
-------------------------------------------------------------------------------
Sort in last group, sort key is `TokenTree::to_string`

```abnf
input  = [prefix] *tt group
prefix = "{" *tt "}"
```
