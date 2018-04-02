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

//! [`VK_NV_viewport_swizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_viewport_swizzle)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &[u8; 23] = b"VK_NV_viewport_swizzle\x00";
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME_STR: &str = "VK_NV_viewport_swizzle";

vks_enum! {
    /// See [`VkViewportCoordinateSwizzleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViewportCoordinateSwizzleNV)
    pub VkViewportCoordinateSwizzleNV: u32 {
        const POSITIVE_X_NV = 0;
        const NEGATIVE_X_NV = 1;
        const POSITIVE_Y_NV = 2;
        const NEGATIVE_Y_NV = 3;
        const POSITIVE_Z_NV = 4;
        const NEGATIVE_Z_NV = 5;
        const POSITIVE_W_NV = 6;
        const NEGATIVE_W_NV = 7;
    }
}

bitflags! {
    /// See [`VkPipelineViewportSwizzleStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportSwizzleStateCreateFlagsNV)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineViewportSwizzleStateCreateFlagsNV: u32 {
        const MAX_ENUM_NV = 0x7fffffff;
    }
}

/// See [`VkPipelineViewportSwizzleStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportSwizzleStateCreateFlagsNV)
pub type VkPipelineViewportSwizzleStateCreateFlagBitsNV = VkPipelineViewportSwizzleStateCreateFlagsNV;

/// See [`VkViewportSwizzleNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViewportSwizzleNV)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkViewportSwizzleNV {
    pub x: VkViewportCoordinateSwizzleNV,
    pub y: VkViewportCoordinateSwizzleNV,
    pub z: VkViewportCoordinateSwizzleNV,
    pub w: VkViewportCoordinateSwizzleNV,
}

/// See [`VkPipelineViewportSwizzleStateCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportSwizzleStateCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
    pub viewportCount: u32,
    pub pViewportSwizzles: *const VkViewportSwizzleNV,
}

impl Default for VkPipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineViewportSwizzleStateCreateInfoNV {
            sType: vk::VkStructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: Default::default(),
            viewportCount: Default::default(),
            pViewportSwizzles: ptr::null(),
        }
    }
}
