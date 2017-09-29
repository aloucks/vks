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

//! [`VK_KHR_external_fence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence)

use core::ptr;
use khr_external_fence_capabilities;
use libc::c_void;
use vk;

pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &'static [u8; 22] = b"VK_KHR_external_fence\x00";
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME_STR: &'static str = "VK_KHR_external_fence";

vks_bitflags! {
    /// See [`VkFenceImportFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceImportFlagsKHR)
    pub struct VkFenceImportFlagsKHR: u32 {
        /// See [`VkFenceImportFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceImportFlagsKHR)
        const VK_FENCE_IMPORT_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;

        /// See [`VkFenceImportFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceImportFlagsKHR)
        const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR = 0x00000001;
    }
}

/// See [`VkFenceImportFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceImportFlagsKHR)
pub type VkFenceImportFlagBitsKHR = VkFenceImportFlagsKHR;

/// See [`VkExportFenceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportFenceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportFenceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: khr_external_fence_capabilities::VkExternalFenceHandleTypeFlagsKHR,
}

impl Default for VkExportFenceCreateInfoKHR {
    fn default() -> Self {
        VkExportFenceCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            handleTypes: Default::default(),
        }
    }
}
