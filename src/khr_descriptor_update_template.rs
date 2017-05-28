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

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static [u8; 34] = b"VK_KHR_descriptor_update_template\x00";
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME_STR: &'static str = "VK_KHR_descriptor_update_template";

/// See [`VkDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
#[repr(C)]
pub struct VkDescriptorUpdateTemplateKHR_T(c_void);

/// See [`VkDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type VkDescriptorUpdateTemplateKHR = *mut VkDescriptorUpdateTemplateKHR_T;

cenum!(VkDescriptorUpdateTemplateTypeKHR: u32 {
    /// See [`VkDescriptorUpdateTemplateTypeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateTypeKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR = 0,

    /// See [`VkDescriptorUpdateTemplateTypeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateTypeKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR = 1,
});

bitflags! {
    /// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDescriptorUpdateTemplateCreateFlagsKHR: u32 {
        const VK_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_DUMMY_KHR = 0x00000000;
    }
}

/// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type VkDescriptorUpdateTemplateCreateFlagBitsKHR = VkDescriptorUpdateTemplateCreateFlagsKHR;

/// See [`VkDescriptorUpdateTemplateEntryKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateEntryKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDescriptorUpdateTemplateEntryKHR {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

/// See [`VkDescriptorUpdateTemplateCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateInfoKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}

impl Default for VkDescriptorUpdateTemplateCreateInfoKHR {
    fn default() -> Self {
        VkDescriptorUpdateTemplateCreateInfoKHR  {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
            pNext: ptr::null_mut(),
            flags: Default::default(),
            descriptorUpdateEntryCount: Default::default(),
            pDescriptorUpdateEntries: ptr::null(),
            templateType: Default::default(),
            descriptorSetLayout: ptr::null_mut(),
            pipelineBindPoint: Default::default(),
            pipelineLayout: ptr::null_mut(),
            set: Default::default(),
        }
    }
}

/// See [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult;

/// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks);

/// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);

/// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
/// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);

extern "system" {
    /// See [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub fn vkCreateDescriptorUpdateTemplateKHR(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult;

    /// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub fn vkDestroyDescriptorUpdateTemplateKHR(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub fn vkUpdateDescriptorSetWithTemplateKHR(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);

    /// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);
}
