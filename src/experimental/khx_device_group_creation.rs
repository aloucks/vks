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

//! [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)

use core::mem;
use core::ptr;
use experimental::khx_device_group;
use libc::c_void;
use vk;

pub const VK_KHX_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
pub const VK_KHX_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static [u8; 29] = b"VK_KHX_device_group_creation\x00";
pub const VK_KHX_DEVICE_GROUP_CREATION_EXTENSION_NAME_STR: &'static str = "VK_KHX_device_group_creation";

/// See [`VkPhysicalDeviceGroupPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceGroupPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceGroupPropertiesKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [vk::VkPhysicalDevice; khx_device_group::VK_MAX_DEVICE_GROUP_SIZE_KHX],
    pub subsetAllocation: vk::VkBool32,
}

impl Default for VkPhysicalDeviceGroupPropertiesKHX {
    fn default() -> Self {
        VkPhysicalDeviceGroupPropertiesKHX {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            physicalDeviceCount: Default::default(),
            physicalDevices: unsafe { mem::zeroed() },
            subsetAllocation: Default::default(),
        }
    }
}

/// See [`VkDeviceGroupDeviceCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGroupDeviceCreateInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGroupDeviceCreateInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const vk::VkPhysicalDevice,
}

impl Default for VkDeviceGroupDeviceCreateInfoKHX {
    fn default() -> Self {
        VkDeviceGroupDeviceCreateInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX,
            pNext: ptr::null(),
            physicalDeviceCount: Default::default(),
            pPhysicalDevices: ptr::null(),
        }
    }
}

/// See [`vkEnumeratePhysicalDeviceGroupsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDeviceGroupsKHX)
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHX = Option<unsafe extern "system" fn(instance: vk::VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHX) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkEnumeratePhysicalDeviceGroupsKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDeviceGroupsKHX)
    pub fn vkEnumeratePhysicalDeviceGroupsKHX(instance: vk::VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHX) -> vk::VkResult;
}
