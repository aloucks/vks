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

//! [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)

use core;
use khr_external_semaphore;
use khr_external_semaphore_capabilities;
use libc::{c_int, c_void};
use std::ptr;

pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static [u8; 29] = b"VK_KHR_external_semaphore_fd\x00";
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_semaphore_fd";

/// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreFdInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: core::VkSemaphore,
    pub flags: khr_external_semaphore::VkSemaphoreImportFlagsKHR,
    pub handleType: khr_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHR,
    pub fd: c_int,
}

impl Default for VkImportSemaphoreFdInfoKHR {
    fn default() -> Self {
        VkImportSemaphoreFdInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR,
            pNext: ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

/// See [`VkSemaphoreGetFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreGetFdInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: core::VkSemaphore,
    pub handleType: khr_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

impl Default for VkSemaphoreGetFdInfoKHR {
    fn default() -> Self {
        VkSemaphoreGetFdInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR,
            pNext: ptr::null(),
            semaphore: Default::default(),
            handleType: Default::default(),
        }
    }
}

/// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
pub type PFN_vkImportSemaphoreFdKHR = Option<unsafe extern "system" fn(device: core::VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> core::VkResult>;

/// See [`vkGetSemaphoreFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHR)
pub type PFN_vkGetSemaphoreFdKHR = Option<unsafe extern "system" fn(device: core::VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult>;

extern "system" {
    /// See [`VkImportSemaphoreFdInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHR)
    pub fn vkImportSemaphoreFdKHR(device: core::VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> core::VkResult;

    /// See [`vkGetSemaphoreFdKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHR)
    pub fn vkGetSemaphoreFdKHR(device: core::VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> core::VkResult;
}
