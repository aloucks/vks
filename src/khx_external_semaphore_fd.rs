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

//! [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)

use core;
use khx_external_semaphore_capabilities;
use libc::{c_int, c_void};
use std::ptr;

pub const VK_KHX_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHX_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static [u8; 29] = b"VK_KHX_external_semaphore_fd\x00";
pub const VK_KHX_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME_STR: &'static str = "VK_KHX_external_semaphore_fd";

/// See [`VkImportSemaphoreFdInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportSemaphoreFdInfoKHX)
/// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreFdInfoKHX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: core::VkSemaphore,
    pub handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHX,
    pub fd: c_int,
}

impl Default for VkImportSemaphoreFdInfoKHX {
    fn default() -> Self {
        VkImportSemaphoreFdInfoKHX  {
            sType: core::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHX,
            pNext: ptr::null(),
            semaphore: Default::default(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

/// See [`vkImportSemaphoreFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportSemaphoreFdKHX)
/// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
pub type PFN_vkImportSemaphoreFdKHX = unsafe extern "system" fn(device: core::VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHX) -> core::VkResult;

/// See [`vkGetSemaphoreFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHX)
/// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
pub type PFN_vkGetSemaphoreFdKHX = unsafe extern "system" fn(device: core::VkDevice, semaphore: core::VkSemaphore, handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHX, pFd: *mut c_int) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkImportSemaphoreFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkImportSemaphoreFdKHX)
    /// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
    pub fn vkImportSemaphoreFdKHX(device: core::VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHX) -> core::VkResult;

    /// See [`vkGetSemaphoreFdKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSemaphoreFdKHX)
    /// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
    pub fn vkGetSemaphoreFdKHX(device: core::VkDevice, semaphore: core::VkSemaphore, handleType: khx_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagBitsKHX, pFd: *mut c_int) -> core::VkResult;
}
