mod arch;
pub use self::arch::*;

/// Memory address operations.
mod addr;
pub use self::addr::Address;

/// Minimal memory allocator.
pub trait Allocator {

    /// Allocate next range of bytes.
    fn alloc(&mut self, size: usize) -> Address;
}

/// Allocator for particular data type.
pub trait TypedAllocator : Allocator {

    /// The type of a value to be allocated.
    type T;

    /// Allocate array of value type (uninitialized).
    fn next(&mut self, count: usize) -> *mut Self::T {
        use ::core::mem::size_of;

        let addr: usize = self.alloc(size_of::<Self::T>() * count).into();
        addr as *const Self::T as _
    }
}

/// Allocator that only allocates bytes. Even has no limit.
pub struct SimpleAllocator {

    /// Current address.
    curaddr: Address,
}

impl Allocator for SimpleAllocator {

    fn alloc(&mut self, size: usize) -> Address {
        let addr = self.curaddr;
        self.curaddr += size;
        addr
    }
}

impl SimpleAllocator {

    /// Create new simple allocator which starts allocating from given
    /// address.
    pub fn new(start: Address) -> Self {
        SimpleAllocator { curaddr : start }
    }

    /// Current allocator pointer on last free memory.
    pub fn current_address(&self) -> Address {
        self.curaddr
    }
}
