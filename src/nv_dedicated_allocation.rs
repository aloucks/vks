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
use std::ptr;

pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static [u8; 27] = b"VK_NV_dedicated_allocation\x00";
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME_STR: &'static str = "VK_NV_dedicated_allocation";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

impl Default for VkDedicatedAllocationImageCreateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationImageCreateInfoNV  {
            sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            pNext: ptr::null(),
            dedicatedAllocation: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

impl Default for VkDedicatedAllocationBufferCreateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationBufferCreateInfoNV  {
            sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            pNext: ptr::null(),
            dedicatedAllocation: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
}

impl Default for VkDedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationMemoryAllocateInfoNV  {
            sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            pNext: ptr::null(),
            image: ptr::null_mut(),
            buffer: ptr::null_mut(),
        }
    }
}
