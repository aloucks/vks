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

use libc::{c_char, c_void};
use std::fmt;
use std::mem;
use std::ptr;

#[cfg(not(feature = "unstable_rust"))]
use union_field::VkSysUnionField;

/// See [`VkBool32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBool32)
pub type VkBool32 = u32;

/// See [`VkDeviceSize`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceSize)
pub type VkDeviceSize = u64;

/// See [`VkSampleMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleMask)
pub type VkSampleMask = u32;

/// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
#[repr(C)]
pub struct VkInstance_T(c_void);

/// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
pub type VkInstance = *mut VkInstance_T;

/// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
#[repr(C)]
pub struct VkPhysicalDevice_T(c_void);

/// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;

/// See [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDevice)
#[repr(C)]
pub struct VkDevice_T(c_void);

/// See [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDevice)
pub type VkDevice = *mut VkDevice_T;

/// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
#[repr(C)]
pub struct VkQueue_T(c_void);

/// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
pub type VkQueue = *mut VkQueue_T;

/// See [`VkSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphore)
#[repr(C)]
pub struct VkSemaphore_T(c_void);

/// See [`VkSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphore)
pub type VkSemaphore = *mut VkSemaphore_T;

/// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
#[repr(C)]
pub struct VkCommandBuffer_T(c_void);

/// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
pub type VkCommandBuffer = *mut VkCommandBuffer_T;

/// See [`VkFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFence)
#[repr(C)]
pub struct VkFence_T(c_void);

/// See [`VkFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFence)
pub type VkFence = *mut VkFence_T;

/// See [`VkDeviceMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceMemory)
#[repr(C)]
pub struct VkDeviceMemory_T(c_void);

/// See [`VkDeviceMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceMemory)
pub type VkDeviceMemory = *mut VkDeviceMemory_T;

/// See [`VkBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBuffer)
#[repr(C)]
pub struct VkBuffer_T(c_void);

/// See [`VkBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBuffer)
pub type VkBuffer = *mut VkBuffer_T;

/// See [`VkImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImage)
#[repr(C)]
pub struct VkImage_T(c_void);

/// See [`VkImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImage)
pub type VkImage = *mut VkImage_T;

/// See [`VkEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEvent)
#[repr(C)]
pub struct VkEvent_T(c_void);

/// See [`VkEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEvent)
pub type VkEvent = *mut VkEvent_T;

/// See [`VkQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPool)
#[repr(C)]
pub struct VkQueryPool_T(c_void);

/// See [`VkQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPool)
pub type VkQueryPool = *mut VkQueryPool_T;

/// See [`VkBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferView)
#[repr(C)]
pub struct VkBufferView_T(c_void);

/// See [`VkBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferView)
pub type VkBufferView = *mut VkBufferView_T;

/// See [`VkImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageView)
#[repr(C)]
pub struct VkImageView_T(c_void);

/// See [`VkImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageView)
pub type VkImageView = *mut VkImageView_T;

/// See [`VkShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModule)
#[repr(C)]
pub struct VkShaderModule_T(c_void);

/// See [`VkShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModule)
pub type VkShaderModule = *mut VkShaderModule_T;

/// See [`VkPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCache)
#[repr(C)]
pub struct VkPipelineCache_T(c_void);

/// See [`VkPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCache)
pub type VkPipelineCache = *mut VkPipelineCache_T;

/// See [`VkPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayout)
#[repr(C)]
pub struct VkPipelineLayout_T(c_void);

/// See [`VkPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayout)
pub type VkPipelineLayout = *mut VkPipelineLayout_T;

/// See [`VkRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPass)
#[repr(C)]
pub struct VkRenderPass_T(c_void);

/// See [`VkRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPass)
pub type VkRenderPass = *mut VkRenderPass_T;

/// See [`VkPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipeline)
#[repr(C)]
pub struct VkPipeline_T(c_void);

/// See [`VkPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipeline)
pub type VkPipeline = *mut VkPipeline_T;

/// See [`VkDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayout)
#[repr(C)]
pub struct VkDescriptorSetLayout_T(c_void);

/// See [`VkDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayout)
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;

/// See [`VkSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampler)
#[repr(C)]
pub struct VkSampler_T(c_void);

/// See [`VkSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampler)
pub type VkSampler = *mut VkSampler_T;

/// See [`VkDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPool)
#[repr(C)]
pub struct VkDescriptorPool_T(c_void);

/// See [`VkDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPool)
pub type VkDescriptorPool = *mut VkDescriptorPool_T;

/// See [`VkDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSet)
#[repr(C)]
pub struct VkDescriptorSet_T(c_void);

/// See [`VkDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSet)
pub type VkDescriptorSet = *mut VkDescriptorSet_T;

/// See [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebuffer)
#[repr(C)]
pub struct VkFramebuffer_T(c_void);

/// See [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebuffer)
pub type VkFramebuffer = *mut VkFramebuffer_T;

/// See [`VkCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPool)
#[repr(C)]
pub struct VkCommandPool_T(c_void);

/// See [`VkCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPool)
pub type VkCommandPool = *mut VkCommandPool_T;

pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
pub const VK_REMAINING_MIP_LEVELS: u32 = 0xffffffff;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 0xffffffff;
pub const VK_WHOLE_SIZE: VkDeviceSize = 0xffffffffffffffff;
pub const VK_ATTACHMENT_UNUSED: u32 = 0xffffffff;
pub const VK_TRUE: VkBool32 = 1;
pub const VK_FALSE: VkBool32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xffffffff;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xffffffff;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;

cenum!(VkPipelineCacheHeaderVersion: u32 {
    /// See [`VkPipelineCacheHeaderVersion`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheHeaderVersion)
    const VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1,
});

cenum!(VkResult: i32 {
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_SUCCESS = 0,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_NOT_READY = 1,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_TIMEOUT = 2,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_EVENT_SET = 3,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_EVENT_RESET = 4,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_INCOMPLETE = 5,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_OUT_OF_HOST_MEMORY = -1,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_INITIALIZATION_FAILED = -3,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_DEVICE_LOST = -4,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_MEMORY_MAP_FAILED = -5,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_LAYER_NOT_PRESENT = -6,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_EXTENSION_NOT_PRESENT = -7,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_FEATURE_NOT_PRESENT = -8,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_INCOMPATIBLE_DRIVER = -9,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_TOO_MANY_OBJECTS = -10,

    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_FORMAT_NOT_SUPPORTED = -11,

    #[cfg(feature = "core_1_0_22")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    const VK_ERROR_FRAGMENTED_POOL = -12,

    #[cfg(feature = "khr_surface_25")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_ERROR_SURFACE_LOST_KHR = -1000000000,

    #[cfg(feature = "khr_surface_25")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,

    #[cfg(feature = "khr_swapchain_67")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    const VK_SUBOPTIMAL_KHR = 1000001003,

    #[cfg(feature = "khr_swapchain_67")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    const VK_ERROR_OUT_OF_DATE_KHR = -1000001004,

    #[cfg(feature = "khr_display_swapchain_9")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,

    #[cfg(feature = "ext_debug_report_1")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_debug_report)
    const VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,

    #[cfg(feature = "nv_glsl_shader_1")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_NV_glsl_shader`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_glsl_shader)
    const VK_ERROR_INVALID_SHADER_NV = -1000012000,

    #[cfg(feature = "khr_maintenance1_1")]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
    const VK_ERROR_OUT_OF_POOL_MEMORY_KHR = -1000069000,

    #[cfg(any(feature = "khx_external_memory_1", feature = "khx_external_semaphore_1"))]
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    /// and extensions [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory),
    /// [`VK_KHX_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore)
    const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHX = -1000072003,
});

