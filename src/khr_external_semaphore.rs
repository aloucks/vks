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

//! [`VK_KHR_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore)

use core::ptr;
use khr_external_semaphore_capabilities;
use libc::c_void;
use vk;

pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &[u8; 26] = b"VK_KHR_external_semaphore\x00";
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME_STR: &str = "VK_KHR_external_semaphore";

vks_bitflags! {
    /// See [`VkSemaphoreImportFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreImportFlagBitsKHR)
    pub struct VkSemaphoreImportFlagsKHR: u32 {
        /// See [`VkSemaphoreImportFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreImportFlagBitsKHR)
        const VK_SEMAPHORE_IMPORT_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        /// See [`VkSemaphoreImportFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreImportFlagBitsKHR)
        const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR = 0x00000001;
    }
}

/// See [`VkSemaphoreImportFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreImportFlagBitsKHR)
pub type VkSemaphoreImportFlagBitsKHR = VkSemaphoreImportFlagsKHR;

/// See [`VkExportSemaphoreCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportSemaphoreCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportSemaphoreCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khr_external_semaphore_capabilities::VkExternalSemaphoreHandleTypeFlagsKHR,
}

impl Default for VkExportSemaphoreCreateInfoKHR {
    fn default() -> Self {
        VkExportSemaphoreCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
