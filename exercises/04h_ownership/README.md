# Ownership

Every value in Rust has exactly one *owner*. When the owner goes out of scope, the value is dropped
(its memory is freed). This is how Rust manages memory without a garbage collector.

- **Copy types** (integers, floats, `bool`, `char`, and arrays, tuples and structs made only of Copy types)
  are copied when assigned or passed to a function. Both variables stay usable.
- **Move types** (like `String`, or structs containing a `String`) own resources, for example memory on the heap.
  Assigning them or passing them to a function *moves* the ownership. The old variable can't be used anymore.
- A struct is a move type by default. It can be made Copy with `#[derive(Clone, Copy)]` if all its fields are Copy.
- `.clone()` creates an independent deep copy of a value.

## Further information

- [What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
