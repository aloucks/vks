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

//! [`VK_KHR_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory)

use core;
use khr_external_memory_capabilities;
use libc::c_void;
use std::ptr;

pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_external_memory\x00";
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_memory";

pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = 0xfffffffe;

/// See [`VkExternalMemoryImageCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryImageCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHR,
}

impl Default for VkExternalMemoryImageCreateInfoKHR {
    fn default() -> Self {
        VkExternalMemoryImageCreateInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExternalMemoryBufferCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryBufferCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryBufferCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHR,
}

impl Default for VkExternalMemoryBufferCreateInfoKHR {
    fn default() -> Self {
        VkExternalMemoryBufferCreateInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}

/// See [`VkExportMemoryAllocateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryAllocateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryAllocateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsKHR,
}

impl Default for VkExportMemoryAllocateInfoKHR {
    fn default() -> Self {
        VkExportMemoryAllocateInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
