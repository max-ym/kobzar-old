use mem::{Address, Allocator, TypedAllocator, AllocatorRelease};

pub trait LlnAllocator<T>
        : TypedAllocator<LinkedListNode<T>> + AllocatorRelease {
}

/// Linked List implementation.
pub struct LinkedList<T, MA : LlnAllocator<T>> {
    mem     : MA,

    top     : *mut LinkedListNode<T>,
    bot     : *mut LinkedListNode<T>,
}

/// Node of linked list.
pub struct LinkedListNode<T> {
    next    : *mut LinkedListNode<T>,
    data    : T,
}

impl<T, MA> LinkedList<T, MA>
        where MA: LlnAllocator<T> {

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
        let ptr = self.mem.next(1);
        let r   = unsafe { &mut *ptr };
        *r = LinkedListNode {
            next: ::core::ptr::null_mut::<LinkedListNode<T>>(),
            data: t,
        };

        unsafe {
            (*self.bot).next = ptr;
            self.bot = ptr;
        }
    }

    /// Add element first in the list.
    ///
    /// This operation should compute in O(1) time.
    pub fn push_front(&mut self, t: T) {
        let ptr = self.mem.next(1);
        let r   = unsafe { &mut *ptr };
        *r = LinkedListNode {
            next: self.top,
            data: t,
        };

        self.top = ptr;
    }

    /// Replace data in the node with uninitialized value.
    /// Used to take the data from the node before deleting it.
    ///
    /// # Safety
    /// This function may corrupt data in the node so do not
    /// use it after this function call.
    unsafe fn replace_data(lln: *mut LinkedListNode<T>) -> T {
        use core::mem::{replace, uninitialized};

        replace(&mut (*lln).data, uninitialized())
    }

    /// Removes the first element and returns it, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let next_top = unsafe { (*self.top).next };
        let data = unsafe { Self::replace_data(self.top) };

        // Release memory from old node.
        self.mem.release_ptr(self.top).unwrap();

        // Set new list top.
        self.top = next_top;

        Some(data)
    }

    /// Removes the last element from a list and returns it, or None if it is
    /// empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // Find previous node before last element.
        let mut prev = ::core::ptr::null();
        let mut ptr = self.top;
        loop {
            if self.bot == ptr {
                break;
            }

            prev = ptr;
            ptr = unsafe { (*ptr).next };
        };
        // This is node before last one.
        let prev = prev;

        let data = unsafe { Self::replace_data(self.bot) };

        // Release memory used by old node.
        self.mem.release_ptr(self.bot).unwrap(); // TODO proper error check.

        // Set new bottom node.
        self.bot = prev as _;

        Some(data)
    }

    /// Provide a reference to the front element, or None if the list is empty.
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &(*self.top).data })
        }
    }

    /// Provides a mutable reference to the front element, or None if the list
    /// is empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &mut (*self.top).data })
        }
    }

    /// Provide a reference to the back element, or None if the list is empty.
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &(*self.bot).data })
        }
    }

    /// Provides a mutable reference to the back element, or None if the list
    /// is empty.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &mut (*self.bot).data })
        }
    }

    /// Check if list is empty.
    ///
    /// This operation should compute in O(1) time.
    pub fn is_empty(&self) -> bool {
        self.top as usize == 0
    }

    /// Length of the list.
    ///
    /// This operation compute time depends on memory manager element count
    /// time.
    pub fn len(&self) -> usize {
        let mut cur = self.top;
        let mut count = 0;

        while cur != self.bot {
            let r = unsafe { &*cur };
            cur = r.next;
            count += 1;
        }

        count
    }

    /// Removes all elements from the LinkedList.
    ///
    /// This operation should compute in O(n) time.
    pub fn clear(&mut self) {
        let mut ptr = self.top;
        while ptr as usize != 0 {
            unsafe {
                let next = (*ptr).next;
                self.mem.release_ptr(ptr).unwrap();
                ptr = next;
            }
        }
    }
}

impl<T, MA> Drop for LinkedList<T, MA>
        where MA: LlnAllocator<T> {

    fn drop(&mut self) {
        self.clear();
    }
}

/// Iterator over linked list.
pub struct LinkedListIterator<'a, T, MA> where
        T: 'a,
        MA: LlnAllocator<T> + 'a {
    iter: LinkedListNodeIterator<'a, T, MA>,
}

/// Iterator over linked list nodes.
pub struct LinkedListNodeIterator<'a, T, MA> where
        T: 'a,
        MA: LlnAllocator<T> + 'a {
    cur     : *mut LinkedListNode<T>,
    list    : &'a LinkedList<T, MA>,
}

