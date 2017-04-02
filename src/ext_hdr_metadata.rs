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

pub const VK_EXT_HDR_METADATA_SPEC_VERSION: u32 = 0;
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &'static [u8; 20] = b"VK_EXT_hdr_metadata\x00";
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME_STR: &'static str = "VK_EXT_hdr_metadata";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXYColorEXT {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkHdrMetadataEXT {
    pub sType: VkStructureType,
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

pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(device: VkDevice, swapchainCount: u32, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkSetHdrMetadataEXT(device: VkDevice, swapchainCount: u32, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);
}
