// Copyright (c) 2018, Dennis Hamester <dennis.hamester@startmail.com>
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

//! [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)

use core::ptr;
use libc::c_void;
use nv_external_memory_capabilities;
use vk;

pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &[u8; 22] = b"VK_NV_external_memory\x00";
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME_STR: &str = "VK_NV_external_memory";

/// See [`VkExternalMemoryImageCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryImageCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV,
}

impl Default for VkExternalMemoryImageCreateInfoNV {
    fn default() -> Self {
        VkExternalMemoryImageCreateInfoNV {
            sType: vk::VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExportMemoryAllocateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryAllocateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryAllocateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV,
}

impl Default for VkExportMemoryAllocateInfoNV {
    fn default() -> Self {
        VkExportMemoryAllocateInfoNV {
            sType: vk::VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
