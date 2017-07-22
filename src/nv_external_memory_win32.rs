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
use core;
use libc::c_void;
use std::ptr;

pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static [u8; 28] = b"VK_NV_external_memory_win32\x00";
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR: &'static str = "VK_NV_external_memory_win32";

/// See [`VkImportMemoryWin32HandleInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryWin32HandleInfoNV)
/// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: win32_wrapper::HANDLE,
}

impl Default for VkImportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        VkImportMemoryWin32HandleInfoNV  {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            pNext: ptr::null(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
        }
    }
}

/// See [`VkExportMemoryWin32HandleInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryWin32HandleInfoNV)
/// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
}

impl Default for VkExportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        VkExportMemoryWin32HandleInfoNV  {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
        }
    }
}

/// See [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
/// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_wrapper::HANDLE) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkGetMemoryWin32HandleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetMemoryWin32HandleNV)
    /// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    pub fn vkGetMemoryWin32HandleNV(device: core::VkDevice, memory: core::VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut win32_wrapper::HANDLE) -> core::VkResult;
}
