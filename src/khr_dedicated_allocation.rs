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

//! [`VK_KHR_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_dedicated_allocation)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &[u8; 28] = b"VK_KHR_dedicated_allocation\x00";
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME_STR: &str = "VK_KHR_dedicated_allocation";

/// See [`VkMemoryDedicatedRequirementsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryDedicatedRequirementsKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryDedicatedRequirementsKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub prefersDedicatedAllocation: vk::VkBool32,
    pub requiresDedicatedAllocation: vk::VkBool32,
}

impl Default for VkMemoryDedicatedRequirementsKHR {
    fn default() -> Self {
        VkMemoryDedicatedRequirementsKHR {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR,
            pNext: ptr::null_mut(),
            prefersDedicatedAllocation: Default::default(),
            requiresDedicatedAllocation: Default::default(),
        }
    }
}

/// See [`VkMemoryDedicatedAllocateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryDedicatedAllocateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryDedicatedAllocateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub image: vk::VkImage,
    pub buffer: vk::VkBuffer,
}

impl Default for VkMemoryDedicatedAllocateInfoKHR {
    fn default() -> Self {
        VkMemoryDedicatedAllocateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR,
            pNext: ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
