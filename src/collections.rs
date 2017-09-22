
/// Linked List implementation.
pub struct LinkedList<T> {
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
}

impl<T> LinkedList<T> {

    /// Create empty linked list.
    pub fn new() -> Self {
        LinkedList {
            top : ::core::ptr::null_mut::<LinkedListNode<T>>(),
            bot : ::core::ptr::null_mut::<LinkedListNode<T>>(),
        }
    }

    /// Add element last in the list.
    pub fn push_back<MA : MemoryAllocator<LinkedListNode<T>>>
            (&mut self, mem: &mut MA, t: T) {
        let ptr = mem.next(LinkedListNode {
            next: ::core::ptr::null_mut::<LinkedListNode<T>>(),
            data: t,
        });

        unsafe {
            (*self.bot).next = ptr;
            self.bot = ptr;
        }
    }
}
