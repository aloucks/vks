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

define_non_dispatchable_handle!(
    /// See [`VkObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    struct VkObjectTableNVX;
);

define_non_dispatchable_handle!(
    /// See [`VkIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    struct VkIndirectCommandsLayoutNVX;
);

cenum!(VkIndirectCommandsTokenTypeNVX: u32 {
    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX = 0,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX = 1,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX = 2,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX = 3,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX = 4,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX = 5,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX = 6,

    /// See [`VkIndirectCommandsTokenTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX = 7,
});

cenum!(VkObjectEntryTypeNVX: u32 {
    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX = 0,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX = 1,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX = 2,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX = 3,

    /// See [`VkObjectEntryTypeNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryTypeNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX = 4,
});

bitflags! {
    /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkIndirectCommandsLayoutUsageFlagsNVX: u32 {
        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX = 0x00000001;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX = 0x00000002;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX = 0x00000004;

        /// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX = 0x00000008;
    }
}

/// See [`VkIndirectCommandsLayoutUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutUsageFlagBitsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type VkIndirectCommandsLayoutUsageFlagBitsNVX = VkIndirectCommandsLayoutUsageFlagsNVX;

bitflags! {
    /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkObjectEntryUsageFlagsNVX: u32 {
        /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX = 0x00000001;

        /// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX = 0x00000002;
    }
}

/// See [`VkObjectEntryUsageFlagBitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectEntryUsageFlagBitsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type VkObjectEntryUsageFlagBitsNVX = VkObjectEntryUsageFlagsNVX;

/// See [`VkDeviceGeneratedCommandsFeaturesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGeneratedCommandsFeaturesNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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

/// See [`VkDeviceGeneratedCommandsLimitsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceGeneratedCommandsLimitsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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

/// See [`VkIndirectCommandsTokenNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsTokenNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkIndirectCommandsTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
}

/// See [`VkIndirectCommandsLayoutTokenNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutTokenNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkIndirectCommandsLayoutTokenNVX {
    pub tokenType: VkIndirectCommandsTokenTypeNVX,
    pub bindingUnit: u32,
    pub dynamicCount: u32,
    pub divisor: u32,
}

/// See [`VkIndirectCommandsLayoutCreateInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndirectCommandsLayoutCreateInfoNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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

/// See [`VkCmdProcessCommandsInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCmdProcessCommandsInfoNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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
            objectTable: Default::default(),
            indirectCommandsLayout: Default::default(),
            maxSequencesCount: Default::default(),
        }
    }
}

/// See [`VkObjectTableCreateInfoNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableCreateInfoNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
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

/// See [`VkObjectTableEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
}

/// See [`VkObjectTablePipelineEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTablePipelineEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTablePipelineEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipeline: VkPipeline,
}

/// See [`VkObjectTableDescriptorSetEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableDescriptorSetEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableDescriptorSetEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: VkPipelineLayout,
    pub descriptorSet: VkDescriptorSet,
}

/// See [`VkObjectTableVertexBufferEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableVertexBufferEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableVertexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: VkBuffer,
}

/// See [`VkObjectTableIndexBufferEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTableIndexBufferEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTableIndexBufferEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub buffer: VkBuffer,
    pub indexType: VkIndexType,
}

/// See [`VkObjectTablePushConstantEntryNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkObjectTablePushConstantEntryNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkObjectTablePushConstantEntryNVX {
    pub type_: VkObjectEntryTypeNVX,
    pub flags: VkObjectEntryUsageFlagsNVX,
    pub pipelineLayout: VkPipelineLayout,
    pub stageFlags: VkShaderStageFlags,
}

/// See [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkCmdProcessCommandsNVX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);

/// See [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkCmdReserveSpaceForCommandsNVX = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);

/// See [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkCreateIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;

/// See [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkDestroyIndirectCommandsLayoutNVX = unsafe extern "system" fn(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkCreateObjectTableNVX = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;

/// See [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkDestroyObjectTableNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);

/// See [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkRegisterObjectsNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;

/// See [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkUnregisterObjectsNVX = unsafe extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;

/// See [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
/// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
pub type PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkCmdProcessCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdProcessCommandsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);

    /// See [`vkCmdReserveSpaceForCommandsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdReserveSpaceForCommandsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkCmdReserveSpaceForCommandsNVX(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);

    /// See [`vkCreateIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIndirectCommandsLayoutNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkCreateIndirectCommandsLayoutNVX(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;

    /// See [`vkDestroyIndirectCommandsLayoutNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyIndirectCommandsLayoutNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkDestroyIndirectCommandsLayoutNVX(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateObjectTableNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkCreateObjectTableNVX(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;

    /// See [`vkDestroyObjectTableNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyObjectTableNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkDestroyObjectTableNVX(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkRegisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkRegisterObjectsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkRegisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;

    /// See [`vkUnregisterObjectsNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnregisterObjectsNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkUnregisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;

    /// See [`vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);
}
