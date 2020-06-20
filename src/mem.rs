pub unsafe fn allocRaw(size: usize) -> *mut u8 {
    //let addr = libc::memalign(core::mem::size_of::<usize>(), size) as *mut u8;
    let addr = libc::calloc(core::mem::size_of::<usize>(), size) as *mut u8;
    //libc::memset(addr as *mut libc::c_void, 0, size);
    addr
}

pub unsafe fn freeRaw(arr: *mut u8) {
    libc::free(arr as *mut libc::c_void);
}

pub unsafe fn alloc<T>() -> *mut T {
    allocRaw(core::mem::size_of::<T>()) as *mut T
}

pub unsafe fn free<T>(t: *mut T) {
    freeRaw(t as *mut u8)
}

// TODO: change this to const generics when they become stable and return a slice
pub unsafe fn allocArray<T>(count: usize) -> *mut T {
    allocRaw(core::mem::size_of::<T>() * count) as *mut T
}

// TODO: change this to slice once const generics stable
pub unsafe fn freeArray<T>(ptr: *mut T, count: usize) {
    let arr      = core::slice::from_raw_parts_mut(ptr, count); // this will keep a pointer (will not free it)
    for i in 0..count {
        ::core::ptr::drop_in_place(&arr[i] as *const T as *mut T);
    }
    free(ptr);
}

#[repr(C)]
pub struct Unique<T: ?Sized> {
    ptr         : *mut T,
    _marker     : ::core::marker::PhantomData<T>,
}

impl<T> Unique<T> {
    pub fn new(ptr: *mut T) -> Self { Self { ptr : ptr, _marker: ::core::marker::PhantomData } }
    pub fn getMutPtr(&mut self) -> *mut T { self.ptr }
    pub fn getPtr(&self) -> *const T { self.ptr }
}

#[repr(C)]
pub struct Box<T>{
    uptr: Unique<T>
}

impl<T> Box<T> {
    /// Allocates memory on the heap and then places `x` into it.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = Box::new(5);
    /// ```
    #[inline(always)]
    pub fn new(x: T) -> Box<T> {
        unsafe {
            let addr = alloc::<T>();
            *addr = x;
            Self { uptr: Unique::new(addr) }
        }
    }

    pub fn asRef(&self) -> &T { unsafe { &(*self.uptr.getPtr()) } }
    pub fn asMut(&mut self) -> &T { unsafe { &mut (*self.uptr.getMutPtr()) } }
    pub fn intoRaw(self) -> *mut T {
        let m = ::core::mem::ManuallyDrop::new(self);
        m.uptr.ptr
    }

    pub fn fromRaw(raw: *mut T) -> Self {
        Self { uptr: Unique::new(raw) }
    }

    pub fn unbox(self) -> T {
        unsafe {
            let ptr = self.uptr.ptr;
            let v = self.intoRaw().read();
            free(ptr);
            v
        }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            let addr = self.uptr.getMutPtr();
            ::core::ptr::drop_in_place(addr);
            free(addr);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testDrop() {
        let _b0 = Box::new(1234);
        let _b1 = Box::new(1234345);
        let mut v = crate::vec::Vec::new();
        for i in 0..100 {
            v.pushBack(i);
        }
        let _bv = Box::new(v);
    }

    #[test]
    fn testDropVecVec() {
        let _b0 = Box::new(1234);
        let _b1 = Box::new(1234345);
        let mut v = crate::vec::Vec::new();
        for _ in 0..100 {
            let mut vj = crate::vec::Vec::new();
            for j in 0..100 {
                vj.pushBack(j);
            }
            v.pushBack(vj);
        }
        let _bv = Box::new(v);
    }

    #[test]
    fn testBoxUnbox() {
        let b = Box::new(1234);
        let _v = b.unbox();
    }

    #[test]
    fn testBoxUnboxVecVec() {
        let _b0 = Box::new(1234);
        let _b1 = Box::new(1234345);
        let mut v = crate::vec::Vec::new();
        for _ in 0..100 {
            let mut vj = crate::vec::Vec::new();
            for j in 0..100 {
                vj.pushBack(j);
            }
            v.pushBack(vj);
        }
        let v2 = Box::new(v);
        let _v3 = v2.unbox();
    }

    #[test]
    fn testBoxFromToRaw() {
        let b = Box::new(1234);
        let r = b.intoRaw();
        let _b = Box::fromRaw(r);
    }
}
