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

pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static [u8; 26] = b"VK_EXT_discard_rectangles\x00";
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME_STR: &'static str = "VK_EXT_discard_rectangles";

cenum!(VkDiscardRectangleModeEXT: u32 {
    const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
    const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
});

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineDiscardRectangleStateCreateFlagsEXT: u32 {
        const VK_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_DUMMY_EXT = 0x00000000,
    }
}
pub type VkPipelineDiscardRectangleStateCreateFlagBitsEXT = VkPipelineDiscardRectangleStateCreateFlagsEXT;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDiscardRectangles: u32,
}

impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceDiscardRectanglePropertiesEXT  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            maxDiscardRectangles: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode: VkDiscardRectangleModeEXT,
    pub discardRectangleCount: u32,
    pub pDiscardRectangles: *const VkRect2D,
}

impl Default for VkPipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> Self {
        VkPipelineDiscardRectangleStateCreateInfoEXT  {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            discardRectangleMode: Default::default(),
            discardRectangleCount: Default::default(),
            pDiscardRectangles: ptr::null(),
        }
    }
}

pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);
}
