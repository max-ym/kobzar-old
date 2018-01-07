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

    /// Align next allocation by given byte boundary and allocate
    /// next range of bytes of given amount. First returned address is
    /// the address of requested memory. Second value is an address of
    /// memory that was used for padding (if any). If allocator supports
    /// releasing, both addresses should be released.
    fn alloc_align(&mut self, size: usize, boundary: usize)
            -> (Address, Option<Address>);
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
    /// All memory allocations above will result in NULL pointers.
    /// Byte by the address is the first above the allocatable memory range.
    fn top_limit(&self) -> Address;
}

pub trait AllocatorRelease : Allocator {

    /// Release given addresses provided they were allocated.
    /// First address is inclusive but last is exclusive in range.
    /// If any part of the memory region was not allocated, Err will
    /// be returned and no changes to memory will be made.
    fn release_range(&mut self, from: Address, to: Address) -> Result<(),()>;

    /// Release given amount of bytes starting from given address.
    fn release_size(&mut self, addr: Address, amount: usize) -> Result<(),()> {
        self.release_range(addr, addr + amount)
    }

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

/// Simple allocator that is limited by maximal allocation address.
#[derive(Clone, Copy)]
pub struct TopLimitedAllocator {

    /// Top limit of this allocator.
    maxaddr     : Address,

    alloc       : SimpleAllocator,
}

impl Allocator for SimpleAllocator {

    fn alloc(&mut self, size: usize) -> Address {
        let addr = self.curaddr;
        self.curaddr += size;
        addr
    }

    fn alloc_align(&mut self, size: usize, boundary: usize)
            -> (Address, Option<Address>) {
        if self.curaddr % boundary != 0 {
            let addrval: usize = self.curaddr.into();
            let oldaddr = self.alloc(boundary - addrval % boundary);
            let alloc = self.alloc(size);
            (alloc, Some(oldaddr))
        } else {
            (self.alloc(size), None)
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
