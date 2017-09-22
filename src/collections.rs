
/// Linked List implementation.
pub struct LinkedList<T, MA : MemoryAllocator<LinkedListNode<T>>> {
    mem     : MA,

    top     : *mut LinkedListNode<T>,
    bot     : *mut LinkedListNode<T>,
}

/// Node of linked list.
pub struct LinkedListNode<T> {
    next    : *mut LinkedListNode<T>,
    data    : T,
}

/// Memory Allocator for specified type.
pub trait MemoryAllocator<T> {

    fn next(&mut self, t: T) -> *mut T;

    /// Allocated elements count.
    fn count(&self) -> usize;

    /// Release previously allocated memory by given address.
    /// If successfully, then Ok will be returned. Otherwise,
    /// pointer is given back.
    fn free(&mut self, ptr: *mut T) -> Result<(), *mut T>;
}

impl<T, MA> LinkedList<T, MA>
        where MA: MemoryAllocator<LinkedListNode<T>> {

    /// Create empty linked list.
    /// Argument: memory allocator that will allocate space for list nodes
    /// and release when nodes get deallocated.
    pub fn new(ma: MA) -> Self {
        LinkedList {
            mem : ma,
            top : ::core::ptr::null_mut::<LinkedListNode<T>>(),
            bot : ::core::ptr::null_mut::<LinkedListNode<T>>(),
        }
    }

    /// Add element last in the list.
    ///
    /// This operation should compute in O(1) time.
    pub fn push_back(&mut self, t: T) {
        let ptr = self.mem.next(LinkedListNode {
            next: ::core::ptr::null_mut::<LinkedListNode<T>>(),
            data: t,
        });

        unsafe {
            (*self.bot).next = ptr;
            self.bot = ptr;
        }
    }

    /// Add element first in the list.
    ///
    /// This operation should compute in O(1) time.
    pub fn push_front(&mut self, t: T) {
        let ptr = self.mem.next(LinkedListNode {
            next: self.top,
            data: t,
        });

        self.top = ptr;
    }

    /// Check if list is empty.
    ///
    /// This operation should compute in O(1) time.
    pub fn is_empty(&self) -> bool {
        self.top as u64 == 0
    }

    /// Length of the list.
    ///
    /// This operation compute time depends on memory manager element count
    /// time.
    pub fn len(&self) -> usize {
        self.mem.count()
    }
}
