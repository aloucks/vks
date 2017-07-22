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

//! [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)

use core;
use libc::c_void;
use std::ptr;

pub const VK_NVX_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 1;
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static [u8; 33] = b"VK_NVX_device_generated_commands\x00";
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME_STR: &'static str = "VK_NVX_device_generated_commands";

define_non_dispatchable_handle!(
    /// See [`VkObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableNVX)
    struct VkObjectTableNVX;
);

define_non_dispatchable_handle!(
    /// See [`VkIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutNVX)
    struct VkIndirectCommandsLayoutNVX;
);

cenum!(VkIndirectCommandsTokenTypeNVX: u32 {
    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX = 0,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX = 1,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX = 2,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX = 3,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX = 4,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX = 5,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX = 6,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX = 7,
});

cenum!(VkObjectEntryTypeNVX: u32 {
    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    const VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX = 0,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    const VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX = 1,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    const VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX = 2,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    const VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX = 3,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    const VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX = 4,
});

vks_bitflags! {
    /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkIndirectCommandsLayoutUsageFlagsNVX: u32 {
        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_MAX_ENUM_NVX = 0x7fffffff;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX = 0x00000001;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX = 0x00000002;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX = 0x00000004;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX = 0x00000008;
    }
}

/// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
pub type VkIndirectCommandsLayoutUsageFlagBitsNVX = VkIndirectCommandsLayoutUsageFlagsNVX;

vks_bitflags! {
    /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkObjectEntryUsageFlagsNVX: u32 {
        /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
        const VK_OBJECT_ENTRY_USAGE_FLAG_BITS_MAX_ENUM_NVX = 0x7fffffff;

        /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
        const VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX = 0x00000001;

        /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
        const VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX = 0x00000002;
    }
}

/// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
pub type VkObjectEntryUsageFlagBitsNVX = VkObjectEntryUsageFlagsNVX;

/// See [`VkDeviceGeneratedCommandsFeaturesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGeneratedCommandsFeaturesNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGeneratedCommandsFeaturesNVX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub computeBindingPointSupport: core::VkBool32,
}

impl Default for VkDeviceGeneratedCommandsFeaturesNVX {
    fn default() -> Self {
        VkDeviceGeneratedCommandsFeaturesNVX  {
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
            pNext: ptr::null(),
            computeBindingPointSupport: Default::default(),
        }
    }
}

/// See [`VkDeviceGeneratedCommandsLimitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGeneratedCommandsLimitsNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceGeneratedCommandsLimitsNVX {
    pub sType: core::VkStructureType,
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
            sType: core::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
            pNext: ptr::null(),
            maxIndirectCommandsLayoutTokenCount: Default::default(),
            maxObjectEntryCounts: Default::default(),
            minSequenceCountBufferOffsetAlignment: Default::default(),
            minSequenceIndexBufferOffsetAlignment: Default::default(),
            minCommandsTokenBufferOffsetAlignment: Default::default(),
        }
    }
}

/// See [`VkIndirectCommandsTokenNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkIndirectCommandsTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub buffer: core::VkBuffer,
    pub offset: core::VkDeviceSize,
}

/// See [`VkIndirectCommandsLayoutTokenNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutTokenNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkIndirectCommandsLayoutTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub bindingUnit: u32,
    pub dynamicCount: u32,
    pub divisor: u32,
}

/// See [`VkIndirectCommandsLayoutCreateInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutCreateInfoNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkIndirectCommandsLayoutCreateInfoNVX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBindPoint: core::VkPipelineBindPoint,
    pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
    pub tokenCount: u32,
    pub pTokens: *const VkIndirectCommandsLayoutTokenNVX,
}

impl Default for VkIndirectCommandsLayoutCreateInfoNVX {
    fn default() -> Self {
        VkIndirectCommandsLayoutCreateInfoNVX  {
            sType: core::VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
            pNext: ptr::null(),
            pipelineBindPoint: Default::default(),
            flags: Default::default(),
            tokenCount: Default::default(),
            pTokens: ptr::null(),
        }
    }
}

/// See [`VkCmdProcessCommandsInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCmdProcessCommandsInfoNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCmdProcessCommandsInfoNVX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub objectTable: VkObjectTableNVX,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
    pub indirectCommandsTokenCount: u32,
    pub pIndirectCommandsTokens: *const VkIndirectCommandsTokenNVX,
    pub maxSequencesCount: u32,
    pub targetCommandBuffer: core::VkCommandBuffer,
    pub sequencesCountBuffer: core::VkBuffer,
    pub sequencesCountOffset: core::VkDeviceSize,
    pub sequencesIndexBuffer: core::VkBuffer,
    pub sequencesIndexOffset: core::VkDeviceSize,
}

