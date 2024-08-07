use super::{internal_array::InternalArray, ArrayId};

/// The final weak reference to the underlying array. If this instance
/// exists, then it means that both strong ref count and weak ref count are 0.
/// It is now impossible to create new strong and weak references, and thus
/// this is the final stage before memory deallocation.
/// 
/// Use this class if you want to add logic after all strong and weak
/// references are gone, just before data deallocation.
pub struct FinalWeakArray<T>
    where T: Sized
{
    internal: InternalArray<T>
}

impl<T> FinalWeakArray<T> {
    pub(super) fn new_raw(internal: InternalArray<T>) -> Self {
        Self { internal }
    }

    /// Returns a unique identifier for this array. It is shared between
    /// strong/weak references.
    #[inline(always)]
    pub fn id(&self) -> ArrayId { self.internal.id() }
}

impl<T> Drop for FinalWeakArray<T> {
    fn drop(&mut self) {
        unsafe {
            self.internal.deallocate();
        }
    }
}

unsafe impl<T> Sync for FinalWeakArray<T> where T: Sync + Send { }
unsafe impl<T> Send for FinalWeakArray<T> where T: Sync + Send { }
