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

//! [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &'static [u8; 32] = b"VK_KHR_get_memory_requirements2\x00";
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME_STR: &'static str = "VK_KHR_get_memory_requirements2";

/// See [`VkBufferMemoryRequirementsInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferMemoryRequirementsInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferMemoryRequirementsInfo2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub buffer: vk::VkBuffer,
}

impl Default for VkBufferMemoryRequirementsInfo2KHR {
    fn default() -> Self {
        VkBufferMemoryRequirementsInfo2KHR {
            sType: vk::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR,
            pNext: ptr::null(),
            buffer: Default::default(),
        }
    }
}

/// See [`VkImageMemoryRequirementsInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageMemoryRequirementsInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageMemoryRequirementsInfo2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub image: vk::VkImage,
}

impl Default for VkImageMemoryRequirementsInfo2KHR {
    fn default() -> Self {
        VkImageMemoryRequirementsInfo2KHR {
            sType: vk::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR,
            pNext: ptr::null(),
            image: Default::default(),
        }
    }
}

/// See [`VkImageSparseMemoryRequirementsInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSparseMemoryRequirementsInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub image: vk::VkImage,
}

impl Default for VkImageSparseMemoryRequirementsInfo2KHR {
    fn default() -> Self {
        VkImageSparseMemoryRequirementsInfo2KHR {
            sType: vk::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR,
            pNext: ptr::null(),
            image: Default::default(),
        }
    }
}

/// See [`VkMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryRequirements2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryRequirements2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: vk::VkMemoryRequirements,
}

impl Default for VkMemoryRequirements2KHR {
    fn default() -> Self {
        VkMemoryRequirements2KHR {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR,
            pNext: ptr::null_mut(),
            memoryRequirements: Default::default(),
        }
    }
}

/// See [`VkSparseImageMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryRequirements2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageMemoryRequirements2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: vk::VkSparseImageMemoryRequirements,
}

impl Default for VkSparseImageMemoryRequirements2KHR {
    fn default() -> Self {
        VkSparseImageMemoryRequirements2KHR {
            sType: vk::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR,
            pNext: ptr::null_mut(),
            memoryRequirements: Default::default(),
        }
    }
}

/// See [`vkGetImageMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements2KHR)
pub type PFN_vkGetImageMemoryRequirements2KHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR)>;

/// See [`vkGetBufferMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements2KHR)
pub type PFN_vkGetBufferMemoryRequirements2KHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR)>;

/// See [`vkGetImageSparseMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements2KHR)
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetImageMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements2KHR)
    pub fn vkGetImageMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);

    /// See [`vkGetBufferMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements2KHR)
    pub fn vkGetBufferMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);

    /// See [`vkGetImageSparseMemoryRequirements2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements2KHR)
    pub fn vkGetImageSparseMemoryRequirements2KHR(device: vk::VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR);
}
