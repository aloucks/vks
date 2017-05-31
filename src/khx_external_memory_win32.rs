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
use libc::c_void;
use std::ptr;

pub const VK_KHX_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static [u8; 29] = b"VK_KHX_external_memory_win32\x00";
pub const VK_KHX_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_memory_win32";

/// See [`VkImportMemoryWin32HandleInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryWin32HandleInfoKHX)
/// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHX,
    pub handle: win32_wrapper::HANDLE,
}

impl Default for VkImportMemoryWin32HandleInfoKHX {
    fn default() -> Self {
        VkImportMemoryWin32HandleInfoKHX  {
            sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
        }
    }
}

/// See [`VkExportMemoryWin32HandleInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryWin32HandleInfoKHX)
/// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
    pub name: win32_wrapper::LPCWSTR,
}

impl Default for VkExportMemoryWin32HandleInfoKHX {
    fn default() -> Self {
        VkExportMemoryWin32HandleInfoKHX  {
            sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

/// See [`VkMemoryWin32HandlePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryWin32HandlePropertiesKHX)
/// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

impl Default for VkMemoryWin32HandlePropertiesKHX {
    fn default() -> Self {
        VkMemoryWin32HandlePropertiesKHX  {
            sType: VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            memoryTypeBits: Default::default(),
        }
    }
}

/// See [`vkGetMemoryWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHX)
/// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
pub type PFN_vkGetMemoryWin32HandleKHX = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;

/// See [`vkGetMemoryWin32HandlePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHX)
/// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
pub type PFN_vkGetMemoryWin32HandlePropertiesKHX = unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, handle: win32_wrapper::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHX) -> VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkGetMemoryWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleKHX)
    /// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    pub fn vkGetMemoryWin32HandleKHX(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> VkResult;

    /// See [`vkGetMemoryWin32HandlePropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandlePropertiesKHX)
    /// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    pub fn vkGetMemoryWin32HandlePropertiesKHX(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBitsKHX, handle: win32_wrapper::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHX) -> VkResult;
}
