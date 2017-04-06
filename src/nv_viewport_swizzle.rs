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

pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static [u8; 23] = b"VK_NV_viewport_swizzle\x00";
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME_STR: &'static str = "VK_NV_viewport_swizzle";

cenum!(VkViewportCoordinateSwizzleNV: u32 {
    const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6,
    const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7,
});

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineViewportSwizzleStateCreateFlagsNV: u32 {
        const VK_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_DUMMY_NV = 0x00000000,
    }
}
pub type VkPipelineViewportSwizzleStateCreateFlagBitsNV = VkPipelineViewportSwizzleStateCreateFlagsNV;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkViewportSwizzleNV {
    pub x: VkViewportCoordinateSwizzleNV,
    pub y: VkViewportCoordinateSwizzleNV,
    pub z: VkViewportCoordinateSwizzleNV,
    pub w: VkViewportCoordinateSwizzleNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
    pub viewportCount: u32,
    pub pViewportSwizzles: *const VkViewportSwizzleNV,
}

impl Default for VkPipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineViewportSwizzleStateCreateInfoNV  {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: Default::default(),
            viewportCount: Default::default(),
            pViewportSwizzles: ptr::null(),
        }
    }
}