cenum!(VkStructureType: u32 {
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,

    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,

    #[cfg(feature = "khr_swapchain_67")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,

    #[cfg(feature = "khr_swapchain_67")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,

    #[cfg(feature = "khr_display_21")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,

    #[cfg(feature = "khr_display_21")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,

    #[cfg(feature = "khr_display_swapchain_9")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR = 1000003000,

    #[cfg(feature = "khr_xlib_surface_6")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,

    #[cfg(feature = "khr_xcb_surface_6")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,

    #[cfg(feature = "khr_wayland_surface_5")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,

    #[cfg(feature = "khr_mir_surface_4")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
    const VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,

    #[cfg(feature = "khr_android_surface_6")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)
    const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,

    #[cfg(feature = "khr_win32_surface_5")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
    const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

    #[cfg(feature = "ext_debug_report_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT = 1000011000,

    #[cfg(feature = "ext_debug_report_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,

    #[cfg(feature = "amd_rasterization_order_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)
    const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,

    #[cfg(feature = "ext_debug_marker_3")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,

    #[cfg(feature = "ext_debug_marker_3")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,

    #[cfg(feature = "ext_debug_marker_3")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002,

    #[cfg(feature = "khx_multiview_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_multiview`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_multiview)
    const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX = 1000053000,

    #[cfg(feature = "khx_multiview_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_multiview`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_multiview)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX = 1000053001,

    #[cfg(feature = "khx_multiview_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_multiview`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_multiview)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX = 1000053002,

    #[cfg(feature = "nv_external_memory_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
    const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000,

    #[cfg(feature = "nv_external_memory_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001,

    #[cfg(feature = "nv_external_memory_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000,

    #[cfg(feature = "nv_external_memory_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001,

    #[cfg(feature = "nv_win32_keyed_mutex_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_win32_keyed_mutex)
    const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR = 1000059000,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR = 1000059001,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR = 1000059002,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059003,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR = 1000059004,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR = 1000059005,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR = 1000059006,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059007,

    #[cfg(feature = "khr_get_physical_device_properties2_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR = 1000059008,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX = 1000060000,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX = 1000060001,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX = 1000060002,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX = 1000060003,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX = 1000060004,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX = 1000060005,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX = 1000060006,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX = 1000060007,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX = 1000060008,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX = 1000060009,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX = 1000060010,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX = 1000060011,

    #[cfg(feature = "khx_device_group_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX = 1000060012,

    #[cfg(feature = "ext_validation_flags_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_validation_flags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_validation_flags)
    const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT = 1000061000,

    #[cfg(feature = "nn_vi_surface_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)
    const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN = 1000062000,

    #[cfg(feature = "khx_device_group_creation_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX = 1000070000,

    #[cfg(feature = "khx_device_group_creation_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
    const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX = 1000070001,

    #[cfg(feature = "khx_external_memory_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHX = 1000071000,

    #[cfg(feature = "khx_external_memory_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHX = 1000071001,

    #[cfg(feature = "khx_external_memory_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHX = 1000071002,

    #[cfg(feature = "khx_external_memory_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHX = 1000071003,

    #[cfg(feature = "khx_external_memory_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_capabilities)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHX = 1000071004,

    #[cfg(feature = "khx_external_memory_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
    const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHX = 1000072000,

    #[cfg(feature = "khx_external_memory_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
    const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHX = 1000072001,

    #[cfg(feature = "khx_external_memory_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory)
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHX = 1000072002,

    #[cfg(feature = "khx_external_memory_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHX = 1000073000,

    #[cfg(feature = "khx_external_memory_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHX = 1000073001,

    #[cfg(feature = "khx_external_memory_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_win32)
    const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHX = 1000073002,

    #[cfg(feature = "khx_external_memory_fd_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
    const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHX = 1000074000,

    #[cfg(feature = "khx_external_memory_fd_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_memory_fd)
    const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHX = 1000074001,

    #[cfg(feature = "khx_win32_keyed_mutex_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_win32_keyed_mutex)
    const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHX = 1000075000,

    #[cfg(feature = "khx_external_semaphore_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHX = 1000076000,

    #[cfg(feature = "khx_external_semaphore_capabilities_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_capabilities)
    const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHX = 1000076001,

    #[cfg(feature = "khx_external_semaphore_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore)
    const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHX = 1000077000,

    #[cfg(feature = "khx_external_semaphore_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX = 1000078000,

    #[cfg(feature = "khx_external_semaphore_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHX = 1000078001,

    #[cfg(feature = "khx_external_semaphore_win32_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_win32)
    const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHX = 1000078002,

    #[cfg(feature = "khx_external_semaphore_fd_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHX_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_external_semaphore_fd)
    const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHX = 1000079000,

    #[cfg(feature = "khr_push_descriptor_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR = 1000080000,

    #[cfg(feature = "khr_incremental_present_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_incremental_present`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_incremental_present)
    const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR = 1000084000,

    #[cfg(feature = "khr_descriptor_update_template_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR = 1000085000,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX = 1000086000,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX = 1000086001,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX = 1000086002,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX = 1000086003,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX = 1000086004,

    #[cfg(feature = "nvx_device_generated_commands_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX = 1000086005,

    #[cfg(feature = "nv_clip_space_w_scaling_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV = 1000087000,

    #[cfg(feature = "ext_display_surface_counter_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
    const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT = 1000090000,

    #[cfg(feature = "ext_display_control_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT = 1000091000,

    #[cfg(feature = "ext_display_control_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT = 1000091001,

    #[cfg(feature = "ext_display_control_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT = 1000091002,

    #[cfg(feature = "ext_display_control_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
    const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT = 1000091003,

    #[cfg(feature = "google_display_timing_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
    const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE = 1000092000,

    #[cfg(feature = "nvx_multiview_per_view_attributes_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NVX_multiview_per_view_attributes`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_multiview_per_view_attributes)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX = 1000097000,

    #[cfg(feature = "nv_viewport_swizzle_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_NV_viewport_swizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_viewport_swizzle)
    const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV = 1000098000,

    #[cfg(feature = "ext_discard_rectangles_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT = 1000099000,

    #[cfg(feature = "ext_discard_rectangles_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
    const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT = 1000099001,

    #[cfg(feature = "ext_hdr_metadata_1")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
    const VK_STRUCTURE_TYPE_HDR_METADATA_EXT = 1000105000,

    #[cfg(feature = "mvk_ios_surface_2")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)
    const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK = 1000122000,

    #[cfg(feature = "mvk_macos_surface_2")]
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    /// and extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
    const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK = 1000123000,
});

cenum!(VkSystemAllocationScope: u32 {
    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,

    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,

    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    const VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,

    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,

    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
});

cenum!(VkInternalAllocationType: u32 {
    /// See [`VkInternalAllocationType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInternalAllocationType)
    const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
});

cenum!(VkFormat: u32 {
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_UNDEFINED = 0,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R4G4_UNORM_PACK8 = 1,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_UNORM = 9,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_SNORM = 10,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_USCALED = 11,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_SSCALED = 12,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_UINT = 13,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_SINT = 14,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8_SRGB = 15,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_UNORM = 16,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_SNORM = 17,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_USCALED = 18,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_SSCALED = 19,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_UINT = 20,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_SINT = 21,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8_SRGB = 22,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_UNORM = 23,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_SNORM = 24,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_USCALED = 25,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_SSCALED = 26,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_UINT = 27,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_SINT = 28,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8_SRGB = 29,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_UNORM = 30,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_SNORM = 31,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_USCALED = 32,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_SSCALED = 33,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_UINT = 34,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_SINT = 35,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8_SRGB = 36,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_UNORM = 37,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_SNORM = 38,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_USCALED = 39,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_SSCALED = 40,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_UINT = 41,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_SINT = 42,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R8G8B8A8_SRGB = 43,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_UNORM = 44,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_SNORM = 45,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_USCALED = 46,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_SSCALED = 47,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_UINT = 48,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_SINT = 49,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B8G8R8A8_SRGB = 50,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_UNORM = 70,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_SNORM = 71,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_USCALED = 72,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_SSCALED = 73,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_UINT = 74,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_SINT = 75,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16_SFLOAT = 76,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_UNORM = 77,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_SNORM = 78,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_USCALED = 79,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_SSCALED = 80,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_UINT = 81,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_SINT = 82,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16_SFLOAT = 83,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_UNORM = 84,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_SNORM = 85,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_USCALED = 86,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_SSCALED = 87,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_UINT = 88,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_SINT = 89,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16_SFLOAT = 90,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_UNORM = 91,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_SNORM = 92,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_USCALED = 93,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_SSCALED = 94,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_UINT = 95,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_SINT = 96,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R16G16B16A16_SFLOAT = 97,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32_UINT = 98,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32_SINT = 99,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32_SFLOAT = 100,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32_UINT = 101,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32_SINT = 102,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32_SFLOAT = 103,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32_UINT = 104,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32_SINT = 105,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32_SFLOAT = 106,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32A32_UINT = 107,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32A32_SINT = 108,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R32G32B32A32_SFLOAT = 109,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64_UINT = 110,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64_SINT = 111,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64_SFLOAT = 112,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64_UINT = 113,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64_SINT = 114,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64_SFLOAT = 115,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64_UINT = 116,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64_SINT = 117,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64_SFLOAT = 118,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64A64_UINT = 119,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64A64_SINT = 120,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_R64G64B64A64_SFLOAT = 121,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_D16_UNORM = 124,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_X8_D24_UNORM_PACK32 = 125,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_D32_SFLOAT = 126,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_S8_UINT = 127,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_D16_UNORM_S8_UINT = 128,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_D24_UNORM_S8_UINT = 129,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_D32_SFLOAT_S8_UINT = 130,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC2_UNORM_BLOCK = 135,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC2_SRGB_BLOCK = 136,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC3_UNORM_BLOCK = 137,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC3_SRGB_BLOCK = 138,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC4_UNORM_BLOCK = 139,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC4_SNORM_BLOCK = 140,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC5_UNORM_BLOCK = 141,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC5_SNORM_BLOCK = 142,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC7_UNORM_BLOCK = 145,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_BC7_SRGB_BLOCK = 146,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,

    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    const VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,

    #[cfg(feature = "img_format_pvrtc_1")]
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    /// and extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
    const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,
});

cenum!(VkImageType: u32 {
    /// See [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageType)
    const VK_IMAGE_TYPE_1D = 0,

    /// See [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageType)
    const VK_IMAGE_TYPE_2D = 1,

    /// See [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageType)
    const VK_IMAGE_TYPE_3D = 2,
});

cenum!(VkImageTiling: u32 {
    /// See [`VkImageTiling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageTiling)
    const VK_IMAGE_TILING_OPTIMAL = 0,

    /// See [`VkImageTiling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageTiling)
    const VK_IMAGE_TILING_LINEAR = 1,
});

cenum!(VkPhysicalDeviceType: u32 {
    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    const VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,

    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,

    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,

    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,

    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    const VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
});

cenum!(VkQueryType: u32 {
    /// See [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryType)
    const VK_QUERY_TYPE_OCCLUSION = 0,

    /// See [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryType)
    const VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,

    /// See [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryType)
    const VK_QUERY_TYPE_TIMESTAMP = 2,
});

cenum!(VkSharingMode: u32 {
    /// See [`VkSharingMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharingMode)
    const VK_SHARING_MODE_EXCLUSIVE = 0,

    /// See [`VkSharingMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharingMode)
    const VK_SHARING_MODE_CONCURRENT = 1,
});

cenum!(VkImageLayout: u32 {
    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_UNDEFINED = 0,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_GENERAL = 1,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,

    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    const VK_IMAGE_LAYOUT_PREINITIALIZED = 8,

    #[cfg(feature = "khr_swapchain_67")]
    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
});

cenum!(VkImageViewType: u32 {
    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_1D = 0,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_2D = 1,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_3D = 2,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_CUBE = 3,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,

    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,
});

