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

pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static [u8; 26] = b"VK_EXT_discard_rectangles\x00";
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME_STR: &'static str = "VK_EXT_discard_rectangles";

cenum!(VkDiscardRectangleModeEXT: u32 {
    /// See [`VkDiscardRectangleModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDiscardRectangleModeEXT)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,

    /// See [`VkDiscardRectangleModeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDiscardRectangleModeEXT)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
});

bitflags! {
    /// See [`VkPipelineDiscardRectangleStateCreateFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDiscardRectangleStateCreateFlagsEXT)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineDiscardRectangleStateCreateFlagsEXT: u32 {
        const VK_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_DUMMY_EXT = 0x00000000,
    }
}

/// See [`VkPipelineDiscardRectangleStateCreateFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDiscardRectangleStateCreateFlagsEXT)
/// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
pub type VkPipelineDiscardRectangleStateCreateFlagBitsEXT = VkPipelineDiscardRectangleStateCreateFlagsEXT;

/// See [`VkPhysicalDeviceDiscardRectanglePropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceDiscardRectanglePropertiesEXT)
/// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDiscardRectangles: u32,
}

impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceDiscardRectanglePropertiesEXT  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            maxDiscardRectangles: Default::default(),
        }
    }
}

/// See [`VkPipelineDiscardRectangleStateCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDiscardRectangleStateCreateInfoEXT)
/// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode: VkDiscardRectangleModeEXT,
    pub discardRectangleCount: u32,
    pub pDiscardRectangles: *const VkRect2D,
}

impl Default for VkPipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> Self {
        VkPipelineDiscardRectangleStateCreateInfoEXT  {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            discardRectangleMode: Default::default(),
            discardRectangleCount: Default::default(),
            pDiscardRectangles: ptr::null(),
        }
    }
}

/// See [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
/// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkCmdSetDiscardRectangleEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDiscardRectangleEXT)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);
}
