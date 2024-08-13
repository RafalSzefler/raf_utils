use crate::atomic_array::NewStrongArrayError;

#[allow(unused_imports)]
use super::ImmutableString;

/// Represents errors during [`ImmutableString`] construction.
#[derive(Debug)]
pub enum NewImmutableStringError {
    MaxLengthExceeded,
    AllocationError,
}

impl From<NewStrongArrayError> for NewImmutableStringError {
    fn from(value: NewStrongArrayError) -> Self {
        match value {
            NewStrongArrayError::MaxLengthExceeded => NewImmutableStringError::MaxLengthExceeded,
            NewStrongArrayError::AllocationError | NewStrongArrayError::MisalignedResultError
                => NewImmutableStringError::AllocationError,
        }
    }
}
