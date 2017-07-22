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

//! [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)

use core;
use experimental::khx_external_memory_capabilities;
use libc::{c_int, c_void};
use std::ptr;

pub const VK_KHX_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static [u8; 26] = b"VK_KHX_external_memory_fd\x00";
pub const VK_KHX_EXTERNAL_MEMORY_FD_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory_fd";

/// See [`VkImportMemoryFdInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryFdInfoKHX)
/// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryFdInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHX,
    pub fd: c_int,
}

impl Default for VkImportMemoryFdInfoKHX {
    fn default() -> Self {
        VkImportMemoryFdInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHX,
            pNext: ptr::null(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

/// See [`VkMemoryFdPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryFdPropertiesKHX)
/// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryFdPropertiesKHX {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

impl Default for VkMemoryFdPropertiesKHX {
    fn default() -> Self {
        VkMemoryFdPropertiesKHX  {
            sType: core::VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            memoryTypeBits: Default::default(),
        }
    }
}

/// See [`vkGetMemoryFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHX)
/// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
pub type PFN_vkGetMemoryFdKHX = unsafe extern "system" fn(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHX, pFd: *mut c_int) -> core::VkResult;

/// See [`vkGetMemoryFdPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHX)
/// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
pub type PFN_vkGetMemoryFdPropertiesKHX = unsafe extern "system" fn(device: core::VkDevice, handleType: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHX, fd: c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHX) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkGetMemoryFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdKHX)
    /// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
    pub fn vkGetMemoryFdKHX(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHX, pFd: *mut c_int) -> core::VkResult;

    /// See [`vkGetMemoryFdPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryFdPropertiesKHX)
    /// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
    pub fn vkGetMemoryFdPropertiesKHX(device: core::VkDevice, handleType: khx_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHX, fd: c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHX) -> core::VkResult;
}
