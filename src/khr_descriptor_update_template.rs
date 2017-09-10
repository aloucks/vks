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

//! [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)

use core;
use libc::c_void;
use std::ptr;

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static [u8; 34] = b"VK_KHR_descriptor_update_template\x00";
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME_STR: &'static str = "VK_KHR_descriptor_update_template";

define_non_dispatchable_handle!(
    /// See [`VkDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateKHR)
    struct VkDescriptorUpdateTemplateKHR;
);

cenum!(VkDescriptorUpdateTemplateTypeKHR: u32 {
    /// See [`VkDescriptorUpdateTemplateTypeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateTypeKHR)
    const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR = 0,

    /// See [`VkDescriptorUpdateTemplateTypeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateTypeKHR)
    const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR = 1,
});

vks_bitflags! {
    /// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkDescriptorUpdateTemplateCreateFlagsKHR: u32 {
        /// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
        const VK_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
pub type VkDescriptorUpdateTemplateCreateFlagBitsKHR = VkDescriptorUpdateTemplateCreateFlagsKHR;

/// See [`VkDescriptorUpdateTemplateEntryKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateEntryKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDescriptorUpdateTemplateEntryKHR {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: core::VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

/// See [`VkDescriptorUpdateTemplateCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: core::VkDescriptorSetLayout,
    pub pipelineBindPoint: core::VkPipelineBindPoint,
    pub pipelineLayout: core::VkPipelineLayout,
    pub set: u32,
}

impl Default for VkDescriptorUpdateTemplateCreateInfoKHR {
    fn default() -> Self {
        VkDescriptorUpdateTemplateCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
            pNext: ptr::null_mut(),
            flags: Default::default(),
            descriptorUpdateEntryCount: Default::default(),
            pDescriptorUpdateEntries: ptr::null(),
            templateType: Default::default(),
            descriptorSetLayout: Default::default(),
            pipelineBindPoint: Default::default(),
            pipelineLayout: Default::default(),
            set: Default::default(),
        }
    }
}

/// See [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = Option<unsafe extern "system" fn(device: core::VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> core::VkResult>;

/// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = Option<unsafe extern "system" fn(device: core::VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const core::VkAllocationCallbacks)>;

/// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = Option<unsafe extern "system" fn(device: core::VkDevice, descriptorSet: core::VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void)>;

/// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = Option<unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: core::VkPipelineLayout, set: u32, pData: *const c_void)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
    pub fn vkCreateDescriptorUpdateTemplateKHR(device: core::VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> core::VkResult;

    /// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
    pub fn vkDestroyDescriptorUpdateTemplateKHR(device: core::VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const core::VkAllocationCallbacks);

    /// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
    pub fn vkUpdateDescriptorSetWithTemplateKHR(device: core::VkDevice, descriptorSet: core::VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);

    /// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
    pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: core::VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: core::VkPipelineLayout, set: u32, pData: *const c_void);
}