impl<'a, T, MA> Iterator for LinkedListNodeIterator<'a, T, MA>
        where MA: LlnAllocator<T> {

    type Item = *mut LinkedListNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if current node exists.
        if self.cur as usize == 0 {
            return None;
        }

        // Move to next node.
        self.cur = unsafe { (*self.cur).next };

        // Check if new node exists.
        if self.cur as usize == 0 {
            return None;
        }

        Some(self.cur)
    }

    fn last(self) -> Option<Self::Item> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.bot)
        }
    }
}

impl<'a, T, MA> Iterator for LinkedListIterator<'a, T, MA>
        where MA: LlnAllocator<T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(node) => unsafe { Some(&(*node).data) },
            None       => None,
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.iter.last() {
            Some(node) => unsafe { Some(&(*node).data) },
            None       => None,
        }
    }
}

impl<'a, T, MA> LinkedListNodeIterator<'a, T, MA>
        where MA: LlnAllocator<T> {

    /// Create linked list iterator from linked list.
    pub fn new(ll: &'a LinkedList<T, MA>) -> Self {
        LinkedListNodeIterator {
            cur     : ll.top,
            list    : ll,
        }
    }
}

impl<'a, T, MA> IntoIterator for &'a LinkedList<T, MA>
        where MA: LlnAllocator<T> {

    type Item = *mut LinkedListNode<T>;

    type IntoIter = LinkedListNodeIterator<'a, T, MA>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListNodeIterator::new(self)
    }
}

/// Contiguous heap-allocated fixed-size array.
pub struct FixedArray<T> {
    start   : *mut T,
    len     : usize,
}

impl<T> FixedArray<T> {

    /// Create array which starts at specified address and has defined
    /// length. This function does not initialize array elements.
    ///
    /// # Safety
    /// It is up to creator to ensure array address and length are correct.
    /// Otherwise, array can read invalid data or even corrupt memory.
    pub unsafe fn new_unititialized(start: *mut T, length: usize) -> Self {
        FixedArray {
            start   : start,
            len     : length,
        }
    }

    /// Create array which starts at specified address and has defined
    /// length. Elements of array will be initialized with given value.
    ///
    /// # Safety
    /// It is up to creator to ensure array address and length are correct.
    /// Otherwise, array can read invalid data or even corrupt memory.
    pub unsafe fn new(start: *mut T, length: usize, t: T) -> Self
            where T: Clone {
        let mut arr = Self::new_unititialized(start, length);

        // Fill array with initial value.
        let mut i = 0;
        while i < arr.len {
            *arr.get_unchecked_mut(i) = t.clone();
            i += 1;
        }

        arr
    }

    /// Get element reference by given index, if any.
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.out_of_bounds(index) {
            None
        } else {
            unsafe {
                Some(self.get_unchecked(index))
            }
        }
    }

    /// Get element reference by given index without bound check.
    ///
    /// # Safety
    /// Use it carefully and ensure bounds are not broken.
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        &*self.start.offset(index as _)
    }

    /// Get mutable element reference by given index, if any.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.out_of_bounds(index) {
            None
        } else {
            unsafe {
                Some(self.get_unchecked_mut(index))
            }
        }
    }

    fn out_of_bounds(&self, index: usize) -> bool {
        self.len <= index
    }

    /// Get mutable element reference by given index without bound check.
    ///
    /// # Safety
    /// Use it carefully and ensure bounds are not broken.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        &mut *self.start.offset(index as _)
    }

    /// Array length.
    pub fn length(&self) -> usize {
        self.len
    }

    /// Array size in bytes.
    pub fn bytes_length(&self) -> usize {
        self.len * ::core::mem::size_of::<T>()
    }

    /// Raw pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    pub fn as_ptr(&self) -> *const T {
        self.start
    }

    /// Unsafe mutable pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.start
    }

    /// Swaps two elements in an array.
    pub fn swap(&mut self, a: usize, b: usize) -> bool {
        if self.out_of_bounds(a) || self.out_of_bounds(b) {
            return false;
        }

        unsafe {
            use core::mem::swap;
            let ptr_a = self.as_ptr().offset(a as _) as *mut T;
            let ptr_b = self.as_ptr().offset(b as _) as *mut T;

            swap(&mut *ptr_a, &mut *ptr_b);
        }

        true
    }

    /// Reverse the order of elements in an array.
    pub fn reverse(&mut self) {
        let mut i = 0;
        let len = self.len;
        while i < len / 2 {
            self.swap(i, len - 1 - i);
            i += 1;
        }
    }
}
