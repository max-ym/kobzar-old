mod arch;
pub use self::arch::*;

/// Memory address operations.
mod addr;
pub use self::addr::Address;

/// Minimal memory allocator.
pub trait Allocator {

    /// Allocate next range of bytes.
    fn alloc(&mut self, size: usize) -> Address;

    /// Allocate next range of bytes exactly as much as needed for given
    /// type.
    fn alloc_for<T>(&mut self) -> Address {
        self.alloc(::core::mem::size_of::<T>())
    }
}

pub trait AllocatorAlign : Allocator {

    /// Align next allocation to given byte boundary. If any memory was
    /// allocated for aligning, amount of allocated memory and it's address
    /// is returned.
    fn align(&mut self, val: usize) -> Option<(usize, Address)>;
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

/// Allocator that has memory amount limitation.
pub trait AllocatorTopLimit : Allocator {

    /// Maximal address value after which no memory can be allocated.
    /// All memory allocations will result in NULL pointers.
    /// This address is not included in allocatable memory.
    fn top_limit(&self) -> Address;
}

/// Allocator that only allocates bytes. Even has no limits.
#[derive(Clone, Copy)]
pub struct SimpleAllocator {

    /// Current address.
    curaddr: Address,
}

#[derive(Clone, Copy)]
pub struct TopLimitedAllocator {

    /// Top limit of this allocator.
    maxaddr     : Address,

    /// Current address.
    curaddr     : Address,
}

impl Allocator for SimpleAllocator {

    fn alloc(&mut self, size: usize) -> Address {
        let addr = self.curaddr;
        self.curaddr += size;
        addr
    }
}

impl AllocatorAlign for SimpleAllocator {

    fn align(&mut self, val: usize) -> Option<(usize, Address)> {
        let next_boundary = ((self.curaddr + val - 1) / val) * val;
        let padding_size: usize = (next_boundary - self.curaddr).into();

        if padding_size == 0 {
            None // Memory is already aligned
        } else {
            Some((padding_size, self.alloc(padding_size)))
        }
    }
}

impl SimpleAllocator {

    /// Create new simple allocator which starts allocating from given
    /// address.
    pub const fn new(start: Address) -> Self {
        SimpleAllocator { curaddr : start }
    }

    /// Current allocator pointer on last free memory.
    pub fn current_address(&self) -> Address {
        self.curaddr
    }
}

impl Allocator for TopLimitedAllocator {

    fn alloc(&mut self, size: usize) -> Address {
        // Check limits.
        if self.curaddr + size >= self.top_limit() {
            return Address::null();
        }

        let addr = self.curaddr;
        self.curaddr += size;
        Address::from(addr)
    }
}

impl AllocatorAlign for TopLimitedAllocator {

    fn align(&mut self, val: usize) -> Option<(usize, Address)> {
        let next_boundary = ((self.curaddr + val - 1) / val) * val;
        let padding_size: usize = (next_boundary - self.curaddr).into();

        if padding_size == 0 {
            None // Memory is already aligned
        } else {
            let addr = self.alloc(padding_size);
            if addr == Address::null() {
                None // Could not allocate
            } else {
                Some((padding_size, addr))
            }
        }
    }
}

impl AllocatorTopLimit for TopLimitedAllocator {

    fn top_limit(&self) -> Address {
        self.maxaddr
    }
}

impl TopLimitedAllocator {

    /// Create new allocator from given start address to given end address.
    pub const fn new(start: Address, end: Address) -> Self {
        TopLimitedAllocator {
            maxaddr : end,
            curaddr : start,
        }
    }

    /// Current allocator pointer on last free memory.
    pub fn current_address(&self) -> Address {
        self.curaddr
    }
}