cenum!(VkComponentSwizzle: u32 {
    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_IDENTITY = 0,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_ZERO = 1,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_ONE = 2,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_R = 3,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_G = 4,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_B = 5,

    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    const VK_COMPONENT_SWIZZLE_A = 6,
});

cenum!(VkVertexInputRate: u32 {
    /// See [`VkVertexInputRate`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputRate)
    const VK_VERTEX_INPUT_RATE_VERTEX = 0,

    /// See [`VkVertexInputRate`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputRate)
    const VK_VERTEX_INPUT_RATE_INSTANCE = 1,
});

cenum!(VkPrimitiveTopology: u32 {
    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,

    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
});

cenum!(VkPolygonMode: u32 {
    /// See [`VkPolygonMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPolygonMode)
    const VK_POLYGON_MODE_FILL = 0,

    /// See [`VkPolygonMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPolygonMode)
    const VK_POLYGON_MODE_LINE = 1,

    /// See [`VkPolygonMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPolygonMode)
    const VK_POLYGON_MODE_POINT = 2,
});

cenum!(VkFrontFace: u32 {
    /// See [`VkFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFrontFace)
    const VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,

    /// See [`VkFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFrontFace)
    const VK_FRONT_FACE_CLOCKWISE = 1,
});

cenum!(VkCompareOp: u32 {
    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_NEVER = 0,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_LESS = 1,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_EQUAL = 2,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_LESS_OR_EQUAL = 3,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_GREATER = 4,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_NOT_EQUAL = 5,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_GREATER_OR_EQUAL = 6,

    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    const VK_COMPARE_OP_ALWAYS = 7,
});

cenum!(VkStencilOp: u32 {
    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_KEEP = 0,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_ZERO = 1,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_REPLACE = 2,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_INVERT = 5,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,

    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    const VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
});

cenum!(VkLogicOp: u32 {
    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_CLEAR = 0,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_AND = 1,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_AND_REVERSE = 2,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_COPY = 3,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_AND_INVERTED = 4,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_NO_OP = 5,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_XOR = 6,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_OR = 7,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_NOR = 8,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_EQUIVALENT = 9,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_INVERT = 10,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_OR_REVERSE = 11,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_COPY_INVERTED = 12,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_OR_INVERTED = 13,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_NAND = 14,

    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    const VK_LOGIC_OP_SET = 15,
});

cenum!(VkBlendFactor: u32 {
    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ZERO = 0,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE = 1,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_SRC_COLOR = 2,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_DST_COLOR = 4,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_SRC_ALPHA = 6,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_DST_ALPHA = 8,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_CONSTANT_COLOR = 10,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_SRC1_COLOR = 15,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_SRC1_ALPHA = 17,

    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
});

cenum!(VkBlendOp: u32 {
    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    const VK_BLEND_OP_ADD = 0,

    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    const VK_BLEND_OP_SUBTRACT = 1,

    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    const VK_BLEND_OP_REVERSE_SUBTRACT = 2,

    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    const VK_BLEND_OP_MIN = 3,

    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    const VK_BLEND_OP_MAX = 4,
});

cenum!(VkDynamicState: u32 {
    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_VIEWPORT = 0,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_SCISSOR = 1,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_LINE_WIDTH = 2,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_DEPTH_BIAS = 3,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,

    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    const VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,

    #[cfg(feature = "nv_clip_space_w_scaling_1")]
    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    /// and extension [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
    const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV = 1000087000,

    #[cfg(feature = "ext_discard_rectangles_1")]
    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    /// and extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#EXT_discard_rectangles)
    const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT = 1000099000,
});

cenum!(VkFilter: u32 {
    /// See [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFilter)
    const VK_FILTER_NEAREST = 0,

    /// See [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFilter)
    const VK_FILTER_LINEAR = 1,

    #[cfg(feature = "img_filter_cubic_1")]
    /// See [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFilter)
    /// and extension [`VK_IMG_filter_cubic`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#IMG_filter_cubic)
    const VK_FILTER_CUBIC_IMG = 1000015000,
});

cenum!(VkSamplerMipmapMode: u32 {
    /// See [`VkSamplerMipmapMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerMipmapMode)
    const VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,

    /// See [`VkSamplerMipmapMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerMipmapMode)
    const VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
});

cenum!(VkSamplerAddressMode: u32 {
    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    const VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,

    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,

    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,

    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,

    #[cfg(any(all(feature = "core_1_0_3", not(feature = "core_1_0_4")),
              all(feature = "core_1_0_4", feature = "khr_sampler_mirror_clamp_to_edge_1")))]
    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    /// and extension [`VK_KHR_sampler_mirror_clamp_to_edge`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_sampler_mirror_clamp_to_edge)
    const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
});

cenum!(VkBorderColor: u32 {
    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,

    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,

    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,

    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,

    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,

    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    const VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
});

cenum!(VkDescriptorType: u32 {
    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_SAMPLER = 0,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,

    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
});

cenum!(VkAttachmentLoadOp: u32 {
    /// See [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentLoadOp)
    const VK_ATTACHMENT_LOAD_OP_LOAD = 0,

    /// See [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentLoadOp)
    const VK_ATTACHMENT_LOAD_OP_CLEAR = 1,

    /// See [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentLoadOp)
    const VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
});

cenum!(VkAttachmentStoreOp: u32 {
    /// See [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentStoreOp)
    const VK_ATTACHMENT_STORE_OP_STORE = 0,

    /// See [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentStoreOp)
    const VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
});

cenum!(VkPipelineBindPoint: u32 {
    /// See [`VkPipelineBindPoint`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineBindPoint)
    const VK_PIPELINE_BIND_POINT_GRAPHICS = 0,

    /// See [`VkPipelineBindPoint`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineBindPoint)
    const VK_PIPELINE_BIND_POINT_COMPUTE = 1,
});

cenum!(VkCommandBufferLevel: u32 {
    /// See [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferLevel)
    const VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,

    /// See [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferLevel)
    const VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
});

cenum!(VkIndexType: u32 {
    /// See [`VkIndexType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndexType)
    const VK_INDEX_TYPE_UINT16 = 0,

    /// See [`VkIndexType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndexType)
    const VK_INDEX_TYPE_UINT32 = 1,
});

cenum!(VkSubpassContents: u32 {
    /// See [`VkSubpassContents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassContents)
    const VK_SUBPASS_CONTENTS_INLINE = 0,

    /// See [`VkSubpassContents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassContents)
    const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
});

bitflags! {
    /// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkInstanceCreateFlags: u32 {
        const VK_INSTANCE_CREATE_DUMMY = 0,
    }
}

/// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
pub type VkInstanceCreateFlagBits = VkInstanceCreateFlags;

bitflags! {
    /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkFormatFeatureFlags: u32 {
        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800,

        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,

        #[cfg(feature = "img_filter_cubic_1")]
        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        /// and extension [`VK_IMG_filter_cubic`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_filter_cubic)
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000,

        #[cfg(feature = "khr_maintenance1_1")]
        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        /// and extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR = 0x00004000,

        #[cfg(feature = "khr_maintenance1_1")]
        /// See [`VkFormatFeatureFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlags)
        /// and extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR = 0x00008000,
    }
}

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags;

bitflags! {
    /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkImageUsageFlags: u32 {
        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,

        /// See [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlags)
        const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
    }
}

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub type VkImageUsageFlagBits = VkImageUsageFlags;

bitflags! {
    /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkImageCreateFlags: u32 {
        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        const VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,

        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,

        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,

        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,

        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010,

        #[cfg(feature = "khx_device_group_1")]
        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_IMAGE_CREATE_BIND_SFR_BIT_KHX = 0x00000040,

        #[cfg(feature = "khr_maintenance1_1")]
        /// See [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlags)
        /// and extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR = 0x00000020,
    }
}

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub type VkImageCreateFlagBits = VkImageCreateFlags;

bitflags! {
    /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSampleCountFlags: u32 {
        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_1_BIT = 0x00000001,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_2_BIT = 0x00000002,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_4_BIT = 0x00000004,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_8_BIT = 0x00000008,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_16_BIT = 0x00000010,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_32_BIT = 0x00000020,

        /// See [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlags)
        const VK_SAMPLE_COUNT_64_BIT = 0x00000040,
    }
}

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub type VkSampleCountFlagBits = VkSampleCountFlags;

bitflags! {
    /// See [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkQueueFlags: u32 {
        /// See [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlags)
        const VK_QUEUE_GRAPHICS_BIT = 0x00000001,

        /// See [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlags)
        const VK_QUEUE_COMPUTE_BIT = 0x00000002,

        /// See [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlags)
        const VK_QUEUE_TRANSFER_BIT = 0x00000004,

        /// See [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlags)
        const VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
    }
}

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub type VkQueueFlagBits = VkQueueFlags;

bitflags! {
    /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkMemoryPropertyFlags: u32 {
        /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
        const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,

        /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
        const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,

        /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
        const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,

        /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
        const VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,

        /// See [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlags)
        const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
    }
}

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags;

bitflags! {
    /// See [`VkMemoryHeapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkMemoryHeapFlags: u32 {
        /// See [`VkMemoryHeapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlags)
        const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,

        #[cfg(feature = "khx_device_group_creation_1")]
        /// See [`VkMemoryHeapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlags)
        /// and extension [`VK_KHX_device_group_creation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group_creation)
        const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX = 0x00000002,
    }
}

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags;

bitflags! {
    /// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDeviceCreateFlags: u32 {
        const VK_DEVICE_CREATE_DUMMY = 0,
    }
}

/// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
pub type VkDeviceCreateFlagBits = VkDeviceCreateFlags;

bitflags! {
    /// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDeviceQueueCreateFlags: u32 {
        const VK_DEVICE_QUEUE_CREATE_DUMMY = 0,
    }
}

/// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags;

