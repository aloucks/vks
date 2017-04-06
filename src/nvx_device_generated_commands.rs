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

pub const VK_NVX_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 1;
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static [u8; 33] = b"VK_NVX_device_generated_commands\x00";
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME_STR: &'static str = "VK_NVX_device_generated_commands";

#[repr(C)]
pub struct VkObjectTableNVX_T(c_void);
pub type VkObjectTableNVX = *mut VkObjectTableNVX_T;

#[repr(C)]
pub struct VkIndirectCommandsLayoutNVX_T(c_void);
pub type VkIndirectCommandsLayoutNVX = *mut VkIndirectCommandsLayoutNVX_T;

cenum!(VkIndirectCommandsTokenTypeNVX: u32 {
    const VK_INDIRECT_COMMANDS_TOKEN_PIPELINE_NVX = 0,
    const VK_INDIRECT_COMMANDS_TOKEN_DESCRIPTOR_SET_NVX = 1,
    const VK_INDIRECT_COMMANDS_TOKEN_INDEX_BUFFER_NVX = 2,
    const VK_INDIRECT_COMMANDS_TOKEN_VERTEX_BUFFER_NVX = 3,
    const VK_INDIRECT_COMMANDS_TOKEN_PUSH_CONSTANT_NVX = 4,
    const VK_INDIRECT_COMMANDS_TOKEN_DRAW_INDEXED_NVX = 5,
    const VK_INDIRECT_COMMANDS_TOKEN_DRAW_NVX = 6,
    const VK_INDIRECT_COMMANDS_TOKEN_DISPATCH_NVX = 7,
});

cenum!(VkObjectEntryTypeNVX: u32 {
    const VK_OBJECT_ENTRY_DESCRIPTOR_SET_NVX = 0,
    const VK_OBJECT_ENTRY_PIPELINE_NVX = 1,
    const VK_OBJECT_ENTRY_INDEX_BUFFER_NVX = 2,
    const VK_OBJECT_ENTRY_VERTEX_BUFFER_NVX = 3,
    const VK_OBJECT_ENTRY_PUSH_CONSTANT_NVX = 4,
});

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkIndirectCommandsLayoutUsageFlagsNVX: u32 {
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX = 0x00000001,
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX = 0x00000002,
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX = 0x00000004,
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX = 0x00000008,
    }
}
pub type VkIndirectCommandsLayoutUsageFlagBitsNVX = VkIndirectCommandsLayoutUsageFlagsNVX;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkObjectEntryUsageFlagsNVX: u32 {
        const VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX = 0x00000001,
        const VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX = 0x00000002,
    }
}
pub type VkObjectEntryUsageFlagBitsNVX = VkObjectEntryUsageFlagsNVX;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGeneratedCommandsFeaturesNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub computeBindingPointSupport: VkBool32,
}

