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

pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static [u8; 32] = b"VK_EXT_blend_operation_advanced\x00";
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME_STR: &'static str = "VK_EXT_blend_operation_advanced";

/// See [`VkBlendOverlapEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOverlapEXT)
/// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
cenum!(VkBlendOverlapEXT: u32 {
    /// See [`VkBlendOverlapEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOverlapEXT)
    /// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
    const VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,

    /// See [`VkBlendOverlapEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOverlapEXT)
    /// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
    const VK_BLEND_OVERLAP_DISJOINT_EXT = 1,

    /// See [`VkBlendOverlapEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOverlapEXT)
    /// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
    const VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
});

/// See [`VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT)
/// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendCoherentOperations: VkBool32,
}

impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> Self {
        VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            pNext: ptr::null_mut(),
            advancedBlendCoherentOperations: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT)
/// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendMaxColorAttachments: u32,
    pub advancedBlendIndependentBlend: VkBool32,
    pub advancedBlendNonPremultipliedSrcColor: VkBool32,
    pub advancedBlendNonPremultipliedDstColor: VkBool32,
    pub advancedBlendCorrelatedOverlap: VkBool32,
    pub advancedBlendAllOperations: VkBool32,
}

impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            advancedBlendMaxColorAttachments: Default::default(),
            advancedBlendIndependentBlend: Default::default(),
            advancedBlendNonPremultipliedSrcColor: Default::default(),
            advancedBlendNonPremultipliedDstColor: Default::default(),
            advancedBlendCorrelatedOverlap: Default::default(),
            advancedBlendAllOperations: Default::default(),
        }
    }
}

/// See [`VkPipelineColorBlendAdvancedStateCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendAdvancedStateCreateInfoEXT)
/// and extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcPremultiplied: VkBool32,
    pub dstPremultiplied: VkBool32,
    pub blendOverlap: VkBlendOverlapEXT,
}

impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> Self {
        VkPipelineColorBlendAdvancedStateCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
            pNext: ptr::null(),
            srcPremultiplied: Default::default(),
            dstPremultiplied: Default::default(),
            blendOverlap: Default::default(),
        }
    }
}
