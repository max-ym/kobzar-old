use mem::Address;

#[derive(Clone, Copy, PartialEq)]
/// 2MiB page info.
pub struct Page2m {

    /// Address of a page.
    addr    : Address
}

impl Page2m {

    pub fn new(addr: Address) -> Self {
        Page2m { addr : addr }
    }

    pub fn addr(&self) -> Address {
        self.addr
    }
}
