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
use libc::{c_char, c_void};
use std::ptr;

#[cfg(feature = "ext_debug_marker_4")]
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;

#[cfg(all(feature = "ext_debug_marker_3", not(feature = "ext_debug_marker_4")))]
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 3;

pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static [u8; 20] = b"VK_EXT_debug_marker\x00";
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME_STR: &'static str = "VK_EXT_debug_marker";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub pObjectName: *const c_char,
}

impl Default for VkDebugMarkerObjectNameInfoEXT {
    fn default() -> Self {
        VkDebugMarkerObjectNameInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            pNext: ptr::null(),
            objectType: Default::default(),
            object: Default::default(),
            pObjectName: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

impl Default for VkDebugMarkerObjectTagInfoEXT {
    fn default() -> Self {
        VkDebugMarkerObjectTagInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
            pNext: ptr::null(),
            objectType: Default::default(),
            object: Default::default(),
            tagName: Default::default(),
            tagSize: Default::default(),
            pTag: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pMarkerName: *const c_char,
    pub color: [f32; 4],
}

impl Default for VkDebugMarkerMarkerInfoEXT {
    fn default() -> Self {
        VkDebugMarkerMarkerInfoEXT  {
            sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
            pNext: ptr::null(),
            pMarkerName: ptr::null(),
            color: Default::default(),
        }
    }
}

pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(device: VkDevice, pTagInfo: *mut VkDebugMarkerObjectTagInfoEXT) -> VkResult;
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(device: VkDevice, pNameInfo: *mut VkDebugMarkerObjectNameInfoEXT) -> VkResult;
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: *mut VkDebugMarkerObjectTagInfoEXT) -> VkResult;
    pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: *mut VkDebugMarkerObjectNameInfoEXT) -> VkResult;
    pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);
    pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer);
    pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT);
}
