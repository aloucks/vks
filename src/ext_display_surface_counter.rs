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

//! [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)

use core::ptr;
use khr_surface;
use libc::c_void;
use vk;

pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &[u8; 31] = b"VK_EXT_display_surface_counter\x00";
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME_STR: &str = "VK_EXT_display_surface_counter";

bitflags! {
    /// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSurfaceCounterFlagsEXT: u32 {
        const MAX_ENUM_EXT = 0x7fffffff;
        const VBLANK_EXT = 0x00000001;
    }
}

/// See [`VkSurfaceCounterFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCounterFlagBitsEXT)
pub type VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagsEXT;

/// See [`VkSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilities2EXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilities2EXT {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: vk::VkExtent2D,
    pub minImageExtent: vk::VkExtent2D,
    pub maxImageExtent: vk::VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: khr_surface::VkSurfaceTransformFlagsKHR,
    pub currentTransform: khr_surface::VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: khr_surface::VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: vk::VkImageUsageFlags,
    pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

impl Default for VkSurfaceCapabilities2EXT {
    fn default() -> Self {
        VkSurfaceCapabilities2EXT {
            sType: vk::VkStructureType::SURFACE_CAPABILITIES_2_EXT,
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
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilities2EXT)
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: vk::VkPhysicalDevice, surface: khr_surface::VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> vk::VkResult;
}