bitflags! {
    /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineStageFlags: u32 {
        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,

        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,

        #[cfg(feature = "nvx_device_generated_commands_1")]
        /// See [`VkPipelineStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlags)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX = 0x00020000,
    }
}

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub type VkPipelineStageFlagBits = VkPipelineStageFlags;

bitflags! {
    /// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkMemoryMapFlags: u32 {
        const VK_MEMORY_MAP_DUMMY = 0,
    }
}

/// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
pub type VkMemoryMapFlagBits = VkMemoryMapFlags;

bitflags! {
    /// See [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkImageAspectFlags: u32 {
        /// See [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlags)
        const VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,

        /// See [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlags)
        const VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,

        /// See [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlags)
        const VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,

        /// See [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlags)
        const VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008,
    }
}

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub type VkImageAspectFlagBits = VkImageAspectFlags;

bitflags! {
    /// See [`VkSparseImageFormatFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSparseImageFormatFlags: u32 {
        /// See [`VkSparseImageFormatFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlags)
        const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,

        /// See [`VkSparseImageFormatFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlags)
        const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,

        /// See [`VkSparseImageFormatFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlags)
        const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004,
    }
}

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags;

bitflags! {
    /// See [`VkSparseMemoryBindFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSparseMemoryBindFlags: u32 {
        /// See [`VkSparseMemoryBindFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlags)
        const VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001,
    }
}

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags;

bitflags! {
    /// See [`VkFenceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkFenceCreateFlags: u32 {
        /// See [`VkFenceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlags)
        const VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
    }
}

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub type VkFenceCreateFlagBits = VkFenceCreateFlags;

bitflags! {
    /// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSemaphoreCreateFlags: u32 {
        const VK_SEMAPHORE_CREATE_DUMMY = 0,
    }
}

/// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
pub type VkSemaphoreCreateFlagBits = VkSemaphoreCreateFlags;

bitflags! {
    /// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkEventCreateFlags: u32 {
        const VK_EVENT_CREATE_DUMMY = 0,
    }
}

/// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
pub type VkEventCreateFlagBits = VkEventCreateFlags;

bitflags! {
    /// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkQueryPoolCreateFlags: u32 {
        const VK_QUERY_POOL_CREATE_DUMMY = 0,
    }
}

/// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
pub type VkQueryPoolCreateFlagBits = VkQueryPoolCreateFlags;

bitflags! {
    /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkQueryPipelineStatisticFlags: u32 {
        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,

        /// See [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlags)
        const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
    }
}

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags;

bitflags! {
    /// See [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkQueryResultFlags: u32 {
        /// See [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlags)
        const VK_QUERY_RESULT_64_BIT = 0x00000001,

        /// See [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlags)
        const VK_QUERY_RESULT_WAIT_BIT = 0x00000002,

        /// See [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlags)
        const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,

        /// See [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlags)
        const VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008,
    }
}

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub type VkQueryResultFlagBits = VkQueryResultFlags;

bitflags! {
    /// See [`VkBufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkBufferCreateFlags: u32 {
        /// See [`VkBufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlags)
        const VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,

        /// See [`VkBufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlags)
        const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,

        /// See [`VkBufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlags)
        const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
    }
}

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub type VkBufferCreateFlagBits = VkBufferCreateFlags;

bitflags! {
    /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkBufferUsageFlags: u32 {
        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,

        /// See [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlags)
        const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
    }
}

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub type VkBufferUsageFlagBits = VkBufferUsageFlags;

bitflags! {
    /// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkBufferViewCreateFlags: u32 {
        const VK_BUFFER_VIEW_CREATE_DUMMY = 0,
    }
}

/// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
pub type VkBufferViewCreateFlagBits = VkBufferViewCreateFlags;

bitflags! {
    /// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkImageViewCreateFlags: u32 {
        const VK_IMAGE_VIEW_CREATE_DUMMY = 0,
    }
}

/// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags;

bitflags! {
    /// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkShaderModuleCreateFlags: u32 {
        const VK_SHADER_MODULE_CREATE_DUMMY = 0,
    }
}

/// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags;

bitflags! {
    /// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineCacheCreateFlags: u32 {
        const VK_PIPELINE_CACHE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags;

bitflags! {
    /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineCreateFlags: u32 {
        /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
        const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,

        /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
        const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,

        /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
        const VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004,

        #[cfg(feature = "khx_device_group_1")]
        /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHX = 0x00000008,

        #[cfg(feature = "khx_device_group_1")]
        /// See [`VkPipelineCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlags)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_PIPELINE_CREATE_DISPATCH_BASE_KHX = 0x00000010,
    }
}

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags;

bitflags! {
    /// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineShaderStageCreateFlags: u32 {
        const VK_PIPELINE_SHADER_STAGE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags;

bitflags! {
    /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkShaderStageFlags: u32 {
        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,

        /// See [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlags)
        const VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
    }
}

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub type VkShaderStageFlagBits = VkShaderStageFlags;

bitflags! {
    /// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineVertexInputStateCreateFlags: u32 {
        const VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
pub type VkPipelineVertexInputStateCreateFlagBits = VkPipelineVertexInputStateCreateFlags;

bitflags! {
    /// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineInputAssemblyStateCreateFlags: u32 {
        const VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
pub type VkPipelineInputAssemblyStateCreateFlagBits = VkPipelineInputAssemblyStateCreateFlags;

bitflags! {
    /// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineTessellationStateCreateFlags: u32 {
        const VK_PIPELINE_TESSELLATION_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
pub type VkPipelineTessellationStateCreateFlagBits = VkPipelineTessellationStateCreateFlags;

bitflags! {
    /// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineViewportStateCreateFlags: u32 {
        const VK_PIPELINE_VIEWPORT_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
pub type VkPipelineViewportStateCreateFlagBits = VkPipelineViewportStateCreateFlags;

bitflags! {
    /// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineRasterizationStateCreateFlags: u32 {
        const VK_PIPELINE_RASTERIZATION_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
pub type VkPipelineRasterizationStateCreateFlagBits = VkPipelineRasterizationStateCreateFlags;

bitflags! {
    /// See [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCullModeFlags: u32 {
        /// See [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlags)
        const VK_CULL_MODE_NONE = 0,

        /// See [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlags)
        const VK_CULL_MODE_FRONT_BIT = 0x00000001,

        /// See [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlags)
        const VK_CULL_MODE_BACK_BIT = 0x00000002,

        /// See [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlags)
        const VK_CULL_MODE_FRONT_AND_BACK = 0x00000003,
    }
}

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub type VkCullModeFlagBits = VkCullModeFlags;

bitflags! {
    /// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineMultisampleStateCreateFlags: u32 {
        const VK_PIPELINE_MULTISAMPLE_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
pub type VkPipelineMultisampleStateCreateFlagBits = VkPipelineMultisampleStateCreateFlags;

bitflags! {
    /// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineDepthStencilStateCreateFlags: u32 {
        const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
pub type VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlags;

bitflags! {
    /// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineColorBlendStateCreateFlags: u32 {
        const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub type VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlags;

bitflags! {
    /// See [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkColorComponentFlags: u32 {
        /// See [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlags)
        const VK_COLOR_COMPONENT_R_BIT = 0x00000001,

        /// See [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlags)
        const VK_COLOR_COMPONENT_G_BIT = 0x00000002,

        /// See [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlags)
        const VK_COLOR_COMPONENT_B_BIT = 0x00000004,

        /// See [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlags)
        const VK_COLOR_COMPONENT_A_BIT = 0x00000008,
    }
}

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub type VkColorComponentFlagBits = VkColorComponentFlags;

bitflags! {
    /// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineDynamicStateCreateFlags: u32 {
        const VK_PIPELINE_DYNAMIC_STATE_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
pub type VkPipelineDynamicStateCreateFlagBits = VkPipelineDynamicStateCreateFlags;

bitflags! {
    /// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkPipelineLayoutCreateFlags: u32 {
        const VK_PIPELINE_LAYOUT_CREATE_DUMMY = 0,
    }
}

/// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
pub type VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlags;

bitflags! {
    /// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSamplerCreateFlags: u32 {
        const VK_SAMPLER_CREATE_DUMMY = 0,
    }
}

/// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags;

bitflags! {
    /// See [`VkDescriptorSetLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDescriptorSetLayoutCreateFlags: u32 {
        #[cfg(not(feature = "khr_push_descriptor_1"))]
        const VK_DESCRIPTOR_SET_LAYOUT_CREATE_DUMMY = 0,

        #[cfg(feature = "khr_push_descriptor_1")]
        /// See [`VkDescriptorSetLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlags)
        /// and extension [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
        const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR = 0x00000001,
    }
}

/// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDescriptorPoolCreateFlags: u32 {
        /// See [`VkDescriptorPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlags)
        const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
    }
}

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDescriptorPoolResetFlags: u32 {
        const VK_DESCRIPTOR_POOL_RESET_DUMMY = 0,
    }
}

/// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
pub type VkDescriptorPoolResetFlagBits = VkDescriptorPoolResetFlags;

bitflags! {
    /// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkFramebufferCreateFlags: u32 {
        const VK_FRAMEBUFFER_CREATE_DUMMY = 0,
    }
}

/// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags;

bitflags! {
    /// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkRenderPassCreateFlags: u32 {
        const VK_RENDER_PASS_CREATE_DUMMY = 0x00000001,
    }
}

/// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags;

bitflags! {
    /// See [`VkAttachmentDescriptionFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkAttachmentDescriptionFlags: u32 {
        /// See [`VkAttachmentDescriptionFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlags)
        const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001,
    }
}

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags;

