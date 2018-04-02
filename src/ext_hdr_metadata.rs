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

//! [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)

use core::ptr;
use khr_swapchain;
use libc::c_void;
use vk;

pub const VK_EXT_HDR_METADATA_SPEC_VERSION: u32 = 1;
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_hdr_metadata\x00";
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME_STR: &str = "VK_EXT_hdr_metadata";

/// See [`VkXYColorEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXYColorEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkXYColorEXT {
    pub x: f32,
    pub y: f32,
}

/// See [`VkHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkHdrMetadataEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkHdrMetadataEXT {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub displayPrimaryRed: VkXYColorEXT,
    pub displayPrimaryGreen: VkXYColorEXT,
    pub displayPrimaryBlue: VkXYColorEXT,
    pub whitePoint: VkXYColorEXT,
    pub maxLuminance: f32,
    pub minLuminance: f32,
    pub maxContentLightLevel: f32,
    pub maxFrameAverageLightLevel: f32,
}

impl Default for VkHdrMetadataEXT {
    fn default() -> Self {
        VkHdrMetadataEXT {
            sType: vk::VK_STRUCTURE_TYPE_HDR_METADATA_EXT,
            pNext: ptr::null(),
            displayPrimaryRed: Default::default(),
            displayPrimaryGreen: Default::default(),
            displayPrimaryBlue: Default::default(),
            whitePoint: Default::default(),
            maxLuminance: Default::default(),
            minLuminance: Default::default(),
            maxContentLightLevel: Default::default(),
            maxFrameAverageLightLevel: Default::default(),
        }
    }
}

/// See [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
pub type PFN_vkSetHdrMetadataEXT = Option<unsafe extern "system" fn(device: vk::VkDevice, swapchainCount: u32, pSwapchains: *const khr_swapchain::VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkSetHdrMetadataEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetHdrMetadataEXT)
    pub fn vkSetHdrMetadataEXT(device: vk::VkDevice, swapchainCount: u32, pSwapchains: *const khr_swapchain::VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);
}
