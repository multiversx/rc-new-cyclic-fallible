# rc_new_cyclic_fallible function

At least as of Rust 1.85, there is no standard way of using Rc::new_cyclic with a fallible closure, that is a closure that can fail.

[UniqueRc](https://doc.rust-lang.org/stable/alloc/rc/struct.UniqueRc.html) is a promising development, but it is currently unstable.

This solution is as follows: the `rc_new_cyclic_fallible` function takes a closure that returns a `Result`, and returns a `Result` with the same error type itself.
