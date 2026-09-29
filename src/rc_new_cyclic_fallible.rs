use std::mem::MaybeUninit;
use std::{rc::Rc, rc::Weak};

/// Creates a cyclic [`Rc`] from a closure that may fail.
///
/// This has the same weak-reference behavior as [`Rc::new_cyclic`]: the supplied
/// [`Weak`] cannot be upgraded while `f` runs. It can be cloned and stored in the
/// returned value; once this function returns `Ok`, it upgrades to that value.
/// If `f` returns `Err`, the allocation is discarded and any cloned weak references
/// can no longer be upgraded.
///
/// # Examples
///
/// ```
/// use std::rc::Weak;
///
/// use rc_new_cyclic_fallible::rc_new_cyclic_fallible;
///
/// struct Node {
///     self_reference: Weak<Node>,
/// }
///
/// let node = rc_new_cyclic_fallible(|weak| {
///     Ok::<_, ()>(Node {
///         self_reference: weak.clone(),
///     })
/// })?;
///
/// assert!(node.self_reference.upgrade().is_some());
/// # Ok::<(), ()>(())
/// ```
///
/// [`Rc::new_cyclic`]: std::rc::Rc::new_cyclic
pub fn rc_new_cyclic_fallible<T, E, F>(f: F) -> Result<Rc<T>, E>
where
    F: FnOnce(&Weak<T>) -> Result<T, E>,
{
    let mut result: Result<(), E> = Ok(());
    let maybe_uninit_rc = Rc::<MaybeUninit<T>>::new_cyclic(|weak_uninit| unsafe {
        // transmute guaranteed to be ok, because MaybeUninit has repr(transparent),
        // additionally, the reference is not going to be used in case of error
        let weak: &Weak<T> = core::mem::transmute(weak_uninit);

        match f(weak) {
            Ok(t) => MaybeUninit::<T>::new(t),
            Err(err) => {
                result = Err(err);
                MaybeUninit::<T>::uninit()
            }
        }
    });
    result?;

    // transmute guaranteed to be ok, because MaybeUninit has repr(transparent)
    let converted: Rc<T> = unsafe { core::mem::transmute(maybe_uninit_rc) };

    Ok(converted)
}
