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

//! [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_sampler_filter_minmax\x00";
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME_STR: &str = "VK_EXT_sampler_filter_minmax";

vks_enum! {
    /// See [`VkSamplerReductionModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeEXT)
    pub VkSamplerReductionModeEXT: u32 {
        const WEIGHTED_AVERAGE_EXT = 0;
        const MIN_EXT = 1;
        const MAX_EXT = 2;
    }
}

/// See [`VkSamplerReductionModeCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeCreateInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSamplerReductionModeCreateInfoEXT {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub reductionMode: VkSamplerReductionModeEXT,
}

impl Default for VkSamplerReductionModeCreateInfoEXT {
    fn default() -> Self {
        VkSamplerReductionModeCreateInfoEXT {
            sType: vk::VkStructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
            pNext: ptr::null(),
            reductionMode: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub filterMinmaxSingleComponentFormats: vk::VkBool32,
    pub filterMinmaxImageComponentMapping: vk::VkBool32,
}

impl Default for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
            sType: vk::VkStructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            filterMinmaxSingleComponentFormats: Default::default(),
            filterMinmaxImageComponentMapping: Default::default(),
        }
    }
}
