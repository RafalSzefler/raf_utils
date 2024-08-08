use core::{
    hint::spin_loop,
    mem::forget};

use super::{internal_array::InternalArray, ArrayId, StrongArray, FinalWeakArray, WeakUpgradeError};


/// Represents a weak reference to the underlying array. Unlike strong reference
/// it doesn't allow using the underlying array. It does allow upgrade to
/// strong reference, if there is at least one other strong reference alive.
/// 
/// **Note**: All [`StrongArray`] pointing to the same array count as a single
/// [`WeakArray`]. In other words the number of weak references can never go
/// to 0 before the number of strong references does.
#[derive(Debug)]
pub struct WeakArray<T>
    where T: Sized
{
    internal: InternalArray<T>
}

impl<T> WeakArray<T> {
    pub(super) fn new_raw(internal: InternalArray<T>) -> Self {
        Self { internal }
    }

    /// Upgrades to [`StrongArray`]. For that call to succeed there must be
    /// at least other strong reference alive.
    /// 
    /// # Errors
    /// * [`WeakUpgradeError::NoStrongReference`] if there are no more strong
    ///   references alive.
    pub fn upgrade(&self) -> Result<StrongArray<T>, WeakUpgradeError> {
        let strong = self.internal.strong_mut();
        let mut old_value = strong.atomic_load();
        loop {
            if old_value == 0 {
                return Err(WeakUpgradeError::NoStrongReference);
            }

            let cas = strong.compare_exchange_weak(
                old_value,
                old_value+1);

            match cas {
                Ok(_) => {
                    return Ok(StrongArray::new_raw(unsafe { self.internal.make_alias() }));
                },
                Err(current_value) => {
                    old_value = current_value;
                }
            }

            spin_loop();
        }
    }

    /// Releases current [`WeakArray`]. If it was the last [`WeakArray`],
    /// it returns the final [`FinalWeakArray`] and [`None`] otherwise.
    #[must_use]
    #[inline(always)]
    pub fn release(mut self) -> Option<FinalWeakArray<T>> {
        let result = self.release_mut();
        forget(self);
        result
    }

    /// Returns a unique identifier for this array. It is shared between
    /// strong/weak references.
    #[inline(always)]
    pub fn id(&self) -> ArrayId { self.internal.id() }

    /// Returns the number of alive strong references.
    #[inline(always)]
    pub fn strong_count(&self) -> u32 {
        self.internal.strong_mut().atomic_load()
    }

    /// Returns the number of alive weak references.
    #[inline(always)]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak_mut().atomic_load()
    }

    #[inline(always)]
    pub fn additional_data(&self) -> u32 {
        self.internal.additional_data()
    }

    #[inline(always)]
    fn release_mut(&mut self) -> Option<FinalWeakArray<T>> {
        let weak = self.internal.weak_mut();
        if weak.atomic_sub(1) == 1 {
            Some(FinalWeakArray::new_raw(unsafe { self.internal.make_alias() }))
        } else {
            None
        }
    }
}

impl<T> Drop for WeakArray<T> {
    fn drop(&mut self) {
        let _ = self.release_mut();
    }
}

impl<T> Clone for WeakArray<T> {
    /// Clones current [`WeakArray`] by bumping internal weak ref counter.
    fn clone(&self) -> Self {
        let _ = self.internal.weak_mut().atomic_add(1);
        Self {
            internal: unsafe { self.internal.make_alias() }
        }
    }
}

unsafe impl<T> Sync for WeakArray<T> where T: Sync + Send { }
unsafe impl<T> Send for WeakArray<T> where T: Sync + Send { }
