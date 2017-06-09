use arch::{DtLimit, RegValue};
use arch::gdt::*;
use arch::tentr::*;
use mem::map::GDT;

/// Make a stack from a GDT table. Allows to push new entries and
/// keeps record of entry count. This allows to update GDT limit to a
/// new correct value to correspond to the final element count.
struct GdtStack {
    counter : u16,
}

impl GdtStack {

    /// Pointer to GdtDescriptor array (which is GDT table).
    fn gdt() -> *mut GdtDescriptor {
        GDT as *const GdtDescriptor as *mut _
    }

    pub unsafe fn push(&mut self, dsc: GdtDescriptor) {
        *Self::gdt().offset(self.counter as _) = dsc;
        self.counter += 1;
    }

    pub fn counter(&self) -> u16 {
        self.counter
    }

    pub fn new() -> Self {
        GdtStack {
            counter: 0
        }
    }
}

pub fn setup() {
    let mut gdtr  = GdtrValue::new(GDT as _, 0);
    let mut table = gdtr.into_table();
    let mut stack = GdtStack::new();

    unimplemented!();

    // Update limit to correspond to entry count.
    unsafe { table.set_limit_by_entry_count(stack.counter()); }

    // Save new GDT pointer to GDTR.
    let gdtr = GdtrValue::from_table(table);
    unsafe { gdtr.write(); }
}
