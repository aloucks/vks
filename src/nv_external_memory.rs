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
use core;
use libc::c_void;
use std::ptr;

pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static [u8; 22] = b"VK_NV_external_memory\x00";
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME_STR: &'static str = "VK_NV_external_memory";

/// See [`VkExternalMemoryImageCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryImageCreateInfoNV)
/// and extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoNV {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

impl Default for VkExternalMemoryImageCreateInfoNV {
    fn default() -> Self {
        VkExternalMemoryImageCreateInfoNV  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExportMemoryAllocateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryAllocateInfoNV)
/// and extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryAllocateInfoNV {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

impl Default for VkExportMemoryAllocateInfoNV {
    fn default() -> Self {
        VkExportMemoryAllocateInfoNV  {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
