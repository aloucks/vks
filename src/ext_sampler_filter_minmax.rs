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

pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static [u8; 29] = b"VK_EXT_sampler_filter_minmax\x00";
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME_STR: &'static str = "VK_EXT_sampler_filter_minmax";

/// See [`VkSamplerReductionModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeEXT)
/// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
cenum!(VkSamplerReductionModeEXT: u32 {
    /// See [`VkSamplerReductionModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeEXT)
    /// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
    const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT = 0,

    /// See [`VkSamplerReductionModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeEXT)
    /// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
    const VK_SAMPLER_REDUCTION_MODE_MIN_EXT = 1,

    /// See [`VkSamplerReductionModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeEXT)
    /// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
    const VK_SAMPLER_REDUCTION_MODE_MAX_EXT = 2,
});

/// See [`VkSamplerReductionModeCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerReductionModeCreateInfoEXT)
/// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSamplerReductionModeCreateInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub reductionMode: VkSamplerReductionModeEXT,
}

impl Default for VkSamplerReductionModeCreateInfoEXT {
    fn default() -> Self {
        VkSamplerReductionModeCreateInfoEXT {
            sType: core::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
            pNext: ptr::null(),
            reductionMode: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT)
/// and extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub filterMinmaxSingleComponentFormats: core::VkBool32,
    pub filterMinmaxImageComponentMapping: core::VkBool32,
}

impl Default for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
            sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            filterMinmaxSingleComponentFormats: Default::default(),
            filterMinmaxImageComponentMapping: Default::default(),
        }
    }
}
