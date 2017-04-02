// Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

use std::marker;
use std::mem;

#[repr(C)]
pub struct VkSysUnionField<T>(marker::PhantomData<T>);

impl <T> VkSysUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        VkSysUnionField(marker::PhantomData)
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        mem::transmute(self)
    }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        mem::transmute(self)
    }
}

impl<T> Copy for VkSysUnionField<T> { }

impl<T> Clone for VkSysUnionField<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Default for VkSysUnionField<T> {
    fn default() -> Self {
        Self::new()
    }
}
