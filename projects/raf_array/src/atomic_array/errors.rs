/// Represents errors during [`StrongArray`][super::StrongArray] construction.
#[derive(Debug)]
#[repr(u8)]
pub enum NewStrongArrayError {
    MaxLengthExceeded = 0,
    AllocationError = 1,
    MisalignedResultError = 2,
}

/// Represents errors during [`WeakArray`][super::WeakArray] upgrade to
/// [`StrongArray`][super::StrongArray].
#[derive(Debug)]
#[repr(u8)]
pub enum WeakUpgradeError {
    NoStrongReference = 0,
}
