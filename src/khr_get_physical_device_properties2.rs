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

use ::*;
use libc::c_void;
use std::ptr;

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &'static [u8; 39] = b"VK_KHR_get_physical_device_properties2\x00";
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME_STR: &'static str = "VK_KHR_get_physical_device_properties2";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}

impl Default for VkPhysicalDeviceFeatures2KHR {
    fn default() -> Self {
        VkPhysicalDeviceFeatures2KHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
            pNext: ptr::null_mut(),
            features: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}

impl Default for VkPhysicalDeviceProperties2KHR {
    fn default() -> Self {
        VkPhysicalDeviceProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            properties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties,
}

impl Default for VkFormatProperties2KHR {
    fn default() -> Self {
        VkFormatProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            formatProperties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

impl Default for VkImageFormatProperties2KHR {
    fn default() -> Self {
        VkImageFormatProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            imageFormatProperties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub type_: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}

impl Default for VkPhysicalDeviceImageFormatInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceImageFormatInfo2KHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

impl Default for VkQueueFamilyProperties2KHR {
    fn default() -> Self {
        VkQueueFamilyProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            queueFamilyProperties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

impl Default for VkPhysicalDeviceMemoryProperties2KHR {
    fn default() -> Self {
        VkPhysicalDeviceMemoryProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            memoryProperties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties,
}

impl Default for VkSparseImageFormatProperties2KHR {
    fn default() -> Self {
        VkSparseImageFormatProperties2KHR  {
            sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            properties: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub type_: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

impl Default for VkPhysicalDeviceSparseImageFormatInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceSparseImageFormatInfo2KHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}

pub type PFN_vkGetPhysicalDeviceFeatures2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
pub type PFN_vkGetPhysicalDeviceProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
    pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
    pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
    pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);
    pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);
}
