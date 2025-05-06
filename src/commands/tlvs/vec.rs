#[cfg(not(feature = "alloc"))]
pub type TLVVec<T> = ::heapless::Vec<T, 5>;

#[cfg(feature = "alloc")]
pub type TLVVec<T> = ::alloc::vec::Vec<T>;
