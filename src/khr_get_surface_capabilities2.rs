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

//! [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)

use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static [u8; 33] = b"VK_KHR_get_surface_capabilities2\x00";
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME_STR: &'static str = "VK_KHR_get_surface_capabilities2";

/// See [`VkPhysicalDeviceSurfaceInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSurfaceInfo2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub surface: khr_surface::VkSurfaceKHR,
}

impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceSurfaceInfo2KHR  {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            pNext: ptr::null(),
            surface: Default::default(),
        }
    }
}

/// See [`VkSurfaceCapabilities2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilities2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilities2KHR {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceCapabilities: khr_surface::VkSurfaceCapabilitiesKHR,
}

impl Default for VkSurfaceCapabilities2KHR {
    fn default() -> Self {
        VkSurfaceCapabilities2KHR  {
            sType: core::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
            pNext: ptr::null_mut(),
            surfaceCapabilities: Default::default(),
        }
    }
}

/// See [`VkSurfaceFormat2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceFormat2KHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFormat2KHR {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceFormat: khr_surface::VkSurfaceFormatKHR,
}

impl Default for VkSurfaceFormat2KHR {
    fn default() -> Self {
        VkSurfaceFormat2KHR  {
            sType: core::VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR,
            pNext: ptr::null_mut(),
            surfaceFormat: Default::default(),
        }
    }
}

/// See [`PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR) -> core::VkResult>;

/// See [`PFN_vkGetPhysicalDeviceSurfaceFormats2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> core::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)
    pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice: core::VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> core::VkResult;
}