impl Default for VkDeviceGeneratedCommandsFeaturesNVX {
    fn default() -> Self {
        VkDeviceGeneratedCommandsFeaturesNVX  {
            sType: VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
            pNext: ptr::null(),
            computeBindingPointSupport: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGeneratedCommandsLimitsNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxIndirectCommandsLayoutTokenCount: u32,
    pub maxObjectEntryCounts: u32,
    pub minSequenceCountBufferOffsetAlignment: u32,
    pub minSequenceIndexBufferOffsetAlignment: u32,
    pub minCommandsTokenBufferOffsetAlignment: u32,
}

impl Default for VkDeviceGeneratedCommandsLimitsNVX {
    fn default() -> Self {
        VkDeviceGeneratedCommandsLimitsNVX  {
            sType: VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
            pNext: ptr::null(),
            maxIndirectCommandsLayoutTokenCount: Default::default(),
            maxObjectEntryCounts: Default::default(),
            minSequenceCountBufferOffsetAlignment: Default::default(),
            minSequenceIndexBufferOffsetAlignment: Default::default(),
            minCommandsTokenBufferOffsetAlignment: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkIndirectCommandsTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
}

impl Default for VkIndirectCommandsTokenNVX {
    fn default() -> Self {
        VkIndirectCommandsTokenNVX  {
            tokenType: Default::default(),
            buffer: ptr::null_mut(),
            offset: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkIndirectCommandsLayoutTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub bindingUnit: u32,
    pub dynamicCount: u32,
    pub divisor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkIndirectCommandsLayoutCreateInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
    pub tokenCount: u32,
    pub pTokens: *const VkIndirectCommandsLayoutTokenNVX,
}

impl Default for VkIndirectCommandsLayoutCreateInfoNVX {
    fn default() -> Self {
        VkIndirectCommandsLayoutCreateInfoNVX  {
            sType: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
            pNext: ptr::null(),
            pipelineBindPoint: Default::default(),
            flags: Default::default(),
            tokenCount: Default::default(),
            pTokens: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCmdProcessCommandsInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectTable: VkObjectTableNVX,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
    pub indirectCommandsTokenCount: u32,
    pub pIndirectCommandsTokens: *const VkIndirectCommandsTokenNVX,
    pub maxSequencesCount: u32,
    pub targetCommandBuffer: VkCommandBuffer,
    pub sequencesCountBuffer: VkBuffer,
    pub sequencesCountOffset: VkDeviceSize,
    pub sequencesIndexBuffer: VkBuffer,
    pub sequencesIndexOffset: VkDeviceSize,
}

impl Default for VkCmdProcessCommandsInfoNVX {
    fn default() -> Self {
        VkCmdProcessCommandsInfoNVX  {
            sType: VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX,
            pNext: ptr::null(),
            objectTable: ptr::null_mut(),
            indirectCommandsLayout: ptr::null_mut(),
            indirectCommandsTokenCount: Default::default(),
            pIndirectCommandsTokens: ptr::null(),
            maxSequencesCount: Default::default(),
            targetCommandBuffer: ptr::null_mut(),
            sequencesCountBuffer: ptr::null_mut(),
            sequencesCountOffset: Default::default(),
            sequencesIndexBuffer: ptr::null_mut(),
            sequencesIndexOffset: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCmdReserveSpaceForCommandsInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectTable: VkObjectTableNVX,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
    pub maxSequencesCount: u32,
}

impl Default for VkCmdReserveSpaceForCommandsInfoNVX {
    fn default() -> Self {
        VkCmdReserveSpaceForCommandsInfoNVX  {
            sType: VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
            pNext: ptr::null(),
            objectTable: ptr::null_mut(),
            indirectCommandsLayout: ptr::null_mut(),
            maxSequencesCount: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTableCreateInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectCount: u32,
    pub pObjectEntryTypes: *const VkObjectEntryTypeNVX,
    pub pObjectEntryCounts: *const u32,
    pub pObjectEntryUsageFlags: *const VkObjectEntryUsageFlagsNVX,
    pub maxUniformBuffersPerDescriptor: u32,
    pub maxStorageBuffersPerDescriptor: u32,
    pub maxStorageImagesPerDescriptor: u32,
    pub maxSampledImagesPerDescriptor: u32,
    pub maxPipelineLayouts: u32,
}

impl Default for VkObjectTableCreateInfoNVX {
    fn default() -> Self {
        VkObjectTableCreateInfoNVX  {
            sType: VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX,
            pNext: ptr::null(),
            objectCount: Default::default(),
            pObjectEntryTypes: ptr::null(),
            pObjectEntryCounts: ptr::null(),
            pObjectEntryUsageFlags: ptr::null(),
            maxUniformBuffersPerDescriptor: Default::default(),
            maxStorageBuffersPerDescriptor: Default::default(),
            maxStorageImagesPerDescriptor: Default::default(),
            maxSampledImagesPerDescriptor: Default::default(),
            maxPipelineLayouts: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTablePipelineEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipeline: VkPipeline,
}

impl Default for VkObjectTablePipelineEntryNVX {
    fn default() -> Self {
        VkObjectTablePipelineEntryNVX  {
            type_: Default::default(),
            flags: Default::default(),
            pipeline: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTableDescriptorSetEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: VkPipelineLayout,
    pub descriptorSet: VkDescriptorSet,
}

impl Default for VkObjectTableDescriptorSetEntryNVX {
    fn default() -> Self {
        VkObjectTableDescriptorSetEntryNVX  {
            type_: Default::default(),
            flags: Default::default(),
            pipelineLayout: ptr::null_mut(),
            descriptorSet: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTableVertexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: VkBuffer,
}

impl Default for VkObjectTableVertexBufferEntryNVX {
    fn default() -> Self {
        VkObjectTableVertexBufferEntryNVX  {
            type_: Default::default(),
            flags: Default::default(),
            buffer: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTableIndexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: VkBuffer,
    pub indexType: VkIndexType,
}

impl Default for VkObjectTableIndexBufferEntryNVX {
    fn default() -> Self {
        VkObjectTableIndexBufferEntryNVX  {
            type_: Default::default(),
            flags: Default::default(),
            buffer: ptr::null_mut(),
            indexType: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTablePushConstantEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: VkPipelineLayout,
    pub stageFlags: VkShaderStageFlags,
}

impl Default for VkObjectTablePushConstantEntryNVX {
    fn default() -> Self {
        VkObjectTablePushConstantEntryNVX  {
            type_: Default::default(),
            flags: Default::default(),
            pipelineLayout: ptr::null_mut(),
            stageFlags: Default::default(),
        }
    }
}

pub type PFN_vkCmdProcessCommandsNVX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);
pub type PFN_vkCmdReserveSpaceForCommandsNVX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);
pub type PFN_vkCreateIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;
pub type PFN_vkDestroyIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateObjectTableNVX = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;
pub type PFN_vkDestroyObjectTableNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkRegisterObjectsNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;
pub type PFN_vkUnregisterObjectsNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;
pub type PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);
    pub fn vkCmdReserveSpaceForCommandsNVX(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);
    pub fn vkCreateIndirectCommandsLayoutNVX(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;
    pub fn vkDestroyIndirectCommandsLayoutNVX(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateObjectTableNVX(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;
    pub fn vkDestroyObjectTableNVX(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);
    pub fn vkRegisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;
    pub fn vkUnregisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;
    pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);
}