bitflags! {
    /// See [`VkSubpassDescriptionFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkSubpassDescriptionFlags: u32 {
        #[cfg(not(feature = "nvx_multiview_per_view_attributes_1"))]
        const VK_SUBPASS_DESCRIPTION_DUMMY = 0,

        #[cfg(feature = "nvx_multiview_per_view_attributes_1")]
        /// See [`VkSubpassDescriptionFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlags)
        /// and extension [`VK_NVX_multiview_per_view_attributes`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_multiview_per_view_attributes)
        const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX = 0x00000001,

        #[cfg(feature = "nvx_multiview_per_view_attributes_1")]
        /// See [`VkSubpassDescriptionFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlags)
        /// and extension [`VK_NVX_multiview_per_view_attributes`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_multiview_per_view_attributes)
        const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX = 0x00000002,
    }
}

/// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags;

bitflags! {
    /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkAccessFlags: u32 {
        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_INDEX_READ_BIT = 0x00000002,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_SHADER_READ_BIT = 0x00000020,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_HOST_READ_BIT = 0x00002000,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_HOST_WRITE_BIT = 0x00004000,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_MEMORY_READ_BIT = 0x00008000,

        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        const VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,

        #[cfg(feature = "nvx_device_generated_commands_1")]
        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX = 0x00020000,

        #[cfg(feature = "nvx_device_generated_commands_1")]
        /// See [`VkAccessFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlags)
        /// and extension [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
        const VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX = 0x00040000,
    }
}

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub type VkAccessFlagBits = VkAccessFlags;

bitflags! {
    /// See [`VkDependencyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDependencyFlags: u32 {
        /// See [`VkDependencyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlags)
        const VK_DEPENDENCY_BY_REGION_BIT = 0x00000001,

        #[cfg(feature = "khx_multiview_1")]
        /// See [`VkDependencyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlags)
        /// and extension [`VK_KHX_multiview`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_multiview)
        const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHX = 0x00000002,

        #[cfg(feature = "khx_device_group_1")]
        /// See [`VkDependencyFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlags)
        /// and extension [`VK_KHX_device_group`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHX_device_group)
        const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHX = 0x00000004,
    }
}

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub type VkDependencyFlagBits = VkDependencyFlags;

bitflags! {
    /// See [`VkCommandPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCommandPoolCreateFlags: u32 {
        /// See [`VkCommandPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlags)
        const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,

        /// See [`VkCommandPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlags)
        const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    }
}

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags;

bitflags! {
    /// See [`VkCommandPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCommandPoolResetFlags: u32 {
        /// See [`VkCommandPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlags)
        const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags;

bitflags! {
    /// See [`VkCommandBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCommandBufferUsageFlags: u32 {
        /// See [`VkCommandBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlags)
        const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,

        /// See [`VkCommandBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlags)
        const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,

        /// See [`VkCommandBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlags)
        const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,
    }
}

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags;

bitflags! {
    /// See [`VkQueryControlFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkQueryControlFlags: u32 {
        /// See [`VkQueryControlFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlags)
        const VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001,
    }
}

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub type VkQueryControlFlagBits = VkQueryControlFlags;

bitflags! {
    /// See [`VkCommandBufferResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkCommandBufferResetFlags: u32 {
        /// See [`VkCommandBufferResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlags)
        const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags;

bitflags! {
    /// See [`VkStencilFaceFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlags)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkStencilFaceFlags: u32 {
        /// See [`VkStencilFaceFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlags)
        const VK_STENCIL_FACE_FRONT_BIT = 0x00000001,

        /// See [`VkStencilFaceFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlags)
        const VK_STENCIL_FACE_BACK_BIT = 0x00000002,

        /// See [`VkStencilFaceFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlags)
        const VK_STENCIL_FRONT_AND_BACK = 0x00000003,
    }
}

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub type VkStencilFaceFlagBits = VkStencilFaceFlags;

/// See [`PFN_vkAllocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkAllocationFunction)
pub type PFN_vkAllocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;

/// See [`PFN_vkReallocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkReallocationFunction)
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;

/// See [`PFN_vkFreeFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkFreeFunction)
pub type PFN_vkFreeFunction = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);

/// See [`PFN_vkInternalAllocationNotification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkInternalAllocationNotification)
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

/// See [`PFN_vkInternalFreeNotification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkInternalFreeNotification)
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

/// See [`PFN_vkVoidFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkVoidFunction)
pub type PFN_vkVoidFunction = unsafe extern "system" fn();

/// See [`VkApplicationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkApplicationInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

impl Default for VkApplicationInfo {
    fn default() -> Self {
        VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: ptr::null(),
            applicationVersion: Default::default(),
            pEngineName: ptr::null(),
            engineVersion: Default::default(),
            apiVersion: Default::default(),
        }
    }
}

/// See [`VkInstanceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

impl Default for VkInstanceCreateInfo {
    fn default() -> Self {
        VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            pApplicationInfo: ptr::null(),
            enabledLayerCount: Default::default(),
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: Default::default(),
            ppEnabledExtensionNames: ptr::null(),
        }
    }
}

/// See [`VkAllocationCallbacks`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAllocationCallbacks)
#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

impl Copy for VkAllocationCallbacks { }

impl Clone for VkAllocationCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for VkAllocationCallbacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkAllocationCallbacks")
            .field("pUserData", &self.pUserData)
            .field("pfnAllocation", &(self.pfnAllocation as *mut c_void))
            .field("pfnReallocation", &(self.pfnReallocation as *mut c_void))
            .field("pfnFree", &(self.pfnFree as *mut c_void))
            .field("pfnInternalAllocation", &(self.pfnInternalAllocation as *mut c_void))
            .field("pfnInternalFree", &(self.pfnInternalFree as *mut c_void))
            .finish()
    }
}

impl Default for VkAllocationCallbacks {
    fn default() -> Self {
        unsafe {
            VkAllocationCallbacks {
                pUserData: ptr::null_mut(),
                pfnAllocation: mem::transmute(0usize),
                pfnReallocation: mem::transmute(0usize),
                pfnFree: mem::transmute(0usize),
                pfnInternalAllocation: mem::transmute(0usize),
                pfnInternalFree: mem::transmute(0usize),
            }
        }
    }
}

/// See [`VkPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceFeatures)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

/// See [`VkFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

/// See [`VkExtent3D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtent3D)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// See [`VkImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageFormatProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

/// See [`VkPhysicalDeviceLimits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceLimits)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

/// See [`VkPhysicalDeviceSparseProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSparseProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

/// See [`VkPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceProperties)
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

impl Copy for VkPhysicalDeviceProperties { }

impl Clone for VkPhysicalDeviceProperties {
    fn clone(&self) -> Self {
        *self
    }
}

struct DeviceNameDebugHelper<'a>(&'a [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]);

impl<'a> fmt::Debug for DeviceNameDebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl fmt::Debug for VkPhysicalDeviceProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkPhysicalDeviceProperties")
            .field("apiVersion", &self.apiVersion)
            .field("driverVersion", &self.driverVersion)
            .field("vendorID", &self.vendorID)
            .field("deviceID", &self.deviceID)
            .field("deviceType", &self.deviceType)
            .field("deviceName", &DeviceNameDebugHelper(&self.deviceName))
            .field("pipelineCacheUUID", &self.pipelineCacheUUID)
            .field("limits", &self.limits)
            .field("sparseProperties", &self.sparseProperties)
            .finish()
    }
}

impl Default for VkPhysicalDeviceProperties {
    fn default() -> Self {
        VkPhysicalDeviceProperties {
            apiVersion: Default::default(),
            driverVersion: Default::default(),
            vendorID: Default::default(),
            deviceID: Default::default(),
            deviceType: Default::default(),
            deviceName: unsafe { mem::zeroed() },
            pipelineCacheUUID: Default::default(),
            limits: Default::default(),
            sparseProperties: Default::default(),
        }
    }
}

/// See [`VkQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFamilyProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

/// See [`VkMemoryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryType)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

/// See [`VkMemoryHeap`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeap)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

/// See [`VkPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMemoryProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

/// See [`VkDeviceQueueCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queueFamilyIndex: Default::default(),
            queueCount: Default::default(),
            pQueuePriorities: ptr::null(),
        }
    }
}

/// See [`VkDeviceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

impl Default for VkDeviceCreateInfo {
    fn default() -> Self {
        VkDeviceCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queueCreateInfoCount: Default::default(),
            pQueueCreateInfos: ptr::null(),
            enabledLayerCount: Default::default(),
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: Default::default(),
            ppEnabledExtensionNames: ptr::null(),
            pEnabledFeatures: ptr::null(),
        }
    }
}

/// See [`VkExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtensionProperties)
#[repr(C)]
pub struct VkExtensionProperties {
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
}

impl Copy for VkExtensionProperties { }

impl Clone for VkExtensionProperties {
    fn clone(&self) -> Self {
        *self
    }
}

struct ExtensionNameDebugHelper<'a>(&'a [c_char; VK_MAX_EXTENSION_NAME_SIZE]);

impl<'a> fmt::Debug for ExtensionNameDebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl fmt::Debug for VkExtensionProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkExtensionProperties")
            .field("extensionName", &ExtensionNameDebugHelper(&self.extensionName))
            .field("specVersion", &self.specVersion)
            .finish()
    }
}

impl Default for VkExtensionProperties {
    fn default() -> Self {
        VkExtensionProperties {
            extensionName: unsafe { mem::zeroed() },
            specVersion: Default::default(),
        }
    }
}

/// See [`VkLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLayerProperties)
#[repr(C)]
pub struct VkLayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
}

impl Copy for VkLayerProperties { }

impl Clone for VkLayerProperties {
    fn clone(&self) -> Self {
        *self
    }
}

struct LayerNameDebugHelper<'a>(&'a [c_char; VK_MAX_EXTENSION_NAME_SIZE]);

impl<'a> fmt::Debug for LayerNameDebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

struct DescriptionDebugHelper<'a>(&'a [c_char; VK_MAX_DESCRIPTION_SIZE]);

