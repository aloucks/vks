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

//! [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static [u8; 27] = b"VK_NV_clip_space_w_scaling\x00";
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME_STR: &'static str = "VK_NV_clip_space_w_scaling";

/// See [`VkViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViewportWScalingNV)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}

/// See [`VkPipelineViewportWScalingStateCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportWScalingStateCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub viewportWScalingEnable: vk::VkBool32,
    pub viewportCount: u32,
    pub pViewportWScalings: *const VkViewportWScalingNV,
}

impl Default for VkPipelineViewportWScalingStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineViewportWScalingStateCreateInfoNV {
            sType: vk::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            viewportWScalingEnable: Default::default(),
            viewportCount: Default::default(),
            pViewportWScalings: ptr::null(),
        }
    }
}

/// See [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
pub type PFN_vkCmdSetViewportWScalingNV = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCmdSetViewportWScalingNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewportWScalingNV)
    pub fn vkCmdSetViewportWScalingNV(commandBuffer: vk::VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);
}
