/// Represents errors during [`StrongArray`][super::StrongArray] construction.
#[derive(Debug)]
pub enum NewStrongArrayError {
    MaxLengthExceeded,
    AllocationError,
    MisalignedResultError,
}

/// Represents errors during [`WeakArray`][super::WeakArray] upgrade to
/// [`StrongArray`][super::StrongArray].
#[derive(Debug)]
pub enum WeakUpgradeError {
    NoStrongReference,
}