impl<'a> fmt::Debug for DescriptionDebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl fmt::Debug for VkLayerProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkLayerProperties")
            .field("layerName", &LayerNameDebugHelper(&self.layerName))
            .field("specVersion", &self.specVersion)
            .field("implementationVersion", &self.implementationVersion)
            .field("description", &DescriptionDebugHelper(&self.description))
            .finish()
    }
}

impl Default for VkLayerProperties {
    fn default() -> Self {
        VkLayerProperties {
            layerName: unsafe { mem::zeroed() },
            specVersion: Default::default(),
            implementationVersion: Default::default(),
            description: unsafe { mem::zeroed() },
        }
    }
}

/// See [`VkSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubmitInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

impl Default for VkSubmitInfo {
    fn default() -> Self {
        VkSubmitInfo {
            sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: Default::default(),
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: Default::default(),
            pCommandBuffers: ptr::null(),
            signalSemaphoreCount: Default::default(),
            pSignalSemaphores: ptr::null(),
        }
    }
}

/// See [`VkMemoryAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

impl Default for VkMemoryAllocateInfo {
    fn default() -> Self {
        VkMemoryAllocateInfo {
            sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: ptr::null(),
            allocationSize: Default::default(),
            memoryTypeIndex: Default::default(),
        }
    }
}

/// See [`VkMappedMemoryRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMappedMemoryRange)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkMappedMemoryRange {
    fn default() -> Self {
        VkMappedMemoryRange {
            sType: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: ptr::null(),
            memory: ptr::null_mut(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}

/// See [`VkMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryRequirements)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

/// See [`VkSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatProperties)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

/// See [`VkSparseImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryRequirements)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

/// See [`VkSparseMemoryBind`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBind)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

impl Default for VkSparseMemoryBind {
    fn default() -> Self {
        VkSparseMemoryBind {
            resourceOffset: Default::default(),
            size: Default::default(),
            memory: ptr::null_mut(),
            memoryOffset: Default::default(),
            flags: Default::default(),
        }
    }
}

/// See [`VkSparseBufferMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseBufferMemoryBindInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

impl Default for VkSparseBufferMemoryBindInfo {
    fn default() -> Self {
        VkSparseBufferMemoryBindInfo {
            buffer: ptr::null_mut(),
            bindCount: Default::default(),
            pBinds: ptr::null(),
        }
    }
}

/// See [`VkSparseImageOpaqueMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageOpaqueMemoryBindInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

impl Default for VkSparseImageOpaqueMemoryBindInfo {
    fn default() -> Self {
        VkSparseImageOpaqueMemoryBindInfo {
            image: ptr::null_mut(),
            bindCount: Default::default(),
            pBinds: ptr::null(),
        }
    }
}

/// See [`VkImageSubresource`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresource)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

/// See [`VkOffset3D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkOffset3D)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// See [`VkSparseImageMemoryBind`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryBind)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

impl Default for VkSparseImageMemoryBind {
    fn default() -> Self {
        VkSparseImageMemoryBind {
            subresource: Default::default(),
            offset: Default::default(),
            extent: Default::default(),
            memory: ptr::null_mut(),
            memoryOffset: Default::default(),
            flags: Default::default(),
        }
    }
}

/// See [`VkSparseImageMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryBindInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

impl Default for VkSparseImageMemoryBindInfo {
    fn default() -> Self {
        VkSparseImageMemoryBindInfo {
            image: ptr::null_mut(),
            bindCount: Default::default(),
            pBinds: ptr::null(),
        }
    }
}

/// See [`VkBindSparseInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindSparseInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

impl Default for VkBindSparseInfo {
    fn default() -> Self {
        VkBindSparseInfo {
            sType: VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: Default::default(),
            pWaitSemaphores: ptr::null(),
            bufferBindCount: Default::default(),
            pBufferBinds: ptr::null(),
            imageOpaqueBindCount: Default::default(),
            pImageOpaqueBinds: ptr::null(),
            imageBindCount: Default::default(),
            pImageBinds: ptr::null(),
            signalSemaphoreCount: Default::default(),
            pSignalSemaphores: ptr::null(),
        }
    }
}

/// See [`VkFenceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

impl Default for VkFenceCreateInfo {
    fn default() -> Self {
        VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
        }
    }
}

/// See [`VkSemaphoreCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

impl Default for VkSemaphoreCreateInfo {
    fn default() -> Self {
        VkSemaphoreCreateInfo {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
        }
    }
}

/// See [`VkEventCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlags,
}

impl Default for VkEventCreateInfo {
    fn default() -> Self {
        VkEventCreateInfo {
            sType: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
        }
    }
}

/// See [`VkQueryPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

impl Default for VkQueryPoolCreateInfo {
    fn default() -> Self {
        VkQueryPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queryType: Default::default(),
            queryCount: Default::default(),
            pipelineStatistics: Default::default(),
        }
    }
}

/// See [`VkBufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

impl Default for VkBufferCreateInfo {
    fn default() -> Self {
        VkBufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            size: Default::default(),
            usage: Default::default(),
            sharingMode: Default::default(),
            queueFamilyIndexCount: Default::default(),
            pQueueFamilyIndices: ptr::null(),
        }
    }
}

/// See [`VkBufferViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

impl Default for VkBufferViewCreateInfo {
    fn default() -> Self {
        VkBufferViewCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            buffer: ptr::null_mut(),
            format: Default::default(),
            offset: Default::default(),
            range: Default::default(),
        }
    }
}

/// See [`VkImageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

impl Default for VkImageCreateInfo {
    fn default() -> Self {
        VkImageCreateInfo {
            sType: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            imageType: Default::default(),
            format: Default::default(),
            extent: Default::default(),
            mipLevels: Default::default(),
            arrayLayers: Default::default(),
            samples: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            sharingMode: Default::default(),
            queueFamilyIndexCount: Default::default(),
            pQueueFamilyIndices: ptr::null(),
            initialLayout: Default::default(),
        }
    }
}

/// See [`VkSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubresourceLayout)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

/// See [`VkComponentMapping`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentMapping)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

/// See [`VkImageSubresourceRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresourceRange)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

/// See [`VkImageViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

impl Default for VkImageViewCreateInfo {
    fn default() -> Self {
        VkImageViewCreateInfo {
            sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            image: ptr::null_mut(),
            viewType: Default::default(),
            format: Default::default(),
            components: Default::default(),
            subresourceRange: Default::default(),
        }
    }
}

/// See [`VkShaderModuleCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

impl Default for VkShaderModuleCreateInfo {
    fn default() -> Self {
        VkShaderModuleCreateInfo {
            sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            codeSize: Default::default(),
            pCode: ptr::null(),
        }
    }
}

/// See [`VkPipelineCacheCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

impl Default for VkPipelineCacheCreateInfo {
    fn default() -> Self {
        VkPipelineCacheCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            initialDataSize: Default::default(),
            pInitialData: ptr::null(),
        }
    }
}

/// See [`VkSpecializationMapEntry`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSpecializationMapEntry)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

/// See [`VkSpecializationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSpecializationInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

impl Default for VkSpecializationInfo {
    fn default() -> Self {
        VkSpecializationInfo {
            mapEntryCount: Default::default(),
            pMapEntries: ptr::null(),
            dataSize: Default::default(),
            pData: ptr::null(),
        }
    }
}

/// See [`VkPipelineShaderStageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

impl Default for VkPipelineShaderStageCreateInfo {
    fn default() -> Self {
        VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            module: ptr::null_mut(),
            pName: ptr::null(),
            pSpecializationInfo: ptr::null(),
        }
    }
}

/// See [`VkVertexInputBindingDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputBindingDescription)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

/// See [`VkVertexInputAttributeDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputAttributeDescription)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

/// See [`VkPipelineVertexInputStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

impl Default for VkPipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        VkPipelineVertexInputStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            vertexBindingDescriptionCount: Default::default(),
            pVertexBindingDescriptions: ptr::null(),
            vertexAttributeDescriptionCount: Default::default(),
            pVertexAttributeDescriptions: ptr::null(),
        }
    }
}

/// See [`VkPipelineInputAssemblyStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

impl Default for VkPipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        VkPipelineInputAssemblyStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            topology: Default::default(),
            primitiveRestartEnable: Default::default(),
        }
    }
}

/// See [`VkPipelineTessellationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

impl Default for VkPipelineTessellationStateCreateInfo {
    fn default() -> Self {
        VkPipelineTessellationStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            patchControlPoints: Default::default(),
        }
    }
}

/// See [`VkViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViewport)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

/// See [`VkOffset2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkOffset2D)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

/// See [`VkExtent2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtent2D)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

/// See [`VkRect2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRect2D)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

/// See [`VkPipelineViewportStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

impl Default for VkPipelineViewportStateCreateInfo {
    fn default() -> Self {
        VkPipelineViewportStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            viewportCount: Default::default(),
            pViewports: ptr::null(),
            scissorCount: Default::default(),
            pScissors: ptr::null(),
        }
    }
}

/// See [`VkPipelineRasterizationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

impl Default for VkPipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        VkPipelineRasterizationStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            depthClampEnable: Default::default(),
            rasterizerDiscardEnable: Default::default(),
            polygonMode: Default::default(),
            cullMode: Default::default(),
            frontFace: Default::default(),
            depthBiasEnable: Default::default(),
            depthBiasConstantFactor: Default::default(),
            depthBiasClamp: Default::default(),
            depthBiasSlopeFactor: Default::default(),
            lineWidth: Default::default(),
        }
    }
}

/// See [`VkPipelineMultisampleStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

