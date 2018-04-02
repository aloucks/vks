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

//! [`VK_KHR_incremental_present`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_incremental_present)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_incremental_present\x00";
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME_STR: &str = "VK_KHR_incremental_present";

/// See [`VkRectLayerKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRectLayerKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkRectLayerKHR {
    pub offset: vk::VkOffset2D,
    pub extent: vk::VkExtent2D,
    pub layer: u32,
}

/// See [`VkPresentRegionKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentRegionKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentRegionKHR {
    pub rectangleCount: u32,
    pub pRectangles: *const VkRectLayerKHR,
}

impl Default for VkPresentRegionKHR {
    fn default() -> Self {
        VkPresentRegionKHR {
            rectangleCount: Default::default(),
            pRectangles: ptr::null(),
        }
    }
}

/// See [`VkPresentRegionsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentRegionsKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPresentRegionsKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pRegions: *const VkPresentRegionKHR,
}

impl Default for VkPresentRegionsKHR {
    fn default() -> Self {
        VkPresentRegionsKHR {
            sType: vk::VkStructureType::PRESENT_REGIONS_KHR,
            pNext: ptr::null(),
            swapchainCount: Default::default(),
            pRegions: ptr::null(),
        }
    }
}
