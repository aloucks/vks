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

//! [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)

use core;
use experimental::khx_external_memory_capabilities;
use libc::c_void;
use std::ptr;

pub const VK_KHX_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHX_external_memory\x00";
pub const VK_KHX_EXTERNAL_MEMORY_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory";
pub const VK_QUEUE_FAMILY_EXTERNAL_KHX: u32 = 0xfffffffe;

/// See [`VkExternalMemoryImageCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryImageCreateInfoKHX)
/// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHX,
}

impl Default for VkExternalMemoryImageCreateInfoKHX {
    fn default() -> Self {
        VkExternalMemoryImageCreateInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHX,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExternalMemoryBufferCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryBufferCreateInfoKHX)
/// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryBufferCreateInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHX,
}

impl Default for VkExternalMemoryBufferCreateInfoKHX {
    fn default() -> Self {
        VkExternalMemoryBufferCreateInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHX,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExportMemoryAllocateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryAllocateInfoKHX)
/// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryAllocateInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHX,
}

impl Default for VkExportMemoryAllocateInfoKHX {
    fn default() -> Self {
        VkExportMemoryAllocateInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHX,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