impl Default for VkCmdProcessCommandsInfoNVX {
    fn default() -> Self {
        VkCmdProcessCommandsInfoNVX  {
            sType: core::VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX,
            pNext: ptr::null(),
            objectTable: Default::default(),
            indirectCommandsLayout: Default::default(),
            indirectCommandsTokenCount: Default::default(),
            pIndirectCommandsTokens: ptr::null(),
            maxSequencesCount: Default::default(),
            targetCommandBuffer: ptr::null_mut(),
            sequencesCountBuffer: Default::default(),
            sequencesCountOffset: Default::default(),
            sequencesIndexBuffer: Default::default(),
            sequencesIndexOffset: Default::default(),
        }
    }
}

/// See [`VkCmdReserveSpaceForCommandsInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCmdReserveSpaceForCommandsInfoNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCmdReserveSpaceForCommandsInfoNVX {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub objectTable: VkObjectTableNVX,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
    pub maxSequencesCount: u32,
}

impl Default for VkCmdReserveSpaceForCommandsInfoNVX {
    fn default() -> Self {
        VkCmdReserveSpaceForCommandsInfoNVX  {
            sType: core::VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
            pNext: ptr::null(),
            objectTable: Default::default(),
            indirectCommandsLayout: Default::default(),
            maxSequencesCount: Default::default(),
        }
    }
}

/// See [`VkObjectTableCreateInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableCreateInfoNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkObjectTableCreateInfoNVX {
    pub sType: core::VkStructureType,
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
            sType: core::VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX,
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

/// See [`VkObjectTableEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
}

/// See [`VkObjectTablePipelineEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTablePipelineEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTablePipelineEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipeline: core::VkPipeline,
}

/// See [`VkObjectTableDescriptorSetEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableDescriptorSetEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableDescriptorSetEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: core::VkPipelineLayout,
    pub descriptorSet: core::VkDescriptorSet,
}

/// See [`VkObjectTableVertexBufferEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableVertexBufferEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableVertexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: core::VkBuffer,
}

/// See [`VkObjectTableIndexBufferEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableIndexBufferEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableIndexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: core::VkBuffer,
    pub indexType: core::VkIndexType,
}

/// See [`VkObjectTablePushConstantEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTablePushConstantEntryNVX)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTablePushConstantEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: core::VkPipelineLayout,
    pub stageFlags: core::VkShaderStageFlags,
}

/// See [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
pub type PFN_vkCmdProcessCommandsNVX = unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);

/// See [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
pub type PFN_vkCmdReserveSpaceForCommandsNVX = unsafe extern "system" fn(commandBuffer: core::VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);

/// See [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
pub type PFN_vkCreateIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: core::VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> core::VkResult;

/// See [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
pub type PFN_vkDestroyIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: core::VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const core::VkAllocationCallbacks);

/// See [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
pub type PFN_vkCreateObjectTableNVX = unsafe extern "system" fn(device: core::VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> core::VkResult;

/// See [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
pub type PFN_vkDestroyObjectTableNVX = unsafe extern "system" fn(device: core::VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const core::VkAllocationCallbacks);

/// See [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
pub type PFN_vkRegisterObjectsNVX = unsafe extern "system" fn(device: core::VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> core::VkResult;

/// See [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
pub type PFN_vkUnregisterObjectsNVX = unsafe extern "system" fn(device: core::VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> core::VkResult;

/// See [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
pub type PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
    pub fn vkCmdProcessCommandsNVX(commandBuffer: core::VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);

    /// See [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
    pub fn vkCmdReserveSpaceForCommandsNVX(commandBuffer: core::VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);

    /// See [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
    pub fn vkCreateIndirectCommandsLayoutNVX(device: core::VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> core::VkResult;

    /// See [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
    pub fn vkDestroyIndirectCommandsLayoutNVX(device: core::VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const core::VkAllocationCallbacks);

    /// See [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
    pub fn vkCreateObjectTableNVX(device: core::VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const core::VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> core::VkResult;

    /// See [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
    pub fn vkDestroyObjectTableNVX(device: core::VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const core::VkAllocationCallbacks);

    /// See [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
    pub fn vkRegisterObjectsNVX(device: core::VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> core::VkResult;

    /// See [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
    pub fn vkUnregisterObjectsNVX(device: core::VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
    pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(physicalDevice: core::VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);
}
