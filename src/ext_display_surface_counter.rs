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

//! [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)

use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static [u8; 31] = b"VK_EXT_display_surface_counter\x00";
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME_STR: &'static str = "VK_EXT_display_surface_counter";

vks_bitflags! {
    /// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
    /// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkSurfaceCounterFlagsEXT: u32 {
        /// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
        /// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
        const VK_SURFACE_COUNTER_FLAG_BITS_MAX_ENUM_EXT = 0x7fffffff;

        /// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
        /// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
        const VK_SURFACE_COUNTER_VBLANK_EXT = 0x00000001;
    }
}

/// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
/// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
pub type VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagsEXT;

/// See [`VkSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilities2EXT)
/// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilities2EXT {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: core::VkExtent2D,
    pub minImageExtent: core::VkExtent2D,
    pub maxImageExtent: core::VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: khr_surface::VkSurfaceTransformFlagsKHR,
    pub currentTransform: khr_surface::VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: khr_surface::VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: core::VkImageUsageFlags,
    pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

impl Default for VkSurfaceCapabilities2EXT {
    fn default() -> Self {
        VkSurfaceCapabilities2EXT  {
            sType: core::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT,
            pNext: ptr::null_mut(),
            minImageCount: Default::default(),
            maxImageCount: Default::default(),
            currentExtent: Default::default(),
            minImageExtent: Default::default(),
            maxImageExtent: Default::default(),
            maxImageArrayLayers: Default::default(),
            supportedTransforms: Default::default(),
            currentTransform: Default::default(),
            supportedCompositeAlpha: Default::default(),
            supportedUsageFlags: Default::default(),
            supportedSurfaceCounters: Default::default(),
        }
    }
}

/// See [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2EXT)
/// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> core::VkResult;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2EXT)
    /// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: core::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> core::VkResult;
}
