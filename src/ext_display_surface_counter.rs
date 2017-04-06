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

pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static [u8; 31] = b"VK_EXT_display_surface_counter\x00";
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME_STR: &'static str = "VK_EXT_display_surface_counter";

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSurfaceCounterFlagsEXT: u32 {
        const VK_SURFACE_COUNTER_VBLANK_EXT = 0x00000001,
    }
}
pub type VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagsEXT;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilities2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
    pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

impl Default for VkSurfaceCapabilities2EXT {
    fn default() -> Self {
        VkSurfaceCapabilities2EXT  {
            sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT,
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

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;
}
