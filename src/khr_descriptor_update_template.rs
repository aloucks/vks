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

//! [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)

use core::ptr;
use libc::c_void;
use vk;

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &[u8; 34] = b"VK_KHR_descriptor_update_template\x00";
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME_STR: &str = "VK_KHR_descriptor_update_template";

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

bitflags! {
    /// See [`VkDescriptorUpdateTemplateCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateFlagsKHR)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDescriptorUpdateTemplateCreateFlagsKHR: u32 {
        const MAX_ENUM_KHR = 0x7fffffff;
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
    pub descriptorType: vk::VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

/// See [`VkDescriptorUpdateTemplateCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorUpdateTemplateCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: vk::VkDescriptorSetLayout,
    pub pipelineBindPoint: vk::VkPipelineBindPoint,
    pub pipelineLayout: vk::VkPipelineLayout,
    pub set: u32,
}

impl Default for VkDescriptorUpdateTemplateCreateInfoKHR {
    fn default() -> Self {
        VkDescriptorUpdateTemplateCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
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
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> vk::VkResult>;

/// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const vk::VkAllocationCallbacks)>;

/// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = Option<unsafe extern "system" fn(device: vk::VkDevice, descriptorSet: vk::VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void)>;

/// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = Option<unsafe extern "system" fn(commandBuffer: vk::VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: vk::VkPipelineLayout, set: u32, pData: *const c_void)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorUpdateTemplateKHR)
    pub fn vkCreateDescriptorUpdateTemplateKHR(device: vk::VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> vk::VkResult;

    /// See [`vkDestroyDescriptorUpdateTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorUpdateTemplateKHR)
    pub fn vkDestroyDescriptorUpdateTemplateKHR(device: vk::VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const vk::VkAllocationCallbacks);

    /// See [`vkUpdateDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSetWithTemplateKHR)
    pub fn vkUpdateDescriptorSetWithTemplateKHR(device: vk::VkDevice, descriptorSet: vk::VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);

    /// See [`vkCmdPushDescriptorSetWithTemplateKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushDescriptorSetWithTemplateKHR)
    pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: vk::VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: vk::VkPipelineLayout, set: u32, pData: *const c_void);
}
