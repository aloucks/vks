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

pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_push_descriptor\x00";
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME_STR: &'static str = "VK_KHR_push_descriptor";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPushDescriptors: u32,
}

impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> Self {
        VkPhysicalDevicePushDescriptorPropertiesKHR  {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            maxPushDescriptors: Default::default(),
        }
    }
}

pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCmdPushDescriptorSetKHR(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
}
