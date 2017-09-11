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

//! [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)

use core;
use ext_debug_report;
use libc::{c_char, c_void};
use std::ptr;

pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static [u8; 20] = b"VK_EXT_debug_marker\x00";
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME_STR: &'static str = "VK_EXT_debug_marker";

/// See [`VkDebugMarkerObjectNameInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerObjectNameInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub objectType: ext_debug_report::VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub pObjectName: *const c_char,
}

impl Default for VkDebugMarkerObjectNameInfoEXT {
    fn default() -> Self {
        VkDebugMarkerObjectNameInfoEXT {
            sType: core::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            pNext: ptr::null(),
            objectType: Default::default(),
            object: Default::default(),
            pObjectName: ptr::null(),
        }
    }
}

/// See [`VkDebugMarkerObjectTagInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerObjectTagInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub objectType: ext_debug_report::VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

impl Default for VkDebugMarkerObjectTagInfoEXT {
    fn default() -> Self {
        VkDebugMarkerObjectTagInfoEXT {
            sType: core::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
            pNext: ptr::null(),
            objectType: Default::default(),
            object: Default::default(),
            tagName: Default::default(),
            tagSize: Default::default(),
            pTag: ptr::null(),
        }
    }
}

/// See [`VkDebugMarkerMarkerInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerMarkerInfoEXT)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub pMarkerName: *const c_char,
    pub color: [f32; 4],
}

impl Default for VkDebugMarkerMarkerInfoEXT {
    fn default() -> Self {
        VkDebugMarkerMarkerInfoEXT {
            sType: core::VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
            pNext: ptr::null(),
            pMarkerName: ptr::null(),
            color: Default::default(),
        }
    }
}

/// See [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<unsafe extern "system" fn(device: core::VkDevice, pTagInfo: *mut VkDebugMarkerObjectTagInfoEXT) -> core::VkResult>;

/// See [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<unsafe extern "system" fn(device: core::VkDevice, pNameInfo: *mut VkDebugMarkerObjectNameInfoEXT) -> core::VkResult>;

/// See [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
pub type PFN_vkCmdDebugMarkerBeginEXT = Option<unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT)>;

/// See [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
pub type PFN_vkCmdDebugMarkerEndEXT = Option<unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer)>;

/// See [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
pub type PFN_vkCmdDebugMarkerInsertEXT = Option<unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
    pub fn vkDebugMarkerSetObjectTagEXT(device: core::VkDevice, pTagInfo: *mut VkDebugMarkerObjectTagInfoEXT) -> core::VkResult;

    /// See [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
    pub fn vkDebugMarkerSetObjectNameEXT(device: core::VkDevice, pNameInfo: *mut VkDebugMarkerObjectNameInfoEXT) -> core::VkResult;

    /// See [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
    pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);

    /// See [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
    pub fn vkCmdDebugMarkerEndEXT(commandBuffer: core::VkCommandBuffer);

    /// See [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
    pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: core::VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);
}
