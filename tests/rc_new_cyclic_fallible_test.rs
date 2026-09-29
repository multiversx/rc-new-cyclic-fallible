use rc_new_cyclic_fallible::rc_new_cyclic_fallible;

use std::cell::Cell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct StructA {
    name: &'static str,
    b: Rc<StructB>,
}

impl PartialEq for StructA {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
struct StructB {
    a: Weak<StructA>,
}

fn new_struct_a(weak: Weak<StructA>) -> StructA {
    StructA {
        name: "StructA",
        b: Rc::new(StructB { a: weak }),
    }
}

#[test]
fn test_new_cyclic_fallible_ok() {
    let result: Result<Rc<StructA>, &str> =
        rc_new_cyclic_fallible(|weak| Ok(new_struct_a(weak.clone())));
    let struct_a = result.unwrap();
    assert_eq!(struct_a.name, "StructA");
    assert_eq!(struct_a.b.a.upgrade().unwrap().name, "StructA");
}

#[test]
fn test_new_cyclic_fallible_err() {
    let result = rc_new_cyclic_fallible(|weak| {
        let _ = new_struct_a(weak.clone());
        Err("error")
    });
    assert_eq!(result, Err("error"))
}

#[test]
fn weak_cannot_be_upgraded_during_initialization() {
    let result = rc_new_cyclic_fallible(|weak| {
        assert!(weak.upgrade().is_none());
        Ok::<_, ()>(42)
    });

    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn weak_escaped_through_error_cannot_be_upgraded() {
    let result: Result<Rc<()>, Weak<()>> = rc_new_cyclic_fallible(|weak| Err(weak.clone()));
    let weak = result.unwrap_err();

    assert!(weak.upgrade().is_none());
}

struct DropCounter(Rc<Cell<usize>>);

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn initialized_value_is_dropped_once() {
    let drops = Rc::new(Cell::new(0));
    let value = rc_new_cyclic_fallible(|_| Ok::<_, ()>(DropCounter(drops.clone()))).unwrap();

    assert_eq!(drops.get(), 0);
    drop(value);
    assert_eq!(drops.get(), 1);
}
