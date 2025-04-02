use rc_new_cyclic_fallible::rc_new_cyclic_fallible;

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
