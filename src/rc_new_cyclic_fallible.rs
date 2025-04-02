use std::mem::MaybeUninit;
use std::{rc::Rc, rc::Weak};

/// Helps building a synchronous Rc cyclic reference, from a closure that can fail.
///
/// Will be replaced by [UniqueRc](https://doc.rust-lang.org/stable/alloc/rc/struct.UniqueRc.html), once it gets stabilized.
pub fn rc_new_cyclic_fallible<T, E, F>(f: F) -> Result<Rc<T>, E>
where
    F: FnOnce(&Weak<T>) -> Result<T, E>,
{
    let mut result: Result<(), E> = Ok(());
    let maybe_uninit_rc = Rc::<MaybeUninit<T>>::new_cyclic(|weak_uninit| unsafe {
        // transmute guaranteed to be ok, because MaybeUniinit has repr(transparent),
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
    let raw = Rc::into_raw(maybe_uninit_rc);
    unsafe { Ok(Rc::from_raw(raw as *const T)) }
}
