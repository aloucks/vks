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

use core;
use libc::c_void;
use std::ptr;

pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static [u8; 33] = b"VK_NV_fragment_coverage_to_color\x00";
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME_STR: &'static str = "VK_NV_fragment_coverage_to_color";

vks_bitflags! {
    /// See [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateFlagsNV)
    /// and extension [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkPipelineCoverageToColorStateCreateFlagsNV: u32 {
        /// See [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateFlagsNV)
        /// and extension [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)
        const VK_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_FLAG_BITS_MAX_ENUM_NV = 0x7fffffff;
    }
}

/// See [`VkPipelineCoverageToColorStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateFlagsNV)
/// and extension [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)
pub type VkPipelineCoverageToColorStateCreateFlagBitsNV = VkPipelineCoverageToColorStateCreateFlagsNV;

/// See [`VkPipelineCoverageToColorStateCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageToColorStateCreateInfoNV)
/// and extension [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable: core::VkBool32,
    pub coverageToColorLocation: u32,
}

impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineCoverageToColorStateCreateInfoNV {
            sType: core::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: Default::default(),
            coverageToColorEnable: Default::default(),
            coverageToColorLocation: Default::default(),
        }
    }
}
