# Borrowing

Moving values around all the time is inconvenient. Instead, you can *borrow* a value with a reference
and leave the ownership where it is.

- `&value` borrows for reading, `&mut value` borrows for changing.
- At any time, a value can have *either* one mutable borrow *or* any number of shared borrows.
- A borrowed value can't be moved while the borrow is still used.
- Different fields of a struct can be borrowed independently.
- Functions that only read text usually take `&str`: it accepts both `&String` and string literals.

## Further information

- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [String Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices)
