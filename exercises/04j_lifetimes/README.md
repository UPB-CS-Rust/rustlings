# Lifetime Labels

Every reference is only valid for a part of the program: its *lifetime*.
Most of the time, the compiler figures lifetimes out on its own.

When a function returns a reference, the compiler needs to know which input reference it comes from.
You tell it with *lifetime annotations*. Think of them as **labels**:

```rust
fn later<'a>(a: &'a str, b: &'a str, c: &str) -> &'a str
```

- `<'a>` declares a label.
- References *entering* the function that are labeled `'a` (here `a` and `b`) are the ones
  the result may come from. `c` has no label: the result never comes from it.
- The reference *exiting* the function (the result) has the same label, so it may only be used while
  all inputs labeled `'a` are still alive.

Labels only describe how long references live. They never make a value live longer.

- Structs that hold references also need labels: `struct Excerpt<'a> { text: &'a str }`.
- `'static` is the label of references that are valid for the whole program, like string literals.
- With exactly one reference parameter, labels can be left out (*lifetime elision*).

## Further information

- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Lifetime elision (Reference)](https://doc.rust-lang.org/reference/lifetime-elision.html)
