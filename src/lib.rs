mod encrypted_log;

pub use encrypted_log::EncryptedLog;
use generic_array::{ArrayLength, GenericArray};

/// Convert slices to `GenericArray`s without panicking.
pub fn generic_array_try_from_slice<T, N>(data: &[T]) -> Option<&GenericArray<T, N>>
where
    N: ArrayLength<T>,
{
    if data.len() == N::to_usize() {
        Some(data.into())
    } else {
        None
    }
}
