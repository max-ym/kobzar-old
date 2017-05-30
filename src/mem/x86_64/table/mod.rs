mod page;
pub use self::page::*;

#[repr(packed)]
pub struct P4 {
    entries : [P4E; 512]
}

#[repr(packed)]
pub struct P3 {
    entries : [P3E; 512]
}

#[repr(packed)]
pub struct P2 {
    entries : [P2E; 512]
}

#[repr(packed)]
pub struct P1 {
    entries : [P1E; 512]
}

impl Default for P1 {

    fn default() -> Self {
        P1 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P2 {

    fn default() -> Self {
        P2 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P3 {

    fn default() -> Self {
        P3 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P4 {

    fn default() -> Self {
        P4 {
            entries : [Default::default(); 512]
        }
    }
}
