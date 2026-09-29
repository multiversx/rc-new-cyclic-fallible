# rc_new_cyclic_fallible function

`rc_new_cyclic_fallible` is a fallible variant of [`Rc::new_cyclic`]. It creates
an `Rc<T>` whose value can hold a `Weak<T>` reference to itself, while allowing
the initializer to return an error.

## MSRV

The minimum supported Rust version is 1.85.

## Usage

```rust
use std::rc::{Rc, Weak};

use rc_new_cyclic_fallible::rc_new_cyclic_fallible;

struct Node {
		parent: Weak<Node>,
}

let node: Rc<Node> = rc_new_cyclic_fallible(|weak| {
		Ok::<_, &'static str>(Node {
				parent: weak.clone(),
		})
})?;

assert!(node.parent.upgrade().is_some());
# Ok::<(), &'static str>(())
```

## Behavior

- The closure receives a `Weak<T>` that cannot be upgraded until initialization
	succeeds, matching `Rc::new_cyclic`.
- A successful closure result returns `Rc<T>`; weak references cloned by the
	closure then upgrade to it.
- An error is returned unchanged. Any weak references cloned by the closure
	remain valid handles but cannot be upgraded.

[`Rc::new_cyclic`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.new_cyclic
