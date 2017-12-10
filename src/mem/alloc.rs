use super::Address;

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
pub trait TypedAllocator<T> : Allocator {

    /// Allocate array of value type (uninitialized).
    fn next(&mut self, count: usize) -> *mut T {
        use ::core::mem::size_of;

        let addr: usize = self.alloc(size_of::<T>() * count).into();
        addr as *const T as _
    }
}

/// Allocator that has memory amount limitation.
pub trait AllocatorTopLimit : Allocator {

    /// Maximal address value after which no memory can be allocated.
    /// All memory allocations will result in NULL pointers.
    /// This address is not included in allocatable memory.
    fn top_limit(&self) -> Address;
}

pub trait AllocatorRelease : Allocator {

    /// Release given addresses provided they were allocated.
    /// First address is inclusive but last is exclusive in range.
    /// If any part of the memory region was not allocated, Err will
    /// be returned and no changes to memory will be made.
    fn release_range(&mut self, from: Address, to: Address) -> Result<(),()>;

    /// Release value by given pointer provided it was allocated by
    /// this allocator.
    /// If any part of the memory region was not allocated, Err will
    /// be returned and no changes to memory will be made.
    fn release_ptr<T>(&mut self, t: *const T) -> Result<(),()> {
        let start = Address::from(t as usize);
        let end   = start + ::core::mem::size_of::<T>();

        self.release_range(start, end)
    }

    /// Check if given address range can be released by `release_range`.
    fn is_releasable_range(&self, from: Address, to: Address) -> bool;
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
