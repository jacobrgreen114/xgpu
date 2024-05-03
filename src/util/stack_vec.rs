// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use std::ops::Deref;

pub struct StackVec<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T, const N: usize> StackVec<T, N> {
    const fn new() -> Self {
        Self {
            data: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    const fn len(&self) -> usize {
        self.len
    }

    const fn capacity(&self) -> usize {
        N
    }

    fn push(&mut self, value: T) {
        std::mem::forget(std::mem::replace(&mut self.data[self.len], value));
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(std::mem::replace(&mut self.data[self.len], unsafe {
            std::mem::MaybeUninit::uninit().assume_init()
        }))
    }

    fn as_slice(&self) -> &[T] {
        &self.data[0..self.len]
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[0..self.len]
    }

    fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                std::ptr::drop_in_place(&mut self.data[i]);
            }
        }
    }
}

impl<T, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Deref for StackVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for StackVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for StackVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> FromIterator<T> for StackVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack_vec = StackVec::default();
        for item in iter {
            stack_vec.push(item);
        }
        stack_vec
    }
}