impl Default for VkPipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        VkPipelineMultisampleStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            rasterizationSamples: Default::default(),
            sampleShadingEnable: Default::default(),
            minSampleShading: Default::default(),
            pSampleMask: ptr::null(),
            alphaToCoverageEnable: Default::default(),
            alphaToOneEnable: Default::default(),
        }
    }
}

/// See [`VkStencilOpState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOpState)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

/// See [`VkPipelineDepthStencilStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

impl Default for VkPipelineDepthStencilStateCreateInfo {
    fn default() -> Self {
        VkPipelineDepthStencilStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            depthTestEnable: Default::default(),
            depthWriteEnable: Default::default(),
            depthCompareOp: Default::default(),
            depthBoundsTestEnable: Default::default(),
            stencilTestEnable: Default::default(),
            front: Default::default(),
            back: Default::default(),
            minDepthBounds: Default::default(),
            maxDepthBounds: Default::default(),
        }
    }
}

/// See [`VkPipelineColorBlendAttachmentState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendAttachmentState)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

/// See [`VkPipelineColorBlendStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

impl Default for VkPipelineColorBlendStateCreateInfo {
    fn default() -> Self {
        VkPipelineColorBlendStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            logicOpEnable: Default::default(),
            logicOp: Default::default(),
            attachmentCount: Default::default(),
            pAttachments: ptr::null(),
            blendConstants: Default::default(),
        }
    }
}

/// See [`VkPipelineDynamicStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

impl Default for VkPipelineDynamicStateCreateInfo {
    fn default() -> Self {
        VkPipelineDynamicStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            dynamicStateCount: Default::default(),
            pDynamicStates: ptr::null(),
        }
    }
}

/// See [`VkGraphicsPipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkGraphicsPipelineCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl Default for VkGraphicsPipelineCreateInfo {
    fn default() -> Self {
        VkGraphicsPipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            stageCount: Default::default(),
            pStages: ptr::null(),
            pVertexInputState: ptr::null(),
            pInputAssemblyState: ptr::null(),
            pTessellationState: ptr::null(),
            pViewportState: ptr::null(),
            pRasterizationState: ptr::null(),
            pMultisampleState: ptr::null(),
            pDepthStencilState: ptr::null(),
            pColorBlendState: ptr::null(),
            pDynamicState: ptr::null(),
            layout: ptr::null_mut(),
            renderPass: ptr::null_mut(),
            subpass: Default::default(),
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: Default::default(),
        }
    }
}

/// See [`VkComputePipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComputePipelineCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl Default for VkComputePipelineCreateInfo {
    fn default() -> Self {
        VkComputePipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            layout: ptr::null_mut(),
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: Default::default(),
        }
    }
}

/// See [`VkPushConstantRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPushConstantRange)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

/// See [`VkPipelineLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

impl Default for VkPipelineLayoutCreateInfo {
    fn default() -> Self {
        VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            setLayoutCount: Default::default(),
            pSetLayouts: ptr::null(),
            pushConstantRangeCount: Default::default(),
            pPushConstantRanges: ptr::null(),
        }
    }
}

/// See [`VkSamplerCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: f32,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: f32,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

impl Default for VkSamplerCreateInfo {
    fn default() -> Self {
        VkSamplerCreateInfo {
            sType: VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            magFilter: Default::default(),
            minFilter: Default::default(),
            mipmapMode: Default::default(),
            addressModeU: Default::default(),
            addressModeV: Default::default(),
            addressModeW: Default::default(),
            mipLodBias: Default::default(),
            anisotropyEnable: Default::default(),
            maxAnisotropy: Default::default(),
            compareEnable: Default::default(),
            compareOp: Default::default(),
            minLod: Default::default(),
            maxLod: Default::default(),
            borderColor: Default::default(),
            unnormalizedCoordinates: Default::default(),
        }
    }
}

/// See [`VkDescriptorSetLayoutBinding`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutBinding)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

impl Default for VkDescriptorSetLayoutBinding {
    fn default() -> Self {
        VkDescriptorSetLayoutBinding {
            binding: Default::default(),
            descriptorType: Default::default(),
            descriptorCount: Default::default(),
            stageFlags: Default::default(),
            pImmutableSamplers: ptr::null(),
        }
    }
}

/// See [`VkDescriptorSetLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

impl Default for VkDescriptorSetLayoutCreateInfo {
    fn default() -> Self {
        VkDescriptorSetLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            bindingCount: Default::default(),
            pBindings: ptr::null(),
        }
    }
}

/// See [`VkDescriptorPoolSize`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolSize)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDescriptorPoolSize {
    pub type_: VkDescriptorType,
    pub descriptorCount: u32,
}

/// See [`VkDescriptorPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

impl Default for VkDescriptorPoolCreateInfo {
    fn default() -> Self {
        VkDescriptorPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            maxSets: Default::default(),
            poolSizeCount: Default::default(),
            pPoolSizes: ptr::null(),
        }
    }
}

/// See [`VkDescriptorSetAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetAllocateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

impl Default for VkDescriptorSetAllocateInfo {
    fn default() -> Self {
        VkDescriptorSetAllocateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: ptr::null(),
            descriptorPool: ptr::null_mut(),
            descriptorSetCount: Default::default(),
            pSetLayouts: ptr::null(),
        }
    }
}

/// See [`VkDescriptorImageInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorImageInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

impl Default for VkDescriptorImageInfo {
    fn default() -> Self {
        VkDescriptorImageInfo {
            sampler: ptr::null_mut(),
            imageView: ptr::null_mut(),
            imageLayout: Default::default(),
        }
    }
}

/// See [`VkDescriptorBufferInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorBufferInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

impl Default for VkDescriptorBufferInfo {
    fn default() -> Self {
        VkDescriptorBufferInfo {
            buffer: ptr::null_mut(),
            offset: Default::default(),
            range: Default::default(),
        }
    }
}

/// See [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWriteDescriptorSet)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

impl Default for VkWriteDescriptorSet {
    fn default() -> Self {
        VkWriteDescriptorSet {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: ptr::null_mut(),
            dstBinding: Default::default(),
            dstArrayElement: Default::default(),
            descriptorCount: Default::default(),
            descriptorType: Default::default(),
            pImageInfo: ptr::null(),
            pBufferInfo: ptr::null(),
            pTexelBufferView: ptr::null(),
        }
    }
}

/// See [`VkCopyDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCopyDescriptorSet)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

impl Default for VkCopyDescriptorSet {
    fn default() -> Self {
        VkCopyDescriptorSet {
            sType: VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
            pNext: ptr::null(),
            srcSet: ptr::null_mut(),
            srcBinding: Default::default(),
            srcArrayElement: Default::default(),
            dstSet: ptr::null_mut(),
            dstBinding: Default::default(),
            dstArrayElement: Default::default(),
            descriptorCount: Default::default(),
        }
    }
}

/// See [`VkFramebufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl Default for VkFramebufferCreateInfo {
    fn default() -> Self {
        VkFramebufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            renderPass: ptr::null_mut(),
            attachmentCount: Default::default(),
            pAttachments: ptr::null(),
            width: Default::default(),
            height: Default::default(),
            layers: Default::default(),
        }
    }
}

/// See [`VkAttachmentDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescription)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

/// See [`VkAttachmentReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentReference)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

/// See [`VkSubpassDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescription)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

impl Default for VkSubpassDescription {
    fn default() -> Self {
        VkSubpassDescription {
            flags: Default::default(),
            pipelineBindPoint: Default::default(),
            inputAttachmentCount: Default::default(),
            pInputAttachments: ptr::null(),
            colorAttachmentCount: Default::default(),
            pColorAttachments: ptr::null(),
            pResolveAttachments: ptr::null(),
            pDepthStencilAttachment: ptr::null(),
            preserveAttachmentCount: Default::default(),
            pPreserveAttachments: ptr::null(),
        }
    }
}

/// See [`VkSubpassDependency`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDependency)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

/// See [`VkRenderPassCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

impl Default for VkRenderPassCreateInfo {
    fn default() -> Self {
        VkRenderPassCreateInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            attachmentCount: Default::default(),
            pAttachments: ptr::null(),
            subpassCount: Default::default(),
            pSubpasses: ptr::null(),
            dependencyCount: Default::default(),
            pDependencies: ptr::null(),
        }
    }
}

/// See [`VkCommandPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

impl Default for VkCommandPoolCreateInfo {
    fn default() -> Self {
        VkCommandPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queueFamilyIndex: Default::default(),
        }
    }
}

/// See [`VkCommandBufferAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferAllocateInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

impl Default for VkCommandBufferAllocateInfo {
    fn default() -> Self {
        VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: ptr::null_mut(),
            level: Default::default(),
            commandBufferCount: Default::default(),
        }
    }
}

/// See [`VkCommandBufferInheritanceInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferInheritanceInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

impl Default for VkCommandBufferInheritanceInfo {
    fn default() -> Self {
        VkCommandBufferInheritanceInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
            pNext: ptr::null(),
            renderPass: ptr::null_mut(),
            subpass: Default::default(),
            framebuffer: ptr::null_mut(),
            occlusionQueryEnable: Default::default(),
            queryFlags: Default::default(),
            pipelineStatistics: Default::default(),
        }
    }
}

/// See [`VkCommandBufferBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferBeginInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

impl Default for VkCommandBufferBeginInfo {
    fn default() -> Self {
        VkCommandBufferBeginInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            pInheritanceInfo: ptr::null(),
        }
    }
}

/// See [`VkBufferCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCopy)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

/// See [`VkImageSubresourceLayers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresourceLayers)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

/// See [`VkImageCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCopy)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

/// See [`VkImageBlit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageBlit)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

/// See [`VkBufferImageCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferImageCopy)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[cfg(feature = "unstable_rust")]
/// See [`VkClearColorValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearColorValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[cfg(feature = "unstable_rust")]
impl fmt::Debug for VkClearColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.debug_struct("VkClearColorValue")
                .field("float32", &self.float32)
                .field("int32", &self.int32)
                .field("uint32", &self.uint32)
                .finish()
        }
    }
}

