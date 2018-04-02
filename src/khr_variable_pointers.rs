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

//! [`VK_KHR_variable_pointers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_variable_pointers)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_variable_pointers\x00";
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME_STR: &str = "VK_KHR_variable_pointers";

/// See [`VkPhysicalDeviceVariablePointerFeaturesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceVariablePointerFeaturesKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub variablePointersStorageBuffer: vk::VkBool32,
    pub variablePointers: vk::VkBool32,
}

impl Default for VkPhysicalDeviceVariablePointerFeaturesKHR {
    fn default() -> Self {
        VkPhysicalDeviceVariablePointerFeaturesKHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR,
            pNext: ptr::null_mut(),
            variablePointersStorageBuffer: Default::default(),
            variablePointers: Default::default(),
        }
    }
}
