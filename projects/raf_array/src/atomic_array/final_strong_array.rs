use core::hash::{Hash, Hasher};

use super::{internal_array::InternalArray, ArrayId, WeakArray};

#[allow(unused_imports)]
use super::StrongArray;

/// The final strong reference to the underlying array. If this instance
/// exists, then it means that strong ref counter is 0 and it is impossible
/// to create new strong references. It still is possible to create weak
/// references though.
/// 
/// Use this class if you want to add logic after all strong references are
/// gone but before the actual memory is deallocated.
pub struct FinalStrongArray<T>
    where T: Sized
{
    internal: InternalArray<T>
}

impl<T> FinalStrongArray<T> {
    pub(super) fn new_raw(internal: InternalArray<T>) -> Self {
        Self { internal }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.internal.as_slice()
    }

    /// Creates new [`WeakArray`] from current [`FinalStrongArray`]. Note
    /// that the newly created [`WeakArray`] instance won't be upgradable
    /// to [`StrongArray`]. The existence of [`FinalStrongArray`] implies
    /// that there are no more strong references alive.
    #[inline(always)]
    pub fn downgrade(&self) -> WeakArray<T> {
        let _ = self.internal.weak_mut().atomic_add(1);
        WeakArray::new_raw(self.internal.clone())
    }

    /// Returns a unique identifier for this array. It is shared between
    /// strong/weak references.
    #[inline(always)]
    pub fn id(&self) -> ArrayId { self.internal.id() }

    /// Returns the number of alive weak references.
    #[inline(always)]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak_mut().atomic_load()
    }
}

impl<T> Drop for FinalStrongArray<T> {
    fn drop(&mut self) {
        let _weak = WeakArray::new_raw(self.internal.clone());
    }
}

impl<T> PartialEq for FinalStrongArray<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.internal == other.internal
    }
}

impl<T> Eq for FinalStrongArray<T>
    where T: Eq
{ }

impl<T> Hash for FinalStrongArray<T>
    where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal.hash(state);
    }
}


unsafe impl<T> Sync for FinalStrongArray<T> where T: Sync + Send { }
unsafe impl<T> Send for FinalStrongArray<T> where T: Sync + Send { }
