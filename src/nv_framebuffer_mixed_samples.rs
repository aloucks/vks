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

//! [`VK_NV_framebuffer_mixed_samples`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_framebuffer_mixed_samples)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &[u8; 32] = b"VK_NV_framebuffer_mixed_samples\x00";
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME_STR: &str = "VK_NV_framebuffer_mixed_samples";

vks_enum! {
    /// See [`VkCoverageModulationModeNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCoverageModulationModeNV)
    pub VkCoverageModulationModeNV: u32 {
        const NONE_NV = 0;
        const RGB_NV = 1;
        const ALPHA_NV = 2;
        const RGBA_NV = 3;
    }
}

bitflags! {
    /// See [`VkPipelineCoverageModulationStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageModulationStateCreateFlagsNV)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineCoverageModulationStateCreateFlagsNV: u32 {
        const MAX_ENUM_NV = 0x7fffffff;
    }
}

/// See [`VkPipelineCoverageModulationStateCreateFlagsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageModulationStateCreateFlagsNV)
pub type VkPipelineCoverageModulationStateCreateFlagBitsNV = VkPipelineCoverageModulationStateCreateFlagsNV;

/// See [`VkPipelineCoverageModulationStateCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCoverageModulationStateCreateInfoNV)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
    pub coverageModulationMode: VkCoverageModulationModeNV,
    pub coverageModulationTableEnable: vk::VkBool32,
    pub coverageModulationTableCount: u32,
    pub pCoverageModulationTable: *const f32,
}

impl Default for VkPipelineCoverageModulationStateCreateInfoNV {
    fn default() -> Self {
        VkPipelineCoverageModulationStateCreateInfoNV {
            sType: vk::VkStructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: Default::default(),
            coverageModulationMode: Default::default(),
            coverageModulationTableEnable: Default::default(),
            coverageModulationTableCount: Default::default(),
            pCoverageModulationTable: ptr::null(),
        }
    }
}
