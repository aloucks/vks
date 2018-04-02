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

//! [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &[u8; 27] = b"VK_NV_dedicated_allocation\x00";
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME_STR: &str = "VK_NV_dedicated_allocation";

/// See [`VkDedicatedAllocationImageCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationImageCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: vk::VkBool32,
}

impl Default for VkDedicatedAllocationImageCreateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationImageCreateInfoNV {
            sType: vk::VkStructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            pNext: ptr::null(),
            dedicatedAllocation: Default::default(),
        }
    }
}

/// See [`VkDedicatedAllocationBufferCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationBufferCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: vk::VkBool32,
}

impl Default for VkDedicatedAllocationBufferCreateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationBufferCreateInfoNV {
            sType: vk::VkStructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            pNext: ptr::null(),
            dedicatedAllocation: Default::default(),
        }
    }
}

/// See [`VkDedicatedAllocationMemoryAllocateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationMemoryAllocateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub image: vk::VkImage,
    pub buffer: vk::VkBuffer,
}

impl Default for VkDedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> Self {
        VkDedicatedAllocationMemoryAllocateInfoNV {
            sType: vk::VkStructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            pNext: ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
