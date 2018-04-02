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

//! [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)

use core::ptr;
use khr_external_memory_capabilities;
use libc::c_void;
use vk;
use win32_types;

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_external_memory_win32\x00";
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR: &str = "VK_KHR_external_memory_win32";

/// See [`VkImportMemoryWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR,
    pub handle: win32_types::HANDLE,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkImportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        VkImportMemoryWin32HandleInfoKHR {
            sType: vk::VkStructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
            name: ptr::null(),
        }
    }
}

/// See [`VkExportMemoryWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_types::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_types::DWORD,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkExportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        VkExportMemoryWin32HandleInfoKHR {
            sType: vk::VkStructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

/// See [`VkMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryWin32HandlePropertiesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

impl Default for VkMemoryWin32HandlePropertiesKHR {
    fn default() -> Self {
        VkMemoryWin32HandlePropertiesKHR {
            sType: vk::VkStructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            memoryTypeBits: Default::default(),
        }
    }
}

/// See [`VkMemoryGetWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryGetWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub memory: vk::VkDeviceMemory,
    pub handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR,
}

impl Default for VkMemoryGetWin32HandleInfoKHR {
    fn default() -> Self {
        VkMemoryGetWin32HandleInfoKHR {
            sType: vk::VkStructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            memory: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`vkGetMemoryWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHR)
pub type PFN_vkGetMemoryWin32HandleKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult>;

/// See [`vkGetMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHR)
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, handle: win32_types::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetMemoryWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHR)
    pub fn vkGetMemoryWin32HandleKHR(device: vk::VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult;

    /// See [`vkGetMemoryWin32HandlePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHR)
    pub fn vkGetMemoryWin32HandlePropertiesKHR(device: vk::VkDevice, handleType: khr_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsKHR, handle: win32_types::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> vk::VkResult;
}
