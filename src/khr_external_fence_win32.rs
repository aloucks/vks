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

//! [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)

use core;
use khr_external_fence;
use khr_external_fence_capabilities;
use libc::c_void;
use std::ptr;
use win32_types;

pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static [u8; 28] = b"VK_KHR_external_fence_win32\x00";
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_fence_win32";

/// See [`VkImportFenceWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportFenceWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub fence: core::VkFence,
    pub flags: khr_external_fence::VkFenceImportFlagsKHR,
    pub handleType: khr_external_fence_capabilities::VkExternalFenceHandleTypeFlagBitsKHR,
    pub handle: win32_types::HANDLE,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkImportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        VkImportFenceWin32HandleInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
            name: ptr::null(),
        }
    }
}

/// See [`VkExportFenceWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportFenceWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_types::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_types::DWORD,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkExportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        VkExportFenceWin32HandleInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

/// See [`VkFenceGetWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceGetWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub fence: core::VkFence,
    pub handleType: khr_external_fence_capabilities::VkExternalFenceHandleTypeFlagBitsKHR,
}

impl Default for VkFenceGetWin32HandleInfoKHR {
    fn default() -> Self {
        VkFenceGetWin32HandleInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`vkImportFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceWin32HandleKHR)
pub type PFN_vkImportFenceWin32HandleKHR = Option<unsafe extern "system" fn(device: core::VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> core::VkResult>;

/// See [`vkGetFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceWin32HandleKHR)
pub type PFN_vkGetFenceWin32HandleKHR = Option<unsafe extern "system" fn(device: core::VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkImportFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportFenceWin32HandleKHR)
    pub fn vkImportFenceWin32HandleKHR(device: core::VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> core::VkResult;

    /// See [`vkGetFenceWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceWin32HandleKHR)
    pub fn vkGetFenceWin32HandleKHR(device: core::VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> core::VkResult;
}
