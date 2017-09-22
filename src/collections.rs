
/// Linked List implementation.
pub struct LinkedList<T> {
    top     : *mut LinkedListNode<T>
}

/// Node of linked list.
struct LinkedListNode<T> {
    pub next    : *mut LinkedListNode<T>,
    pub data    : T,
}