#[cfg(feature = "unstable_rust")]
impl Default for VkClearColorValue {
    fn default() -> Self {
        VkClearColorValue {
            uint32: Default::default(),
        }
    }
}

#[cfg(not(feature = "unstable_rust"))]
/// See [`VkClearColorValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearColorValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearColorValue {
    pub float32: VkSysUnionField<[f32; 4usize]>,
    pub int32: VkSysUnionField<[i32; 4usize]>,
    pub uint32: VkSysUnionField<[u32; 4usize]>,
    pub vks_union_field: [u32; 4usize],
}

#[cfg(not(feature = "unstable_rust"))]
impl fmt::Debug for VkClearColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkClearColorValue")
            .field("vks_union_field", &self.vks_union_field)
            .finish()
    }
}

#[cfg(not(feature = "unstable_rust"))]
impl Default for VkClearColorValue {
    fn default() -> Self {
        VkClearColorValue {
            float32: VkSysUnionField::new(),
            int32: VkSysUnionField::new(),
            uint32: VkSysUnionField::new(),
            vks_union_field: Default::default(),
        }
    }
}

/// See [`VkClearDepthStencilValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearDepthStencilValue)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[cfg(feature = "unstable_rust")]
/// See [`VkClearValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[cfg(feature = "unstable_rust")]
impl fmt::Debug for VkClearValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.debug_struct("VkClearValue")
                .field("color", &self.color)
                .field("depthStencil", &self.depthStencil)
                .finish()
        }
    }
}

#[cfg(feature = "unstable_rust")]
impl Default for VkClearValue {
    fn default() -> Self {
        VkClearValue {
            color: Default::default(),
        }
    }
}

#[cfg(not(feature = "unstable_rust"))]
/// See [`VkClearValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearValue {
    pub color: VkSysUnionField<VkClearColorValue>,
    pub depthStencil: VkSysUnionField<VkClearDepthStencilValue>,
    pub vks_union_field: [u32; 4usize],
}

#[cfg(not(feature = "unstable_rust"))]
impl fmt::Debug for VkClearValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkClearValue")
            .field("vks_union_field", &self.vks_union_field)
            .finish()
    }
}

#[cfg(not(feature = "unstable_rust"))]
impl Default for VkClearValue {
    fn default() -> Self {
        VkClearValue {
            color: VkSysUnionField::new(),
            depthStencil: VkSysUnionField::new(),
            vks_union_field: Default::default(),
        }
    }
}

/// See [`VkClearAttachment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearAttachment)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

/// See [`VkClearRect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearRect)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

/// See [`VkImageResolve`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageResolve)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

/// See [`VkMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryBarrier)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

impl Default for VkMemoryBarrier {
    fn default() -> Self {
        VkMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: Default::default(),
            dstAccessMask: Default::default(),
        }
    }
}

/// See [`VkBufferMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferMemoryBarrier)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkBufferMemoryBarrier {
    fn default() -> Self {
        VkBufferMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: Default::default(),
            dstAccessMask: Default::default(),
            srcQueueFamilyIndex: Default::default(),
            dstQueueFamilyIndex: Default::default(),
            buffer: ptr::null_mut(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}

/// See [`VkImageMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageMemoryBarrier)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

impl Default for VkImageMemoryBarrier {
    fn default() -> Self {
        VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: Default::default(),
            dstAccessMask: Default::default(),
            oldLayout: Default::default(),
            newLayout: Default::default(),
            srcQueueFamilyIndex: Default::default(),
            dstQueueFamilyIndex: Default::default(),
            image: ptr::null_mut(),
            subresourceRange: Default::default(),
        }
    }
}

/// See [`VkRenderPassBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassBeginInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

impl Default for VkRenderPassBeginInfo {
    fn default() -> Self {
        VkRenderPassBeginInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: ptr::null_mut(),
            framebuffer: ptr::null_mut(),
            renderArea: Default::default(),
            clearValueCount: Default::default(),
            pClearValues: ptr::null(),
        }
    }
}

/// See [`VkDispatchIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDispatchIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// See [`VkDrawIndexedIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndexedIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

/// See [`VkDrawIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

/// See [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
pub type PFN_vkCreateInstance = unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;

/// See [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyInstance)
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);

/// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

/// See [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);

/// See [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);

/// See [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;

/// See [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);

/// See [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

/// See [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);

/// See [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetInstanceProcAddr)
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;

/// See [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;

/// See [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
pub type PFN_vkCreateDevice = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;

/// See [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);

/// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

/// See [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

/// See [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

/// See [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

/// See [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);

/// See [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;

/// See [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;

/// See [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;

/// See [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;

/// See [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
pub type PFN_vkFreeMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);

/// See [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
pub type PFN_vkMapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;

/// See [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);

/// See [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

/// See [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

/// See [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);

/// See [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

/// See [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

/// See [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);

/// See [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);

/// See [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);

/// See [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);

/// See [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;

/// See [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
pub type PFN_vkCreateFence = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

/// See [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
pub type PFN_vkDestroyFence = unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);

/// See [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
pub type PFN_vkResetFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;

/// See [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;

/// See [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
pub type PFN_vkWaitForFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;

/// See [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;

/// See [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
pub type PFN_vkCreateEvent = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;

/// See [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

/// See [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

/// See [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

/// See [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;

/// See [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;

/// See [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;

/// See [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;

/// See [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
pub type PFN_vkCreateImage = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;

/// See [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
pub type PFN_vkDestroyImage = unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);

/// See [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
pub type PFN_vkCreateImageView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;

/// See [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;

/// See [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;

/// See [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;

/// See [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;

/// See [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;

/// See [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;

/// See [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;

/// See [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
pub type PFN_vkCreateSampler = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;

/// See [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
pub type PFN_vkDestroySampler = unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;

/// See [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;

/// See [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);

/// See [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;

/// See [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;

/// See [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;

/// See [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);

/// See [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;

/// See [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);

/// See [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

/// See [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);

/// See [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);

/// See [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;

/// See [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);

/// See [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;

/// See [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

/// See [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

/// See [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;

/// See [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;

/// See [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

/// See [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);

/// See [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);

/// See [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);

/// See [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);

/// See [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);

/// See [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const f32);

/// See [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);

/// See [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);

/// See [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);

/// See [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);

/// See [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);

/// See [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);

/// See [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);

/// See [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
pub type PFN_vkCmdDraw = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);

/// See [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);

/// See [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

/// See [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

/// See [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

/// See [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);

/// See [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);

/// See [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);

/// See [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);

/// See [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);

/// See [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

#[cfg(feature = "core_1_0_19")]
/// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);

#[cfg(all(feature = "core_1_0_3", not(feature = "core_1_0_19")))]
/// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const u32);

/// See [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);

/// See [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

/// See [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

/// See [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);

/// See [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);

/// See [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

/// See [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

/// See [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);

/// See [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);

/// See [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);

/// See [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);

/// See [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

/// See [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);

/// See [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);

/// See [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);

/// See [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);

/// See [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);

/// See [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

/// See [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;

    /// See [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyInstance)
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

    /// See [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);

    /// See [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);

    /// See [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;

    /// See [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);

    /// See [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

    /// See [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);

    /// See [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetInstanceProcAddr)
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;

    /// See [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;

    /// See [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;

    /// See [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

    /// See [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

    /// See [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

    /// See [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

    /// See [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);

    /// See [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;

    /// See [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;

    /// See [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;

    /// See [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;

    /// See [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;

    /// See [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);

    /// See [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

    /// See [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

    /// See [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
    pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);

    /// See [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

    /// See [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

    /// See [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);

    /// See [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);

    /// See [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
    pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);

    /// See [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);

    /// See [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
    pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;

    /// See [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

    /// See [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;

    /// See [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;

    /// See [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;

    /// See [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;

    /// See [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
    pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;

    /// See [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;

    /// See [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;

    /// See [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;

    /// See [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
    pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;

    /// See [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
    pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;

    /// See [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;

    /// See [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;

    /// See [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;

    /// See [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
    pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);

    /// See [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;

    /// See [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;

    /// See [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
    pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;

    /// See [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;

    /// See [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;

    /// See [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;

    /// See [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
    pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;

    /// See [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;

    /// See [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;

    /// See [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;

    /// See [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;

    /// See [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
    pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;

    /// See [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;

    /// See [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
    pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;

    /// See [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);

    /// See [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;

    /// See [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

    /// See [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);

    /// See [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;

    /// See [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;

    /// See [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

    /// See [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

    /// See [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;

    /// See [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;

    /// See [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

    /// See [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);

    /// See [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
    pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);

    /// See [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
    pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);

    /// See [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32);

    /// See [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
    pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);

    /// See [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const f32);

    /// See [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);

    /// See [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);

    /// See [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);

    /// See [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);

    /// See [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);

    /// See [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);

    /// See [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);

    /// See [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);

    /// See [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);

    /// See [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
    pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

    /// See [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

    /// See [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

    /// See [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);

    /// See [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);

    /// See [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
    pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);

    /// See [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);

    /// See [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);

    /// See [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
    pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

    #[cfg(feature = "core_1_0_19")]
    /// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);

    #[cfg(all(feature = "core_1_0_3", not(feature = "core_1_0_19")))]
    /// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const u32);

    /// See [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
    pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);

    /// See [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

    /// See [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

    /// See [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
    pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);

    /// See [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
    pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);

    /// See [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

    /// See [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

    /// See [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
    pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);

    /// See [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);

    /// See [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
    pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);

    /// See [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);

    /// See [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
    pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

    /// See [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
    pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);

    /// See [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);

    /// See [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);

    /// See [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);

    /// See [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);

    /// See [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);

    /// See [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
}
