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

//! [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &[u8; 33] = b"VK_NV_fragment_coverage_to_color\x00";
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME_STR: &str = "VK_NV_fragment_coverage_to_color";

bitflags! {
    /// See [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateFlagsNV)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineCoverageToColorStateCreateFlagsNV: u32 {
        const MAX_ENUM_NV = 0x7fffffff;
    }
}

/// See [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateFlagsNV)
pub type VkPipelineCoverageToColorStateCreateFlagBitsNV = VkPipelineCoverageToColorStateCreateFlagsNV;

/// See [`VkPipelineCoverageToColorStateCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable: vk::VkBool32,
    pub coverageToColorLocation: u32,
}

impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineCoverageToColorStateCreateInfoNV {
            sType: vk::VkStructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: Default::default(),
            coverageToColorEnable: Default::default(),
            coverageToColorLocation: Default::default(),
        }
    }
}
