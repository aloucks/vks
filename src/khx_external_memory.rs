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

use ::*;
use libc::c_void;

pub const VK_KHX_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHX_external_memory\x00";
pub const VK_KHX_EXTERNAL_MEMORY_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory";
pub const VK_QUEUE_FAMILY_EXTERNAL_KHX: u32 = 0xfffffffe;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryBufferCreateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryAllocateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHX,
}
