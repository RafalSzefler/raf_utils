#[allow(unused_imports)]
use super::{StrongArray, WeakArray};

/// Represents errors during [`StrongArray`] construction.
#[derive(Debug)]
pub enum NewStrongArrayError {
    MaxLengthExceeded,
    AllocationError,
    MisalignedResultError,
}

/// Represents errors during [`WeakArray`] upgrade to [`StrongArray`].
#[derive(Debug)]
pub enum WeakUpgradeError {
    NoStrongReference,
}
