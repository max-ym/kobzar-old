use mem::{Address, Allocator, TypedAllocator, AllocatorRelease};

pub trait LlnAllocator<T>
        : TypedAllocator<LinkedListNode<T>> + AllocatorRelease {
}

pub trait HeapAllocator<T>
        : TypedAllocator<HeapFrame<T>> + AllocatorRelease {
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

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitmap64 {
    val     : u64,
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

/// Stack without memory allocation.
pub struct FreeStack<T>
    where T: Clone + Copy {

    /// Current element pointer.
    cur     : *mut T,

    /// Count of total stored elements.
    count   : usize,
}

/// The heap that stores an instance of particular type only.
///
/// Heap is represented by frames. Each frame stores information
/// about usage of 64 memory cells allocated for storing data of given
/// type. It contains these cells and the pointer to next heap frame.
/// Frames are cycled - last frame points to first one. Thus, heap
/// of only one frame has it's pointer referring to itself.
#[cfg(target_arch = "x86_64")]
pub struct Heap<'a, T, MA: HeapAllocator<T> + 'a> {

    /// Memory allocator used to allocate and release memory for frames.
    mem     : &'a mut MA,

    /// The first frame of the heap. May be NULL.
    top     : *mut HeapFrame<T>,

    /// Counter of free entries.
    free    : usize,
}

/// The heap frame. See Heap docs for more info.
pub struct HeapFrame<T> {

    /// Bitmap of current frame usage. Shows what cells are released or used.
    map     : Bitmap64,

    /// Next frame of heap.
    next    : *mut HeapFrame<T>,

    /// Array with data that current heap frame stores.
    arr     : [T; 64],
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

macro_rules! bitmap64_convert {
    ($opt:expr) => {{
        match $opt {
            Some(t) => Some(t as _),
            None    => None
        }
    }};
}

impl Bitmap64 {

    /// Whether all bits are ones.
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        self.val == !0
    }

    /// Whether all bits are zeros.
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        self.val == 0
    }

    /// First bit index with value one.
    #[inline(always)]
    pub fn first_one(&self) -> Option<usize> {
        self.first_one_arch()
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn first_one_arch(&self) -> Option<usize> {
        use arch::bit::bsf_u64 as bsf;
        bitmap64_convert!(bsf(self.val))
    }

    /// First bit index with value zero.
    #[inline(always)]
    pub fn first_zero(&self) -> Option<usize> {
        self.first_zero_arch()
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn first_zero_arch(&self) -> Option<usize> {
        use arch::bit::bsf_u64 as bsf;
        bitmap64_convert!(bsf(!self.val))
    }

    /// Last bit index with value one.
    #[inline(always)]
    pub fn last_one(&self) -> Option<usize> {
        self.last_one_arch()
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn last_one_arch(&self) -> Option<usize> {
        use arch::bit::bsr_u64 as bsr;
        bitmap64_convert!(bsr(self.val))
    }

    /// Last bit index with value zero.
    #[inline(always)]
    pub fn last_zero(&self) -> Option<usize> {
        self.last_zero_arch()
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn last_zero_arch(&self) -> Option<usize> {
        use arch::bit::bsr_u64 as bsr;
        bitmap64_convert!(bsr(!self.val))
    }

    /// Set bit by given index to one. If value is bigger than bitmap,
    /// nothing will be changed.
    #[inline(always)]
    pub fn set_one(&mut self, index: usize)  {
        self.val |= 1 << index;
    }

    /// Set bit by given index to zero. If value is bigger than bitmap,
    /// nothing will be changed.
    #[inline(always)]
    pub fn set_zero(&mut self, index: usize)  {
        self.val &= !0 ^ 1 << index;
    }

    /// Invert all ones to zeros and vice versa.
    #[inline(always)]
    pub fn invert(&mut self) {
        self.val = !self.val;
    }


    /// Whether bit at given index is set to one. If index is greater that
    /// amount of bits in the bitmap then 'false' will be returned.
    #[inline(always)]
    pub fn is_one(&self, index: usize) -> bool {
        self.val & (1 << index) != 0
    }

    /// Whether bit at given index is set to zero. If index is greater that
    /// amount of bits in the bitmap then 'true' will be returned.
    #[inline(always)]
    pub fn is_zero(&self, index: usize) -> bool {
        self.val & (1 << index) == 0
    }
}

impl Into<u64> for Bitmap64 {

    fn into(self) -> u64 {
        self.val
    }
}

impl Into<i64> for Bitmap64 {

    fn into(self) -> i64 {
        self.val as _
    }
}

impl From<u64> for Bitmap64 {

    fn from(val: u64) -> Self {
        Bitmap64 { val }
    }
}

impl From<i64> for Bitmap64 {

    fn from(val: i64) -> Self {
        let val = val as u64;
        Bitmap64 { val }
    }
}

impl<T> FreeStack<T>
    where T: Clone + Copy {

    /// Create new unchecked stack starting with given memory pointer.
    pub const fn new(start: *mut T) -> Self {
        FreeStack {
            cur     : start,
            count   : 0,
        }
    }

    /// Element count of the stack.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Last element reference.
    pub fn last(&self) -> Option<&T> {
        if self.count > 0 {
            Some(unsafe { &*self.cur })
        } else {
            None
        }
    }

    /// Last element mutable reference.
    pub fn last_mut(&self) -> Option<&mut T> {
        if self.count > 0 {
            Some(unsafe { &mut *self.cur })
        } else {
            None
        }
    }

    /// Pop last element from the stack.
    pub fn pop(&mut self) -> Option<T> {
        if self.count > 0 {
            self.count -= 1;
            let r = unsafe { &mut *self.cur };
            unsafe { self.cur = self.cur.offset(1) };
            Some(r.clone())
        } else {
            None
        }
    }

    /// Push new element onto the stack.
    pub fn push(&mut self, elm: T) {
        self.count += 1;
        unsafe { self.cur.offset(-1) };
        unsafe { *self.cur = elm };
    }
}

#[cfg(target_arch = "x86_64")]
impl<'a, T, MA> Heap<'a, T, MA>
        where MA: HeapAllocator<T> {

    pub const FRAME_LENGTH: usize = 64;

    /// Create new heap. For memory operations use given frame allocator.
    /// The heap will not be created if memory allocator fails.
    pub fn new(alloc: &'a mut MA) -> Option<Self> {
        let initial_frame_ptr = alloc.next(1);
        if initial_frame_ptr.is_null() {
            None
        } else {
            Some(Heap {
                mem     : alloc,
                top     : initial_frame_ptr,
                free    : Self::FRAME_LENGTH,
            })
        }
    }

    /// Allocate new frame and increase counter of free entries.
    /// None is returned if memory allocator fails.
    fn allocate_new_frame(&mut self) -> Option<&mut HeapFrame<T>> {
        let new_frame_ptr = self.mem.next(1);
        if new_frame_ptr.is_null() {
            // Failed to allocate memory.
            return None;
        }

        // Find proper place to store the frame. The heap frames must be
        // sorted ascending. This is required because of release search
        // optimization which expects ascending order. See 'release_frame' fn.
        let mut prev = self.top;
        let mut cur = self.top;
        loop {
            if cur > new_frame_ptr {
                // 'cur' became great enough and must be stored after
                // 'new_frame'. 'prev' must be stored before 'new_frame'.

                // Initialize new frame with proper values.
                unsafe {
                    let new_frame = &mut *new_frame_ptr;
                    new_frame.initialize(cur);
                }

                // Stop 'prev' from pointing to 'cur' and point to 'new_frame'
                // instead.
                unsafe {
                    let prev = &mut *prev;
                    prev.set_next_frame(new_frame_ptr);
                }

                break;
            }

            // Move to next frame.
            unsafe {
                let c = &*cur;
                prev = cur;
                cur = c.next;
            }
        }

        // Update free entry counter.
        self.free += Self::FRAME_LENGTH;

        unsafe {
            Some(&mut *new_frame_ptr)
        }
    }

    /// Release a frame that is no longer in use. Note that this fn does not
    /// check whether the frame is really empty. This is the reason it is
    /// marked unsafe.
    /// Returns Err if frame is not present in the heap. Ok does not mean that
    /// frame was deallocated. It will be kept if it is the only frame in
    /// the heap.
    unsafe fn release_frame(&mut self, frame: &HeapFrame<T>) -> Result<(),()> {
        // NOTE. The implementation expects ascending order of frames.

        let mut prev = self.top;
        let mut cur = self.top;
        loop {
            let c = &*cur;
            if c < frame {
                // Search further to find the given frame.
                prev = cur;
                cur = c.next;
            } else if c == frame {
                // Found the frame! Release it.
                if prev == cur {
                    // This is the only frame in the heap and cannot be
                    // released!
                    return Ok(());
                }

                (&mut *prev).set_next_frame(c.next);
                self.mem.release_ptr(cur).unwrap();
                return Ok(());
            } else if c > frame {
                // Frame is not present in the heap!
                return Err(());
            }
        }
    }

    /// Allocate new entry. None if memory allocator fails.
    pub fn allocate(&mut self) -> Option<&mut T> {
        // Find if any frame still has free space.
        if self.free == 0 {
            // Allocate new frame.
            let addr = {
                let frame = self.allocate_new_frame();
                if frame.is_none() {
                    return None;
                }
                let frame = frame.unwrap();
                frame as *const HeapFrame<T> as *mut HeapFrame<T>
            };

            // Allocate new entry from new frame.
            self.free -= 1;
            let entry = unsafe { &mut *addr }.allocate();
            return entry;
        }

        // Find not full frame.
        let mut cur = self.top;
        loop {
            // Try allocating in current frame.
            unsafe {
                let c = &mut *cur;
                let i = c.allocate();
                if i.is_some() {
                    return Some(i.unwrap());
                }
            }

            // Move to next frame.
            unsafe {
                let c = &*cur;
                cur = c.next_frame();
            }
        }
    }

    /// Release the entry from the heap. It must not be accessed any further.
    pub fn release(&mut self, entry: &T) -> Result<(),()> {
        use core::cmp::Ordering;

        let mut cur = self.top;
        loop {
            let result = unsafe {
                let c = &*cur;
                c.index_of(entry)
            };

            if result.is_err() {
                let ord = result.unwrap_err();
                if ord == Ordering::Greater {
                    // Reached end of arrays lower of given
                    // address. As all arays are sorted in ascending
                    // order then this heap does not contain entry.
                    return Err(());
                }
                // continue otherwise...
            } else {
                // Found! Release the entry.
                let index = result.unwrap();

                unsafe {
                    let c = &mut *cur;
                    c.release(index).unwrap();

                    // Release the frame if it is empty.
                    if c.is_empty() {
                        self.release_frame(c).unwrap();
                    }
                }
                self.free += 1;
                return Ok(());
            }

            // Move to next frame.
            unsafe {
                let c = &*cur;
                cur = c.next;
            }
        }
    }

    /// Number of free entries.
    pub fn free_count(&self) -> usize {
        self.free
    }
}

impl<T> HeapFrame<T> {

    /// Initialize given frame of the heap. It was previously allocated by
    /// the memory allocator and must be initialized by this function.
    /// The 'next' address points to next frame of the heap. if this frame is
    /// the only one then given pointer must point to self instance.
    pub fn initialize(&mut self, next: *mut HeapFrame<T>) {
        use core::mem::uninitialized;

        *self = HeapFrame {
            map     : Default::default(),
            next,
            arr     : unsafe { uninitialized() },
        };
    }

    fn is_present(&self, index: usize) -> bool {
        self.map.is_zero(index)
    }

    fn is_absent(&self, index: usize) -> bool {
        !self.is_present(index)
    }

    /// Mark that corresponding value in array cell as present.
    fn mark_present(&mut self, index: usize) {
        self.map.set_zero(index)
    }

    /// Mark that corresponding value in array cell as absent.
    fn mark_absent(&mut self, index: usize) {
        self.map.set_one(index)
    }

    /// Find free array cell.
    fn find_free(&self) -> Option<usize> {
        self.map.first_one()
    }

    /// Release entry in given cell. If there was no entry, Err is returned.
    pub fn release(&mut self, index: usize) -> Result<(),()> {
        if self.is_absent(index) {
            return Err(());
        }

        self.mark_absent(index);
        Ok(())
    }

    /// Allocate new entry. If no more free space then None will be returned.
    pub fn allocate(&mut self) -> Option<&mut T> {
        let free_index = self.find_free();
        if free_index.is_none() {
            return None;
        }
        let free_index = free_index.unwrap();

        self.mark_present(free_index);
        Some(&mut self.arr[free_index])
    }

    /// Address of next frame.
    pub fn next_frame(&self) -> *mut Self {
        self.next
    }

    /// Set new next frame. Old one will be returned.
    pub fn set_next_frame(&mut self, next: *mut HeapFrame<T>)
            -> *mut HeapFrame<T> {
        let old = self.next;
        self.next = next;
        old
    }

    /// Calculate index of given entry reference. If entry is not from this
    /// frame then return whether it's address is greater or less that the
    /// address of internal entry array.
    pub fn index_of(&self, entry: &T) -> Result<usize, ::core::cmp::Ordering> {
        use core::cmp::Ordering;

        let addr = entry as *const T;
        let arr_addr = &self.arr[0] as *const T;
        let arr_end = unsafe { arr_addr.offset(self.arr.len() as _) };

        let addr        = addr      as isize;
        let arr_addr    = arr_addr  as isize;
        let arr_end     = arr_end   as isize;

        let arr_size    = arr_end - arr_addr;

        let offset = arr_end - addr;
        if offset > arr_size {
            Err(Ordering::Greater)
        } else if offset < 0 {
            Err(Ordering::Less)
        } else {
            use core::mem::size_of;
            let index = offset as usize / size_of::<T>();
            Ok(index)
        }
    }

    /// Whether this frames contains no entries.
    pub fn is_empty(&self) -> bool {
        self.map.is_all_ones()
    }
}

impl<T> PartialEq for HeapFrame<T> {

    fn eq(&self, other: &HeapFrame<T>) -> bool {
        let self_addr = self  as *const HeapFrame<T> as usize;
        let rhs_addr  = other as *const HeapFrame<T> as usize;

        self_addr.eq(&rhs_addr)
    }
}

impl<T> PartialOrd for HeapFrame<T> {

    fn partial_cmp(&self, other: &HeapFrame<T>)
            -> Option<::core::cmp::Ordering> {
        let self_addr = self  as *const HeapFrame<T> as usize;
        let rhs_addr  = other as *const HeapFrame<T> as usize;

        self_addr.partial_cmp(&rhs_addr)
    }
}
