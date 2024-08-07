#[derive(Debug)]
pub enum NewStrongArrayError {
    MaxLengthExceeded,
    AllocationError,
    MisalignedResultError,
}

#[derive(Debug)]
pub enum WeakUpgradeError {
    NoStrongReference,
}
