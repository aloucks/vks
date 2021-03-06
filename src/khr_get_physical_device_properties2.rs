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

//! [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &'static [u8; 39] = b"VK_KHR_get_physical_device_properties2\x00";
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME_STR: &'static str = "VK_KHR_get_physical_device_properties2";

/// See [`VkPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceFeatures2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub features: vk::VkPhysicalDeviceFeatures,
}

impl Default for VkPhysicalDeviceFeatures2KHR {
    fn default() -> Self {
        VkPhysicalDeviceFeatures2KHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
            pNext: ptr::null_mut(),
            features: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub properties: vk::VkPhysicalDeviceProperties,
}

impl Default for VkPhysicalDeviceProperties2KHR {
    fn default() -> Self {
        VkPhysicalDeviceProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            properties: Default::default(),
        }
    }
}

/// See [`VkFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFormatProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: vk::VkFormatProperties,
}

impl Default for VkFormatProperties2KHR {
    fn default() -> Self {
        VkFormatProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            formatProperties: Default::default(),
        }
    }
}

/// See [`VkImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageFormatProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageFormatProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: vk::VkImageFormatProperties,
}

impl Default for VkImageFormatProperties2KHR {
    fn default() -> Self {
        VkImageFormatProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            imageFormatProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceImageFormatInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceImageFormatInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub format: vk::VkFormat,
    pub type_: vk::VkImageType,
    pub tiling: vk::VkImageTiling,
    pub usage: vk::VkImageUsageFlags,
    pub flags: vk::VkImageCreateFlags,
}

impl Default for VkPhysicalDeviceImageFormatInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceImageFormatInfo2KHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}

/// See [`VkQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFamilyProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: vk::VkQueueFamilyProperties,
}

impl Default for VkQueueFamilyProperties2KHR {
    fn default() -> Self {
        VkQueueFamilyProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            queueFamilyProperties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMemoryProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: vk::VkPhysicalDeviceMemoryProperties,
}

impl Default for VkPhysicalDeviceMemoryProperties2KHR {
    fn default() -> Self {
        VkPhysicalDeviceMemoryProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            memoryProperties: Default::default(),
        }
    }
}

/// See [`VkSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatProperties2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub properties: vk::VkSparseImageFormatProperties,
}

impl Default for VkSparseImageFormatProperties2KHR {
    fn default() -> Self {
        VkSparseImageFormatProperties2KHR {
            sType: vk::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            properties: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceSparseImageFormatInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSparseImageFormatInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub format: vk::VkFormat,
    pub type_: vk::VkImageType,
    pub samples: vk::VkSampleCountFlagBits,
    pub usage: vk::VkImageUsageFlags,
    pub tiling: vk::VkImageTiling,
}

impl Default for VkPhysicalDeviceSparseImageFormatInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceSparseImageFormatInfo2KHR {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures2KHR)
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR)>;

/// See [`vkGetPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties2KHR)
pub type PFN_vkGetPhysicalDeviceProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR)>;

/// See [`vkGetPhysicalDeviceFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties2KHR)
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, format: vk::VkFormat, pFormatProperties: *mut VkFormatProperties2KHR)>;

/// See [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties2KHR)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties2KHR)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR)>;

/// See [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties2KHR)
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR)>;

/// See [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures2KHR)
    pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: vk::VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);

    /// See [`vkGetPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties2KHR)
    pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: vk::VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);

    /// See [`vkGetPhysicalDeviceFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties2KHR)
    pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: vk::VkPhysicalDevice, format: vk::VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);

    /// See [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties2KHR)
    pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: vk::VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties2KHR)
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: vk::VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);

    /// See [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties2KHR)
    pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: vk::VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);

    /// See [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: vk::VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);
}
