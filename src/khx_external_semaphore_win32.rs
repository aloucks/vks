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

//! [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)

use ::*;
use core;
use khx_external_semaphore_capabilities;
use libc::c_void;
use std::ptr;

pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static [u8; 32] = b"VK_KHX_external_semaphore_win32\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_win32";

/// See [`VkImportSemaphoreWin32HandleInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHX)
/// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: core::VkSemaphore,
    pub handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagsKHX,
    pub handle: win32_wrapper::HANDLE,
}

impl Default for VkImportSemaphoreWin32HandleInfoKHX {
    fn default() -> Self {
        VkImportSemaphoreWin32HandleInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            semaphore: Default::default(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
        }
    }
}

/// See [`VkExportSemaphoreWin32HandleInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportSemaphoreWin32HandleInfoKHX)
/// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportSemaphoreWin32HandleInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_wrapper::DWORD,
    pub name: win32_wrapper::LPCWSTR,
}

impl Default for VkExportSemaphoreWin32HandleInfoKHX {
    fn default() -> Self {
        VkExportSemaphoreWin32HandleInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

/// See [`VkD3D12FenceSubmitInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkD3D12FenceSubmitInfoKHX)
/// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkD3D12FenceSubmitInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

impl Default for VkD3D12FenceSubmitInfoKHX {
    fn default() -> Self {
        VkD3D12FenceSubmitInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHX,
            pNext: ptr::null(),
            waitSemaphoreValuesCount: Default::default(),
            pWaitSemaphoreValues: ptr::null(),
            signalSemaphoreValuesCount: Default::default(),
            pSignalSemaphoreValues: ptr::null(),
        }
    }
}

/// See [`vkImportSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportSemaphoreWin32HandleKHX)
/// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
pub type PFN_vkImportSemaphoreWin32HandleKHX = unsafe extern "system" fn(device: core::VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHX) -> core::VkResult;

/// See [`vkGetSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHX)
/// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
pub type PFN_vkGetSemaphoreWin32HandleKHX = unsafe extern "system" fn(device: core::VkDevice, semaphore: core::VkSemaphore, handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkImportSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportSemaphoreWin32HandleKHX)
    /// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    pub fn vkImportSemaphoreWin32HandleKHX(device: core::VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHX) -> core::VkResult;

    /// See [`vkGetSemaphoreWin32HandleKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHX)
    /// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    pub fn vkGetSemaphoreWin32HandleKHX(device: core::VkDevice, semaphore: core::VkSemaphore, handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHX, pHandle: *mut win32_wrapper::HANDLE) -> core::VkResult;
}
