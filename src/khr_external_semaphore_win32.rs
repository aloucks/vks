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

//! [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)

use core::ptr;
use khr_external_semaphore;
use khr_external_semaphore_capabilities;
use libc::c_void;
use vk;
use win32_types;

pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_external_semaphore_win32\x00";
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME_STR: &str = "VK_KHR_external_semaphore_win32";

/// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: vk::VkSemaphore,
    pub flags: khr_external_semaphore::VkSemaphoreImportFlagsKHR,
    pub handleType: khr_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHR,
    pub handle: win32_types::HANDLE,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkImportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        VkImportSemaphoreWin32HandleInfoKHR {
            sType: vk::VkStructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            handle: ptr::null_mut(),
            name: ptr::null(),
        }
    }
}

/// See [`VkExportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportSemaphoreWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const win32_types::SECURITY_ATTRIBUTES,
    pub dwAccess: win32_types::DWORD,
    pub name: win32_types::LPCWSTR,
}

impl Default for VkExportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        VkExportSemaphoreWin32HandleInfoKHR {
            sType: vk::VkStructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

/// See [`VkD3D12FenceSubmitInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkD3D12FenceSubmitInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

impl Default for VkD3D12FenceSubmitInfoKHR {
    fn default() -> Self {
        VkD3D12FenceSubmitInfoKHR {
            sType: vk::VkStructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreValuesCount: Default::default(),
            pWaitSemaphoreValues: ptr::null(),
            signalSemaphoreValuesCount: Default::default(),
            pSignalSemaphoreValues: ptr::null(),
        }
    }
}

/// See [`VkSemaphoreGetWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreGetWin32HandleInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: vk::VkSemaphore,
    pub handleType: khr_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

impl Default for VkSemaphoreGetWin32HandleInfoKHR {
    fn default() -> Self {
        VkSemaphoreGetWin32HandleInfoKHR {
            sType: vk::VkStructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            semaphore: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> vk::VkResult>;

/// See [`vkGetSemaphoreWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHR)
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`VkImportSemaphoreWin32HandleInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreWin32HandleInfoKHR)
    pub fn vkImportSemaphoreWin32HandleKHR(device: vk::VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> vk::VkResult;

    /// See [`vkGetSemaphoreWin32HandleKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreWin32HandleKHR)
    pub fn vkGetSemaphoreWin32HandleKHR(device: vk::VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut win32_types::HANDLE) -> vk::VkResult;
}
