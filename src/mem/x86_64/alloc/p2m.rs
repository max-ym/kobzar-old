/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut u64,

    /// The count of addresses on the stack.
    count   : u32,
}
