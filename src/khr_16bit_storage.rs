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

//! [`VK_KHR_16bit_storage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_16bit_storage)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &[u8; 21] = b"VK_KHR_16bit_storage\x00";
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME_STR: &str = "VK_KHR_16bit_storage";

/// See [`VkPhysicalDevice16BitStorageFeaturesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice16BitStorageFeaturesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: vk::VkBool32,
    pub uniformAndStorageBuffer16BitAccess: vk::VkBool32,
    pub storagePushConstant16: vk::VkBool32,
    pub storageInputOutput16: vk::VkBool32,
}

impl Default for VkPhysicalDevice16BitStorageFeaturesKHR {
    fn default() -> Self {
        VkPhysicalDevice16BitStorageFeaturesKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR,
            pNext: ptr::null_mut(),
            storageBuffer16BitAccess: Default::default(),
            uniformAndStorageBuffer16BitAccess: Default::default(),
            storagePushConstant16: Default::default(),
            storageInputOutput16: Default::default(),
        }
    }
}
