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

//! [`VK_KHX_multiview`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_multiview)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHX_MULTIVIEW_SPEC_VERSION: u32 = 1;
pub const VK_KHX_MULTIVIEW_EXTENSION_NAME: &'static [u8; 17] = b"VK_KHX_multiview\x00";
pub const VK_KHX_MULTIVIEW_EXTENSION_NAME_STR: &'static str = "VK_KHX_multiview";

/// See [`VkRenderPassMultiviewCreateInfoKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassMultiviewCreateInfoKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRenderPassMultiviewCreateInfoKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const u32,
}

impl Default for VkRenderPassMultiviewCreateInfoKHX {
    fn default() -> Self {
        VkRenderPassMultiviewCreateInfoKHX {
            sType: vk::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX,
            pNext: ptr::null(),
            subpassCount: Default::default(),
            pViewMasks: ptr::null(),
            dependencyCount: Default::default(),
            pViewOffsets: ptr::null(),
            correlationMaskCount: Default::default(),
            pCorrelationMasks: ptr::null(),
        }
    }
}

/// See [`VkPhysicalDeviceMultiviewFeaturesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMultiviewFeaturesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewFeaturesKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub multiview: vk::VkBool32,
    pub multiviewGeometryShader: vk::VkBool32,
    pub multiviewTessellationShader: vk::VkBool32,
}

impl Default for VkPhysicalDeviceMultiviewFeaturesKHX {
    fn default() -> Self {
        VkPhysicalDeviceMultiviewFeaturesKHX {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX,
            pNext: ptr::null_mut(),
            multiview: Default::default(),
            multiviewGeometryShader: Default::default(),
            multiviewTessellationShader: Default::default(),
        }
    }
}

/// See [`VkPhysicalDeviceMultiviewPropertiesKHX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMultiviewPropertiesKHX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewPropertiesKHX {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
}

impl Default for VkPhysicalDeviceMultiviewPropertiesKHX {
    fn default() -> Self {
        VkPhysicalDeviceMultiviewPropertiesKHX {
            sType: vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX,
            pNext: ptr::null_mut(),
            maxMultiviewViewCount: Default::default(),
            maxMultiviewInstanceIndex: Default::default(),
        }
    }
}
