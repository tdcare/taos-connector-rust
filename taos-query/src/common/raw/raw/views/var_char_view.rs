use std::{fmt::Debug, ops::Deref, ffi::c_void};

use super::Offsets;
use crate::{util::InlineStr, common::{Ty, BorrowedValue}};

use bytes::Bytes;

#[derive(Debug)]
pub struct VarCharView {
    // version: Version,
    pub offsets: Offsets,
    pub data: Bytes,
}

impl VarCharView {
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    /// Check if the value at `row` index is NULL or not.
    pub fn is_null(&self, row: usize) -> bool {
        if row < self.len() {
            unsafe { self.is_null_unchecked(row) }
        } else {
            false
        }
    }

    /// Unsafe version for [methods.is_null]
    pub unsafe fn is_null_unchecked(&self, row: usize) -> bool {
        *self.offsets.get_unchecked(row) < 0
    }

    pub unsafe fn get_unchecked(&self, row: usize) -> Option<&InlineStr> {
        let offset = self.offsets.get_unchecked(row);
        if *offset >= 0 {
            Some(InlineStr::<u16>::from_ptr(
                self.data.as_ptr().offset(*offset as isize),
            ))
        } else {
            None
        }
    }

    pub unsafe fn get_value_unchecked(&self, row: usize) -> BorrowedValue {
        self.get_unchecked(row)
            .map(|s| BorrowedValue::VarChar(s.as_str()))
            .unwrap_or(BorrowedValue::Null)
    }

    pub unsafe fn get_raw_value_unchecked(&self, row: usize) -> (Ty, u32, *const c_void) {
        match self.get_unchecked(row) {
            Some(s) => (Ty::VarChar, s.len() as _, s.as_ptr() as _),
            None => (Ty::Null, 0, std::ptr::null()),
        }
    }

    pub fn iter(&self) -> VarCharIter {
        VarCharIter { view: self, row: 0 }
    }

    pub fn to_vec(&self) -> Vec<Option<String>> {
        (0..self.len())
            .map(|row| unsafe { self.get_unchecked(row) }.map(|s| s.to_string()))
            .collect()
    }
}

pub struct VarCharIter<'a> {
    view: &'a VarCharView,
    row: usize,
}

impl<'a> Iterator for VarCharIter<'a> {
    type Item = Option<&'a InlineStr>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row <= self.view.len() {
            let row = self.row;
            self.row += 1;
            Some(unsafe { self.view.get_unchecked(row) })
        } else {
            None
        }
    }
}
