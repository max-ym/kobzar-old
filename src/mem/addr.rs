use core::ops::*;

/// Simple wrapper for memory address.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    addr    : usize,
}

impl Into<usize> for Address {

    fn into(self) -> usize {
        self.addr
    }
}

impl From<usize> for Address {

    fn from(addr: usize) -> Self {
        Address { addr:addr }
    }
}

impl Into<isize> for Address {

    fn into(self) -> isize {
        self.addr as _
    }
}

impl From<isize> for Address {

    fn from(addr: isize) -> Self {
        Address { addr:addr as _ }
    }
}

impl Add for Address {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Address { addr : self.addr + rhs.addr }
    }
}

impl Add<usize> for Address {

    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Address { addr : self.addr + rhs }
    }
}

impl AddAssign for Address {

    fn add_assign(&mut self, rhs: Self) {
        self.addr += rhs.addr;
    }
}

impl AddAssign<usize> for Address {

    fn add_assign(&mut self, rhs: usize) {
        self.addr += rhs;
    }
}

impl Sub for Address {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Address { addr : self.addr - rhs.addr }
    }
}

impl Sub<usize> for Address {

    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Address { addr : self.addr - rhs }
    }
}

impl SubAssign for Address {

    fn sub_assign(&mut self, rhs: Self) {
        self.addr = rhs.addr;
    }
}

impl SubAssign<usize> for Address {

    fn sub_assign(&mut self, rhs: usize) {
        self.addr = rhs;
    }
}

impl Mul for Address {

    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Address { addr : self.addr * rhs.addr }
    }
}

impl Mul<usize> for Address {

    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Address { addr : self.addr * rhs }
    }
}

impl MulAssign for Address {

    fn mul_assign(&mut self, rhs: Self) {
        self.addr *= rhs.addr;
    }
}

impl MulAssign<usize> for Address {

    fn mul_assign(&mut self, rhs: usize) {
        self.addr *= rhs;
    }
}

impl Div for Address {

    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Address { addr : self.addr / rhs.addr }
    }
}

impl Div<usize> for Address {

    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Address { addr : self.addr / rhs }
    }
}

impl DivAssign for Address {

    fn div_assign(&mut self, rhs: Self) {
        self.addr /= rhs.addr;
    }
}

impl DivAssign<usize> for Address {

    fn div_assign(&mut self, rhs: usize) {
        self.addr /= rhs;
    }
}

impl Rem for Address {

    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Address { addr : self.addr % rhs.addr }
    }
}

impl Rem<usize> for Address {

    type Output = Self;

    fn rem(self, rhs: usize) -> Self::Output {
        Address { addr : self.addr % rhs }
    }
}

impl RemAssign for Address {

    fn rem_assign(&mut self, rhs: Self) {
        self.addr %= rhs.addr;
    }
}

impl RemAssign<usize> for Address {

    fn rem_assign(&mut self, rhs: usize) {
        self.addr %= rhs;
    }
}

impl Address {

    pub const fn new_from_usize(addr: usize) -> Self {
        Address { addr }
    }

    /// Convert this address to a pointer of a given type.
    pub fn as_ptr<T>(&self) -> *const T {
        self.addr as _
    }

    /// Convert this address to a mutable pointer of a given type.
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.as_ptr::<T>() as _
    }

    /// Get reference to the value.
    ///
    /// # Safety
    /// Caller must ensure that this address points to a valid value.
    pub unsafe fn as_ref<T>(&self) -> &T {
        &*self.as_ptr::<T>()
    }

    /// Get mutable reference to the value.
    ///
    /// # Safety
    /// Caller must ensure that this address points to a valid value.
    pub unsafe fn as_ref_mut<T>(&self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    /// Get the address of a given value reference.
    pub fn address_of<T>(t: &T) -> Self {
        (t as *const T as usize).into()
    }

    /// New null address.
    pub const fn null() -> Self {
        Address { addr : 0 }
    }
}
