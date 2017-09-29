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

//! [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)

use khr_external_memory_capabilities;
use libc::{c_int, c_void};
use std::ptr;
use vk;

pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static [u8; 26] = b"VK_KHR_external_memory_fd\x00";
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_memory_fd";

/// See [`VkImportMemoryFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryFdInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR,
    pub fd: c_int,
}

impl Default for VkImportMemoryFdInfoKHR {
    fn default() -> Self {
        VkImportMemoryFdInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR,
            pNext: ptr::null(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

/// See [`VkMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryFdPropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryFdPropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

impl Default for VkMemoryFdPropertiesKHR {
    fn default() -> Self {
        VkMemoryFdPropertiesKHR {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            memoryTypeBits: Default::default(),
        }
    }
}

/// See [`VkMemoryGetFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryGetFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub memory: vk::VkDeviceMemory,
    pub handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR,
}

impl Default for VkMemoryGetFdInfoKHR {
    fn default() -> Self {
        VkMemoryGetFdInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR,
            pNext: ptr::null(),
            memory: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`vkGetMemoryFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHR)
pub type PFN_vkGetMemoryFdKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult>;

/// See [`vkGetMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHR)
pub type PFN_vkGetMemoryFdPropertiesKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, fd: c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetMemoryFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHR)
    pub fn vkGetMemoryFdKHR(device: vk::VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> vk::VkResult;

    /// See [`vkGetMemoryFdPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHR)
    pub fn vkGetMemoryFdPropertiesKHR(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, fd: c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> vk::VkResult;
}
