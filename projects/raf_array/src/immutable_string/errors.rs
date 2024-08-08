use crate::atomic_array::NewStrongArrayError;

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
