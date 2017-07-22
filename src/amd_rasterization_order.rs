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

//! [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)

use core;
use libc::c_void;
use std::ptr;

pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static [u8; 27] = b"VK_AMD_rasterization_order\x00";
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME_STR: &'static str = "VK_AMD_rasterization_order";

cenum!(VkRasterizationOrderAMD: u32 {
    /// See [`VkRasterizationOrderAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRasterizationOrderAMD)
    /// and extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)
    const VK_RASTERIZATION_ORDER_STRICT_AMD = 0,

    /// See [`VkRasterizationOrderAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRasterizationOrderAMD)
    /// and extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)
    const VK_RASTERIZATION_ORDER_RELAXED_AMD = 1,
});

/// See [`VkPipelineRasterizationStateRasterizationOrderAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateRasterizationOrderAMD)
/// and extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub rasterizationOrder: VkRasterizationOrderAMD,
}

impl Default for VkPipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> Self {
        VkPipelineRasterizationStateRasterizationOrderAMD  {
            sType: core::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            pNext: ptr::null(),
            rasterizationOrder: Default::default(),
        }
    }
}
