use core::*;
use crate::mem::*;

#[repr(C)]
pub struct Vec<T> {
    elements    : *mut T,
    count       : usize,
    capacity    : usize,
}

impl<T> Vec<T> {
    pub fn withCapacity(c: usize) -> Self {
        if c == 0 { Self::new() }
        else {
            Self {
                elements: unsafe { allocRaw(c * mem::size_of::<T>()) as *mut T },
                count   : 0,
                capacity: c,
            }
        }
    }

    pub fn new() -> Self {
        Self {
            elements: ptr::null_mut(),
            count   : 0,
            capacity: 0,
        }
    }

    pub fn asArray(&self) -> &[T] { unsafe { core::slice::from_raw_parts(self.elements, self.count) } }
    pub fn asMutArray(&mut self) -> &mut [T] { unsafe { core::slice::from_raw_parts_mut(self.elements, self.count) } }

    pub fn len(&self) -> usize { self.count }

    pub fn pushBack(&mut self, t: T) {
        if self.count >= self.capacity {
            let newSize     = if self.capacity == 0 { 16 } else { self.capacity * 2 };
            let newPtr      = unsafe { allocRaw(newSize * mem::size_of::<T>()) as *mut T };
            let oldPtr      = self.elements;
            self.capacity   = newSize;

            for i in 0..self.count {
                let v = unsafe { oldPtr.offset(i as isize).read() };    // v = old[i];
                unsafe { newPtr.offset(i as isize).write(v) };          // new[i] = v;
            }
            unsafe { free(self.elements) };
            self.elements   = newPtr;
        }

        unsafe { self.elements.offset(self.count as isize).write(t) };
        self.count += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 { None }
        else {
            let nc = self.count - 1;
            let v = unsafe { ptr::read(self.get(nc) as *const _) };
            self.count -= 1;
            Some(v)
        }
    }

    #[inline]
    pub fn get(&self, idx: usize) -> &T {
        let arr      = unsafe { core::slice::from_raw_parts(self.elements, self.count) };
        &arr[idx]
    }

    #[inline]
    pub fn getMut(&mut self, idx: usize) -> &mut T {
        let arr      = unsafe { core::slice::from_raw_parts_mut(self.elements, self.count) };
        &mut arr[idx]
    }

    fn dropElements(&mut self) {
        let arr      = unsafe { core::slice::from_raw_parts_mut(self.elements, self.count) };
        for i in 0..self.count {
            unsafe { ptr::drop_in_place(&arr[i] as *const T as *mut T) };
        }
    }

    pub fn toIter<'a>(&self) -> ::core::slice::Iter<'a, T> {
        let arr      = unsafe { core::slice::from_raw_parts(self.elements, self.count) };
        arr.into_iter()
    }

    pub fn last(&self) -> Option<&T> {
        if self.count == 0 {
            None
        } else {
            Some(&self[self.count - 1])
        }
    }
}

pub trait VecAppend<E: Copy> {
    fn append(&mut self, arr: &[E]);
}

impl<T : Copy> VecAppend<T> for Vec<T> {
    fn append(&mut self, arr: &[T]) {
        // TODO: optimize this
        for e in arr {
            self.pushBack(e.clone());
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output { self.get(idx) }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output { self.getMut(idx) }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        self.dropElements();
        unsafe { free(self.elements) }
    }
}

impl<T : Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut c = Vec::<T>::new();
        for i in 0..self.count {
            let v = self.get(i);
            c.pushBack(v.clone());
        }
        c
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testDestructor() {
        let mut v = Vec::<Vec<i32>>::new();
        for i in 0..100 {
            let  mut vj = Vec::<i32>::new();
            for j in 0..100 {
                vj.pushBack(j * i);
            }
            v.pushBack(vj);
        }
    }

    #[test]
    fn testIter() {
        let mut v = Vec::new();
        for i in 0..4 {
            v.pushBack(i);
            assert!(v[i] == i);
        }

        let mut counter = 0;
        for i in v.toIter() {
            if *i != counter { panic!("invalid {} != {}", i, counter) }
            counter += 1;
        }
    }
    #[test]
    fn testPopDestructor() {
        let mut v = Vec::<Vec<i32>>::new();
        for i in 0..100 {
            let  mut vj = Vec::<i32>::new();
            for j in 0..100 {
                vj.pushBack(j * i);
            }
            v.pushBack(vj);
        }

        assert!(v.len() == 100);
        for _ in 0..100 {
            v.pop();
        }
        assert!(v.len() == 0);
    }

    #[test]
    fn testPopDestructorPush() {
        let mut v = Vec::<Vec<i32>>::new();
        for i in 0..100 {
            let  mut vj = Vec::<i32>::new();
            for j in 0..100 {
                vj.pushBack(j * i);
            }
            v.pushBack(vj);
        }

        for _ in 0..100 {
            v.pop();
        }

        assert!(v.len() == 0);

        for i in 0..100 {
            let  mut vj = Vec::<i32>::new();
            for j in 0..100 {
                vj.pushBack(j * i);
            }
            v.pushBack(vj);
        }

        assert!(v.len() == 100);
    }
}