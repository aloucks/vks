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

//! [`Core Vulkan specification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html)

use core::fmt;
use core::mem;
use core::ptr;
use libc::{c_char, c_void};

/// See [`VkBool32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBool32)
pub type VkBool32 = u32;

/// See [`VkDeviceSize`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceSize)
pub type VkDeviceSize = u64;

/// See [`VkSampleMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleMask)
pub type VkSampleMask = u32;

define_handle!(
    /// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
    type VkInstance;

    /// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
    struct VkInstance_T;
);

define_handle!(
    /// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
    type VkPhysicalDevice;

    /// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
    struct VkPhysicalDevice_T;
);

define_handle!(
    /// See [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDevice)
    type VkDevice;

    /// See [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDevice)
    struct VkDevice_T;
);

define_handle!(
    /// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
    type VkQueue;

    /// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
    struct VkQueue_T;
);

define_non_dispatchable_handle!(
    /// See [`VkSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphore)
    struct VkSemaphore;
);

define_handle!(
    /// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
    type VkCommandBuffer;

    /// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
    struct VkCommandBuffer_T;
);

define_non_dispatchable_handle!(
    /// See [`VkFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFence)
    struct VkFence;
);

define_non_dispatchable_handle!(
    /// See [`VkDeviceMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceMemory)
    struct VkDeviceMemory;
);

define_non_dispatchable_handle!(
    /// See [`VkBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBuffer)
    struct VkBuffer;
);

define_non_dispatchable_handle!(
    /// See [`VkImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImage)
    struct VkImage;
);

define_non_dispatchable_handle!(
    /// See [`VkEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEvent)
    struct VkEvent;
);

define_non_dispatchable_handle!(
    /// See [`VkQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPool)
    struct VkQueryPool;
);

define_non_dispatchable_handle!(
    /// See [`VkBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferView)
    struct VkBufferView;
);

define_non_dispatchable_handle!(
    /// See [`VkImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageView)
    struct VkImageView;
);

define_non_dispatchable_handle!(
    /// See [`VkShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModule)
    struct VkShaderModule;
);

define_non_dispatchable_handle!(
    /// See [`VkPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCache)
    struct VkPipelineCache;
);

define_non_dispatchable_handle!(
    /// See [`VkPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayout)
    struct VkPipelineLayout;
);

define_non_dispatchable_handle!(
    /// See [`VkRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPass)
    struct VkRenderPass;
);

define_non_dispatchable_handle!(
    /// See [`VkPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipeline)
    struct VkPipeline;
);

define_non_dispatchable_handle!(
    /// See [`VkDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayout)
    struct VkDescriptorSetLayout;
);

define_non_dispatchable_handle!(
    /// See [`VkSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampler)
    struct VkSampler;
);

define_non_dispatchable_handle!(
    /// See [`VkDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPool)
    struct VkDescriptorPool;
);

define_non_dispatchable_handle!(
    /// See [`VkDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSet)
    struct VkDescriptorSet;
);

define_non_dispatchable_handle!(
    /// See [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebuffer)
    struct VkFramebuffer;
);

define_non_dispatchable_handle!(
    /// See [`VkCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPool)
    struct VkCommandPool;
);

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

vks_enum! {
    /// See [`VkPipelineCacheHeaderVersion`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheHeaderVersion)
    pub VkPipelineCacheHeaderVersion: u32 {
        const ONE = 1;
    }
}

vks_enum! {
    /// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
    pub VkResult: i32 {
        const SUCCESS = 0;
        const NOT_READY = 1;
        const TIMEOUT = 2;
        const EVENT_SET = 3;
        const EVENT_RESET = 4;
        const INCOMPLETE = 5;
        const ERROR_OUT_OF_HOST_MEMORY = -1;
        const ERROR_OUT_OF_DEVICE_MEMORY = -2;
        const ERROR_INITIALIZATION_FAILED = -3;
        const ERROR_DEVICE_LOST = -4;
        const ERROR_MEMORY_MAP_FAILED = -5;
        const ERROR_LAYER_NOT_PRESENT = -6;
        const ERROR_EXTENSION_NOT_PRESENT = -7;
        const ERROR_FEATURE_NOT_PRESENT = -8;
        const ERROR_INCOMPATIBLE_DRIVER = -9;
        const ERROR_TOO_MANY_OBJECTS = -10;
        const ERROR_FORMAT_NOT_SUPPORTED = -11;
        const ERROR_FRAGMENTED_POOL = -12;

        /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const ERROR_SURFACE_LOST_KHR = -1000000000;

        /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const SUBOPTIMAL_KHR = 1000001003;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const ERROR_OUT_OF_DATE_KHR = -1000001004;

        /// See extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
        const ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001;

        /// See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const ERROR_VALIDATION_FAILED_EXT = -1000011001;

        /// See extension [`VK_NV_glsl_shader`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_glsl_shader)
        const ERROR_INVALID_SHADER_NV = -1000012000;

        /// See extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const ERROR_OUT_OF_POOL_MEMORY_KHR = -1000069000;

        /// See extensions [`VK_KHR_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory) and
        /// [`VK_KHR_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore)
        const ERROR_INVALID_EXTERNAL_HANDLE_KHR = -1000072003;
    }
}

vks_enum! {
    /// See [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStructureType)
    pub VkStructureType: u32 {
        const APPLICATION_INFO = 0;
        const INSTANCE_CREATE_INFO = 1;
        const DEVICE_QUEUE_CREATE_INFO = 2;
        const DEVICE_CREATE_INFO = 3;
        const SUBMIT_INFO = 4;
        const MEMORY_ALLOCATE_INFO = 5;
        const MAPPED_MEMORY_RANGE = 6;
        const BIND_SPARSE_INFO = 7;
        const FENCE_CREATE_INFO = 8;
        const SEMAPHORE_CREATE_INFO = 9;
        const EVENT_CREATE_INFO = 10;
        const QUERY_POOL_CREATE_INFO = 11;
        const BUFFER_CREATE_INFO = 12;
        const BUFFER_VIEW_CREATE_INFO = 13;
        const IMAGE_CREATE_INFO = 14;
        const IMAGE_VIEW_CREATE_INFO = 15;
        const SHADER_MODULE_CREATE_INFO = 16;
        const PIPELINE_CACHE_CREATE_INFO = 17;
        const PIPELINE_SHADER_STAGE_CREATE_INFO = 18;
        const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19;
        const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20;
        const PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21;
        const PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22;
        const PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23;
        const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24;
        const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25;
        const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26;
        const PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27;
        const GRAPHICS_PIPELINE_CREATE_INFO = 28;
        const COMPUTE_PIPELINE_CREATE_INFO = 29;
        const PIPELINE_LAYOUT_CREATE_INFO = 30;
        const SAMPLER_CREATE_INFO = 31;
        const DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32;
        const DESCRIPTOR_POOL_CREATE_INFO = 33;
        const DESCRIPTOR_SET_ALLOCATE_INFO = 34;
        const WRITE_DESCRIPTOR_SET = 35;
        const COPY_DESCRIPTOR_SET = 36;
        const FRAMEBUFFER_CREATE_INFO = 37;
        const RENDER_PASS_CREATE_INFO = 38;
        const COMMAND_POOL_CREATE_INFO = 39;
        const COMMAND_BUFFER_ALLOCATE_INFO = 40;
        const COMMAND_BUFFER_INHERITANCE_INFO = 41;
        const COMMAND_BUFFER_BEGIN_INFO = 42;
        const RENDER_PASS_BEGIN_INFO = 43;
        const BUFFER_MEMORY_BARRIER = 44;
        const IMAGE_MEMORY_BARRIER = 45;
        const MEMORY_BARRIER = 46;
        const LOADER_INSTANCE_CREATE_INFO = 47;
        const LOADER_DEVICE_CREATE_INFO = 48;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const SWAPCHAIN_CREATE_INFO_KHR = 1000001000;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const PRESENT_INFO_KHR = 1000001001;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_MODE_CREATE_INFO_KHR = 1000002000;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001;

        /// See extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
        const DISPLAY_PRESENT_INFO_KHR = 1000003000;

        /// See extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
        const XLIB_SURFACE_CREATE_INFO_KHR = 1000004000;

        /// See extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
        const XCB_SURFACE_CREATE_INFO_KHR = 1000005000;

        /// See extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
        const WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000;

        /// See extension [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
        const MIR_SURFACE_CREATE_INFO_KHR = 1000007000;

        /// See extension [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)
        const ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000;

        /// See extension [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
        const WIN32_SURFACE_CREATE_INFO_KHR = 1000009000;

        /// See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const DEBUG_REPORT_CREATE_INFO_EXT = 1000011000;

        /// See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000;

        /// See extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)
        const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000;

        /// See extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
        const DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000;

        /// See extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
        const DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001;

        /// See extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
        const DEBUG_MARKER_MARKER_INFO_EXT = 1000022002;

        /// See extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
        const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000;

        /// See extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
        const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001;

        /// See extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)
        const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002;

        /// See extension [`VK_AMD_texture_gather_bias_lod`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_texture_gather_bias_lod)
        const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD = 1000041000;

        /// See extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
        const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000;

        /// See extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)
        const EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001;

        /// See extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
        const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000;

        /// See extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)
        const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001;

        /// See extension [`VK_NV_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_win32_keyed_mutex)
        const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const PHYSICAL_DEVICE_FEATURES_2_KHR = 1000059000;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const PHYSICAL_DEVICE_PROPERTIES_2_KHR = 1000059001;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const FORMAT_PROPERTIES_2_KHR = 1000059002;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059003;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR = 1000059004;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const QUEUE_FAMILY_PROPERTIES_2_KHR = 1000059005;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR = 1000059006;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059007;

        /// See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
        const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR = 1000059008;

        /// See extension [`VK_EXT_validation_flags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_validation_flags)
        const VALIDATION_FLAGS_EXT = 1000061000;

        /// See extension [`VK_NN_vi_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NN_vi_surface)
        const VI_SURFACE_CREATE_INFO_NN = 1000062000;

        /// See extension [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
        const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR = 1000071000;

        /// See extension [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
        const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR = 1000071001;

        /// See extension [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
        const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR = 1000071002;

        /// See extension [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities)
        const EXTERNAL_BUFFER_PROPERTIES_KHR = 1000071003;

        /// See extensions [`VK_KHR_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_capabilities),
        /// [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities), and
        /// [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)
        const PHYSICAL_DEVICE_ID_PROPERTIES_KHR = 1000071004;

        /// See extension [`VK_KHR_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory)
        const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR = 1000072000;

        /// See extension [`VK_KHR_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory)
        const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR = 1000072001;

        /// See extension [`VK_KHR_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory)
        const EXPORT_MEMORY_ALLOCATE_INFO_KHR = 1000072002;

        /// See extension [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073000;

        /// See extension [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073001;

        /// See extension [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        const MEMORY_WIN32_HANDLE_PROPERTIES_KHR = 1000073002;

        /// See extension [`VK_KHR_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_win32)
        const MEMORY_GET_WIN32_HANDLE_INFO_KHR = 1000073003;

        /// See extension [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
        const IMPORT_MEMORY_FD_INFO_KHR = 1000074000;

        /// See extension [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
        const MEMORY_FD_PROPERTIES_KHR = 1000074001;

        /// See extension [`VK_KHR_external_memory_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_memory_fd)
        const MEMORY_GET_FD_INFO_KHR = 1000074002;

        /// See extension [`VK_KHR_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#win32_keyed_mutex)
        const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR = 1000075000;

        /// See extension [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities)
        const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR = 1000076000;

        /// See extension [`VK_KHR_external_semaphore_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_capabilities)
        const EXTERNAL_SEMAPHORE_PROPERTIES_KHR = 1000076001;

        /// See extension [`VK_KHR_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore)
        const EXPORT_SEMAPHORE_CREATE_INFO_KHR = 1000077000;

        /// See extension [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078000;

        /// See extension [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078001;

        /// See extension [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        const D3D12_FENCE_SUBMIT_INFO_KHR = 1000078002;

        /// See extension [`VK_KHR_external_semaphore_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_win32)
        const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR = 1000078003;

        /// See extension [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
        const IMPORT_SEMAPHORE_FD_INFO_KHR = 1000079000;

        /// See extension [`VK_KHR_external_semaphore_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore_fd)
        const SEMAPHORE_GET_FD_INFO_KHR = 1000079001;

        /// See extension [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
        const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR = 1000080000;

        /// See extension [`VK_KHR_16bit_storage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_16bit_storage)
        const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR = 1000083000;

        /// See extension [`VK_KHR_incremental_present`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_incremental_present)
        const PRESENT_REGIONS_KHR = 1000084000;

        /// See extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
        const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR = 1000085000;

        /// See extension [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
        const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV = 1000087000;

        /// See extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
        const SURFACE_CAPABILITIES_2_EXT = 1000090000;

        /// See extension [`VK_EXT_display_surface_counter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_surface_counter)
        const SURFACE_CAPABILITIES2_EXT = 1000090000;

        /// See extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        const DISPLAY_POWER_INFO_EXT = 1000091000;

        /// See extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        const DEVICE_EVENT_INFO_EXT = 1000091001;

        /// See extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        const DISPLAY_EVENT_INFO_EXT = 1000091002;

        /// See extension [`VK_EXT_display_control`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_display_control)
        const SWAPCHAIN_COUNTER_CREATE_INFO_EXT = 1000091003;

        /// See extension [`VK_GOOGLE_display_timing`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_GOOGLE_display_timing)
        const PRESENT_TIMES_INFO_GOOGLE = 1000092000;

        /// See extension [`VK_NV_viewport_swizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_viewport_swizzle)
        const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV = 1000098000;

        /// See extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
        const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT = 1000099000;

        /// See extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_discard_rectangles)
        const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT = 1000099001;

        /// See extension [`VK_EXT_hdr_metadata`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_hdr_metadata)
        const HDR_METADATA_EXT = 1000105000;

        /// See extension [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)
        const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR = 1000111000;

        /// See extension [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)
        const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR = 1000112000;

        /// See extension [`VK_KHR_external_fence_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_capabilities)
        const EXTERNAL_FENCE_PROPERTIES_KHR = 1000112001;

        /// See extension [`VK_KHR_external_fence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence)
        const EXPORT_FENCE_CREATE_INFO_KHR = 1000113000;

        /// See extension [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
        const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114000;

        /// See extension [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
        const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114001;

        /// See extension [`VK_KHR_external_fence_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_win32)
        const FENCE_GET_WIN32_HANDLE_INFO_KHR = 1000114002;

        /// See extension [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
        const IMPORT_FENCE_FD_INFO_KHR = 1000115000;

        /// See extension [`VK_KHR_external_fence_fd`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_fence_fd)
        const FENCE_GET_FD_INFO_KHR = 1000115001;

        /// See extension [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
        const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR = 1000119000;

        /// See extension [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
        const SURFACE_CAPABILITIES_2_KHR = 1000119001;

        /// See extension [`VK_KHR_get_surface_capabilities2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_surface_capabilities2)
        const SURFACE_FORMAT_2_KHR = 1000119002;

        /// See extension [`VK_KHR_variable_pointers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_variable_pointers)
        const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR = 1000120000;

        /// See extension [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)
        const IOS_SURFACE_CREATE_INFO_MVK = 1000122000;

        /// See extension [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)
        const MACOS_SURFACE_CREATE_INFO_MVK = 1000123000;

        /// See extension [`VK_KHR_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_dedicated_allocation)
        const MEMORY_DEDICATED_REQUIREMENTS_KHR = 1000127000;

        /// See extension [`VK_KHR_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_dedicated_allocation)
        const MEMORY_DEDICATED_ALLOCATE_INFO_KHR = 1000127001;

        /// See extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
        const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT = 1000130000;

        /// See extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
        const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT = 1000130001;

        /// See extension [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146000;

        /// See extension [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146001;

        /// See extension [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146002;

        /// See extension [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        const MEMORY_REQUIREMENTS_2_KHR = 1000146003;

        /// See extension [`VK_KHR_get_memory_requirements2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_memory_requirements2)
        const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR = 1000146004;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT = 1000148000;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT = 1000148001;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT = 1000148002;

        /// See extension [`VK_NV_fragment_coverage_to_color`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fragment_coverage_to_color)
        const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV = 1000149000;

        /// See extension [`VK_NV_framebuffer_mixed_samples`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_framebuffer_mixed_samples)
        const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV = 1000152000;
    }
}

vks_enum! {
    /// See [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationScope)
    pub VkSystemAllocationScope: u32 {
        const COMMAND = 0;
        const OBJECT = 1;
        const CACHE = 2;
        const DEVICE = 3;
        const INSTANCE = 4;
    }
}

vks_enum! {
    /// See [`VkInternalAllocationType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInternalAllocationType)
    pub VkInternalAllocationType: u32 {
        const EXECUTABLE = 0;
    }
}

vks_enum! {
    /// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
    pub VkFormat: u32 {
        const UNDEFINED = 0;
        const R4G4_UNORM_PACK8 = 1;
        const R4G4B4A4_UNORM_PACK16 = 2;
        const B4G4R4A4_UNORM_PACK16 = 3;
        const R5G6B5_UNORM_PACK16 = 4;
        const B5G6R5_UNORM_PACK16 = 5;
        const R5G5B5A1_UNORM_PACK16 = 6;
        const B5G5R5A1_UNORM_PACK16 = 7;
        const A1R5G5B5_UNORM_PACK16 = 8;
        const R8_UNORM = 9;
        const R8_SNORM = 10;
        const R8_USCALED = 11;
        const R8_SSCALED = 12;
        const R8_UINT = 13;
        const R8_SINT = 14;
        const R8_SRGB = 15;
        const R8G8_UNORM = 16;
        const R8G8_SNORM = 17;
        const R8G8_USCALED = 18;
        const R8G8_SSCALED = 19;
        const R8G8_UINT = 20;
        const R8G8_SINT = 21;
        const R8G8_SRGB = 22;
        const R8G8B8_UNORM = 23;
        const R8G8B8_SNORM = 24;
        const R8G8B8_USCALED = 25;
        const R8G8B8_SSCALED = 26;
        const R8G8B8_UINT = 27;
        const R8G8B8_SINT = 28;
        const R8G8B8_SRGB = 29;
        const B8G8R8_UNORM = 30;
        const B8G8R8_SNORM = 31;
        const B8G8R8_USCALED = 32;
        const B8G8R8_SSCALED = 33;
        const B8G8R8_UINT = 34;
        const B8G8R8_SINT = 35;
        const B8G8R8_SRGB = 36;
        const R8G8B8A8_UNORM = 37;
        const R8G8B8A8_SNORM = 38;
        const R8G8B8A8_USCALED = 39;
        const R8G8B8A8_SSCALED = 40;
        const R8G8B8A8_UINT = 41;
        const R8G8B8A8_SINT = 42;
        const R8G8B8A8_SRGB = 43;
        const B8G8R8A8_UNORM = 44;
        const B8G8R8A8_SNORM = 45;
        const B8G8R8A8_USCALED = 46;
        const B8G8R8A8_SSCALED = 47;
        const B8G8R8A8_UINT = 48;
        const B8G8R8A8_SINT = 49;
        const B8G8R8A8_SRGB = 50;
        const A8B8G8R8_UNORM_PACK32 = 51;
        const A8B8G8R8_SNORM_PACK32 = 52;
        const A8B8G8R8_USCALED_PACK32 = 53;
        const A8B8G8R8_SSCALED_PACK32 = 54;
        const A8B8G8R8_UINT_PACK32 = 55;
        const A8B8G8R8_SINT_PACK32 = 56;
        const A8B8G8R8_SRGB_PACK32 = 57;
        const A2R10G10B10_UNORM_PACK32 = 58;
        const A2R10G10B10_SNORM_PACK32 = 59;
        const A2R10G10B10_USCALED_PACK32 = 60;
        const A2R10G10B10_SSCALED_PACK32 = 61;
        const A2R10G10B10_UINT_PACK32 = 62;
        const A2R10G10B10_SINT_PACK32 = 63;
        const A2B10G10R10_UNORM_PACK32 = 64;
        const A2B10G10R10_SNORM_PACK32 = 65;
        const A2B10G10R10_USCALED_PACK32 = 66;
        const A2B10G10R10_SSCALED_PACK32 = 67;
        const A2B10G10R10_UINT_PACK32 = 68;
        const A2B10G10R10_SINT_PACK32 = 69;
        const R16_UNORM = 70;
        const R16_SNORM = 71;
        const R16_USCALED = 72;
        const R16_SSCALED = 73;
        const R16_UINT = 74;
        const R16_SINT = 75;
        const R16_SFLOAT = 76;
        const R16G16_UNORM = 77;
        const R16G16_SNORM = 78;
        const R16G16_USCALED = 79;
        const R16G16_SSCALED = 80;
        const R16G16_UINT = 81;
        const R16G16_SINT = 82;
        const R16G16_SFLOAT = 83;
        const R16G16B16_UNORM = 84;
        const R16G16B16_SNORM = 85;
        const R16G16B16_USCALED = 86;
        const R16G16B16_SSCALED = 87;
        const R16G16B16_UINT = 88;
        const R16G16B16_SINT = 89;
        const R16G16B16_SFLOAT = 90;
        const R16G16B16A16_UNORM = 91;
        const R16G16B16A16_SNORM = 92;
        const R16G16B16A16_USCALED = 93;
        const R16G16B16A16_SSCALED = 94;
        const R16G16B16A16_UINT = 95;
        const R16G16B16A16_SINT = 96;
        const R16G16B16A16_SFLOAT = 97;
        const R32_UINT = 98;
        const R32_SINT = 99;
        const R32_SFLOAT = 100;
        const R32G32_UINT = 101;
        const R32G32_SINT = 102;
        const R32G32_SFLOAT = 103;
        const R32G32B32_UINT = 104;
        const R32G32B32_SINT = 105;
        const R32G32B32_SFLOAT = 106;
        const R32G32B32A32_UINT = 107;
        const R32G32B32A32_SINT = 108;
        const R32G32B32A32_SFLOAT = 109;
        const R64_UINT = 110;
        const R64_SINT = 111;
        const R64_SFLOAT = 112;
        const R64G64_UINT = 113;
        const R64G64_SINT = 114;
        const R64G64_SFLOAT = 115;
        const R64G64B64_UINT = 116;
        const R64G64B64_SINT = 117;
        const R64G64B64_SFLOAT = 118;
        const R64G64B64A64_UINT = 119;
        const R64G64B64A64_SINT = 120;
        const R64G64B64A64_SFLOAT = 121;
        const B10G11R11_UFLOAT_PACK32 = 122;
        const E5B9G9R9_UFLOAT_PACK32 = 123;
        const D16_UNORM = 124;
        const X8_D24_UNORM_PACK32 = 125;
        const D32_SFLOAT = 126;
        const S8_UINT = 127;
        const D16_UNORM_S8_UINT = 128;
        const D24_UNORM_S8_UINT = 129;
        const D32_SFLOAT_S8_UINT = 130;
        const BC1_RGB_UNORM_BLOCK = 131;
        const BC1_RGB_SRGB_BLOCK = 132;
        const BC1_RGBA_UNORM_BLOCK = 133;
        const BC1_RGBA_SRGB_BLOCK = 134;
        const BC2_UNORM_BLOCK = 135;
        const BC2_SRGB_BLOCK = 136;
        const BC3_UNORM_BLOCK = 137;
        const BC3_SRGB_BLOCK = 138;
        const BC4_UNORM_BLOCK = 139;
        const BC4_SNORM_BLOCK = 140;
        const BC5_UNORM_BLOCK = 141;
        const BC5_SNORM_BLOCK = 142;
        const BC6H_UFLOAT_BLOCK = 143;
        const BC6H_SFLOAT_BLOCK = 144;
        const BC7_UNORM_BLOCK = 145;
        const BC7_SRGB_BLOCK = 146;
        const ETC2_R8G8B8_UNORM_BLOCK = 147;
        const ETC2_R8G8B8_SRGB_BLOCK = 148;
        const ETC2_R8G8B8A1_UNORM_BLOCK = 149;
        const ETC2_R8G8B8A1_SRGB_BLOCK = 150;
        const ETC2_R8G8B8A8_UNORM_BLOCK = 151;
        const ETC2_R8G8B8A8_SRGB_BLOCK = 152;
        const EAC_R11_UNORM_BLOCK = 153;
        const EAC_R11_SNORM_BLOCK = 154;
        const EAC_R11G11_UNORM_BLOCK = 155;
        const EAC_R11G11_SNORM_BLOCK = 156;
        const ASTC_4x4_UNORM_BLOCK = 157;
        const ASTC_4x4_SRGB_BLOCK = 158;
        const ASTC_5x4_UNORM_BLOCK = 159;
        const ASTC_5x4_SRGB_BLOCK = 160;
        const ASTC_5x5_UNORM_BLOCK = 161;
        const ASTC_5x5_SRGB_BLOCK = 162;
        const ASTC_6x5_UNORM_BLOCK = 163;
        const ASTC_6x5_SRGB_BLOCK = 164;
        const ASTC_6x6_UNORM_BLOCK = 165;
        const ASTC_6x6_SRGB_BLOCK = 166;
        const ASTC_8x5_UNORM_BLOCK = 167;
        const ASTC_8x5_SRGB_BLOCK = 168;
        const ASTC_8x6_UNORM_BLOCK = 169;
        const ASTC_8x6_SRGB_BLOCK = 170;
        const ASTC_8x8_UNORM_BLOCK = 171;
        const ASTC_8x8_SRGB_BLOCK = 172;
        const ASTC_10x5_UNORM_BLOCK = 173;
        const ASTC_10x5_SRGB_BLOCK = 174;
        const ASTC_10x6_UNORM_BLOCK = 175;
        const ASTC_10x6_SRGB_BLOCK = 176;
        const ASTC_10x8_UNORM_BLOCK = 177;
        const ASTC_10x8_SRGB_BLOCK = 178;
        const ASTC_10x10_UNORM_BLOCK = 179;
        const ASTC_10x10_SRGB_BLOCK = 180;
        const ASTC_12x10_UNORM_BLOCK = 181;
        const ASTC_12x10_SRGB_BLOCK = 182;
        const ASTC_12x12_UNORM_BLOCK = 183;
        const ASTC_12x12_SRGB_BLOCK = 184;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006;

        /// See extension [`VK_IMG_format_pvrtc`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_format_pvrtc)
        const PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007;
    }
}

vks_enum! {
    /// See [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageType)
    pub VkImageType: u32 {
        const V_1D = 0;
        const V_2D = 1;
        const V_3D = 2;
    }
}

vks_enum! {
    /// See [`VkImageTiling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageTiling)
    pub VkImageTiling: u32 {
        const OPTIMAL = 0;
        const LINEAR = 1;
    }
}

vks_enum! {
    /// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
    pub VkPhysicalDeviceType: u32 {
        const OTHER = 0;
        const INTEGRATED_GPU = 1;
        const DISCRETE_GPU = 2;
        const VIRTUAL_GPU = 3;
        const CPU = 4;
    }
}

vks_enum! {
    /// See [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryType)
    pub VkQueryType: u32 {
        const OCCLUSION = 0;
        const PIPELINE_STATISTICS = 1;
        const TIMESTAMP = 2;
    }
}

vks_enum! {
    /// See [`VkSharingMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharingMode)
    pub VkSharingMode: u32 {
        const EXCLUSIVE = 0;
        const CONCURRENT = 1;
    }
}

vks_enum! {
    /// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
    pub VkImageLayout: u32 {
        const UNDEFINED = 0;
        const GENERAL = 1;
        const COLOR_ATTACHMENT_OPTIMAL = 2;
        const DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3;
        const DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4;
        const SHADER_READ_ONLY_OPTIMAL = 5;
        const TRANSFER_SRC_OPTIMAL = 6;
        const TRANSFER_DST_OPTIMAL = 7;
        const PREINITIALIZED = 8;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const PRESENT_SRC_KHR = 1000001002;

        /// See extension [`VK_KHR_shared_presentable_image`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharedPresentSurfaceCapabilitiesKHR)
        const SHARED_PRESENT_KHR = 1000111000;
    }
}

vks_enum! {
    /// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
    pub VkImageViewType: u32 {
        const V_1D = 0;
        const V_2D = 1;
        const V_3D = 2;
        const CUBE = 3;
        const V_1D_ARRAY = 4;
        const V_2D_ARRAY = 5;
        const CUBE_ARRAY = 6;
    }
}

vks_enum! {
    /// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
    pub VkComponentSwizzle: u32 {
        const IDENTITY = 0;
        const ZERO = 1;
        const ONE = 2;
        const R = 3;
        const G = 4;
        const B = 5;
        const A = 6;
    }
}

vks_enum! {
    /// See [`VkVertexInputRate`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputRate)
    pub VkVertexInputRate: u32 {
        const VERTEX = 0;
        const INSTANCE = 1;
    }
}

vks_enum! {
    /// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
    pub VkPrimitiveTopology: u32 {
        const POINT_LIST = 0;
        const LINE_LIST = 1;
        const LINE_STRIP = 2;
        const TRIANGLE_LIST = 3;
        const TRIANGLE_STRIP = 4;
        const TRIANGLE_FAN = 5;
        const LINE_LIST_WITH_ADJACENCY = 6;
        const LINE_STRIP_WITH_ADJACENCY = 7;
        const TRIANGLE_LIST_WITH_ADJACENCY = 8;
        const TRIANGLE_STRIP_WITH_ADJACENCY = 9;
        const PATCH_LIST = 10;
    }
}

vks_enum! {
    /// See [`VkPolygonMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPolygonMode)
    pub VkPolygonMode: u32 {
        const FILL = 0;
        const LINE = 1;
        const POINT = 2;

        /// See extension [`VK_NV_fill_rectangle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_fill_rectangle)
        const RECTANGLE_NV = 1000153000;
    }
}

vks_enum! {
    /// See [`VkFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFrontFace)
    pub VkFrontFace: u32 {
        const COUNTER_CLOCKWISE = 0;
        const CLOCKWISE = 1;
    }
}

vks_enum! {
    /// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
    pub VkCompareOp: u32 {
        const NEVER = 0;
        const LESS = 1;
        const EQUAL = 2;
        const LESS_OR_EQUAL = 3;
        const GREATER = 4;
        const NOT_EQUAL = 5;
        const GREATER_OR_EQUAL = 6;
        const ALWAYS = 7;
    }
}

vks_enum! {
    /// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
    pub VkStencilOp: u32 {
        const KEEP = 0;
        const ZERO = 1;
        const REPLACE = 2;
        const INCREMENT_AND_CLAMP = 3;
        const DECREMENT_AND_CLAMP = 4;
        const INVERT = 5;
        const INCREMENT_AND_WRAP = 6;
        const DECREMENT_AND_WRAP = 7;
    }
}

vks_enum! {
    /// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
    pub VkLogicOp: u32 {
        const CLEAR = 0;
        const AND = 1;
        const AND_REVERSE = 2;
        const COPY = 3;
        const AND_INVERTED = 4;
        const NO_OP = 5;
        const XOR = 6;
        const OR = 7;
        const NOR = 8;
        const EQUIVALENT = 9;
        const INVERT = 10;
        const OR_REVERSE = 11;
        const COPY_INVERTED = 12;
        const OR_INVERTED = 13;
        const NAND = 14;
        const SET = 15;
    }
}

vks_enum! {
    /// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
    pub VkBlendFactor: u32 {
        const ZERO = 0;
        const ONE = 1;
        const SRC_COLOR = 2;
        const ONE_MINUS_SRC_COLOR = 3;
        const DST_COLOR = 4;
        const ONE_MINUS_DST_COLOR = 5;
        const SRC_ALPHA = 6;
        const ONE_MINUS_SRC_ALPHA = 7;
        const DST_ALPHA = 8;
        const ONE_MINUS_DST_ALPHA = 9;
        const CONSTANT_COLOR = 10;
        const ONE_MINUS_CONSTANT_COLOR = 11;
        const CONSTANT_ALPHA = 12;
        const ONE_MINUS_CONSTANT_ALPHA = 13;
        const SRC_ALPHA_SATURATE = 14;
        const SRC1_COLOR = 15;
        const ONE_MINUS_SRC1_COLOR = 16;
        const SRC1_ALPHA = 17;
        const ONE_MINUS_SRC1_ALPHA = 18;
    }
}

vks_enum! {
    /// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
    pub VkBlendOp: u32 {
        const ADD = 0;
        const SUBTRACT = 1;
        const REVERSE_SUBTRACT = 2;
        const MIN = 3;
        const MAX = 4;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const ZERO_EXT = 1000148000;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SRC_EXT = 1000148001;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DST_EXT = 1000148002;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SRC_OVER_EXT = 1000148003;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DST_OVER_EXT = 1000148004;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SRC_IN_EXT = 1000148005;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DST_IN_EXT = 1000148006;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SRC_OUT_EXT = 1000148007;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DST_OUT_EXT = 1000148008;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SRC_ATOP_EXT = 1000148009;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DST_ATOP_EXT = 1000148010;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const XOR_EXT = 1000148011;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const MULTIPLY_EXT = 1000148012;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SCREEN_EXT = 1000148013;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const OVERLAY_EXT = 1000148014;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DARKEN_EXT = 1000148015;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const LIGHTEN_EXT = 1000148016;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const COLORDODGE_EXT = 1000148017;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const COLORBURN_EXT = 1000148018;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HARDLIGHT_EXT = 1000148019;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const SOFTLIGHT_EXT = 1000148020;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const DIFFERENCE_EXT = 1000148021;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const EXCLUSION_EXT = 1000148022;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const INVERT_EXT = 1000148023;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const INVERT_RGB_EXT = 1000148024;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const LINEARDODGE_EXT = 1000148025;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const LINEARBURN_EXT = 1000148026;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const VIVIDLIGHT_EXT = 1000148027;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const LINEARLIGHT_EXT = 1000148028;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PINLIGHT_EXT = 1000148029;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HARDMIX_EXT = 1000148030;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HSL_HUE_EXT = 1000148031;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HSL_SATURATION_EXT = 1000148032;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HSL_COLOR_EXT = 1000148033;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const HSL_LUMINOSITY_EXT = 1000148034;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PLUS_EXT = 1000148035;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PLUS_CLAMPED_EXT = 1000148036;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PLUS_CLAMPED_ALPHA_EXT = 1000148037;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const PLUS_DARKER_EXT = 1000148038;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const MINUS_EXT = 1000148039;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const MINUS_CLAMPED_EXT = 1000148040;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const CONTRAST_EXT = 1000148041;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const INVERT_OVG_EXT = 1000148042;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const RED_EXT = 1000148043;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const GREEN_EXT = 1000148044;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const BLUE_EXT = 1000148045;
    }
}

vks_enum! {
    /// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
    pub VkDynamicState: u32 {
        const VIEWPORT = 0;
        const SCISSOR = 1;
        const LINE_WIDTH = 2;
        const DEPTH_BIAS = 3;
        const BLEND_CONSTANTS = 4;
        const DEPTH_BOUNDS = 5;
        const STENCIL_COMPARE_MASK = 6;
        const STENCIL_WRITE_MASK = 7;
        const STENCIL_REFERENCE = 8;

        /// See extension [`VK_NV_clip_space_w_scaling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_clip_space_w_scaling)
        const VIEWPORT_W_SCALING_NV = 1000087000;

        /// See extension [`VK_EXT_discard_rectangles`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#EXT_discard_rectangles)
        const DISCARD_RECTANGLE_EXT = 1000099000;
    }
}

vks_enum! {
    /// See [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFilter)
    pub VkFilter: u32 {
        const NEAREST = 0;
        const LINEAR = 1;

        /// See extension [`VK_IMG_filter_cubic`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#IMG_filter_cubic)
        const CUBIC_IMG = 1000015000;
    }
}

vks_enum! {
    /// See [`VkSamplerMipmapMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerMipmapMode)
    pub VkSamplerMipmapMode: u32 {
        const NEAREST = 0;
        const LINEAR = 1;
    }
}

vks_enum! {
    /// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
    pub VkSamplerAddressMode: u32 {
        const REPEAT = 0;
        const MIRRORED_REPEAT = 1;
        const CLAMP_TO_EDGE = 2;
        const CLAMP_TO_BORDER = 3;

        /// and extension [`VK_KHR_sampler_mirror_clamp_to_edge`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_sampler_mirror_clamp_to_edge)
        const MIRROR_CLAMP_TO_EDGE = 4;
    }
}

vks_enum! {
    /// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
    pub VkBorderColor: u32 {
        const FLOAT_TRANSPARENT_BLACK = 0;
        const INT_TRANSPARENT_BLACK = 1;
        const FLOAT_OPAQUE_BLACK = 2;
        const INT_OPAQUE_BLACK = 3;
        const FLOAT_OPAQUE_WHITE = 4;
        const INT_OPAQUE_WHITE = 5;
    }
}

vks_enum! {
    /// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
    pub VkDescriptorType: u32 {
        const SAMPLER = 0;
        const COMBINED_IMAGE_SAMPLER = 1;
        const SAMPLED_IMAGE = 2;
        const STORAGE_IMAGE = 3;
        const UNIFORM_TEXEL_BUFFER = 4;
        const STORAGE_TEXEL_BUFFER = 5;
        const UNIFORM_BUFFER = 6;
        const STORAGE_BUFFER = 7;
        const UNIFORM_BUFFER_DYNAMIC = 8;
        const STORAGE_BUFFER_DYNAMIC = 9;
        const INPUT_ATTACHMENT = 10;
    }
}

vks_enum! {
    /// See [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentLoadOp)
    pub VkAttachmentLoadOp: u32 {
        const LOAD = 0;
        const CLEAR = 1;
        const DONT_CARE = 2;
    }
}

vks_enum! {
    /// See [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentStoreOp)
    pub VkAttachmentStoreOp: u32 {
        const STORE = 0;
        const DONT_CARE = 1;
    }
}

vks_enum! {
    /// See [`VkPipelineBindPoint`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineBindPoint)
    pub VkPipelineBindPoint: u32 {
        const GRAPHICS = 0;
        const COMPUTE = 1;
    }
}

vks_enum! {
    /// See [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferLevel)
    pub VkCommandBufferLevel: u32 {
        const PRIMARY = 0;
        const SECONDARY = 1;
    }
}

vks_enum! {
    /// See [`VkIndexType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndexType)
    pub VkIndexType: u32 {
        const UINT16 = 0;
        const UINT32 = 1;
    }
}

vks_enum! {
    /// See [`VkSubpassContents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassContents)
    pub VkSubpassContents: u32 {
        const INLINE = 0;
        const SECONDARY_COMMAND_BUFFERS = 1;
    }
}

vks_enum! {
    /// See [`VkObjectType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassContents)
    pub VkObjectType: u32 {
        const UNKNOWN = 0;
        const INSTANCE = 1;
        const PHYSICAL_DEVICE = 2;
        const DEVICE = 3;
        const QUEUE = 4;
        const SEMAPHORE = 5;
        const COMMAND_BUFFER = 6;
        const FENCE = 7;
        const DEVICE_MEMORY = 8;
        const BUFFER = 9;
        const IMAGE = 10;
        const EVENT = 11;
        const QUERY_POOL = 12;
        const BUFFER_VIEW = 13;
        const IMAGE_VIEW = 14;
        const SHADER_MODULE = 15;
        const PIPELINE_CACHE = 16;
        const PIPELINE_LAYOUT = 17;
        const RENDER_PASS = 18;
        const PIPELINE = 19;
        const DESCRIPTOR_SET_LAYOUT = 20;
        const SAMPLER = 21;
        const DESCRIPTOR_POOL = 22;
        const DESCRIPTOR_SET = 23;
        const FRAMEBUFFER = 24;
        const COMMAND_POOL = 25;

        /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const SURFACE_KHR = 1000000000;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const SWAPCHAIN_KHR = 1000001000;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_KHR = 1000002000;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_MODE_KHR = 1000002001;

        /// See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const DEBUG_REPORT_CALLBACK_EXT = 1000011000;

        /// See extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
        const DESCRIPTOR_UPDATE_TEMPLATE_KHR = 1000085000;
    }
}

bitflags! {
    /// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkInstanceCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
pub type VkInstanceCreateFlagBits = VkInstanceCreateFlags;

bitflags! {
    /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkFormatFeatureFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const SAMPLED_IMAGE_BIT = 0x00000001;
        const STORAGE_IMAGE_BIT = 0x00000002;
        const STORAGE_IMAGE_ATOMIC_BIT = 0x00000004;
        const UNIFORM_TEXEL_BUFFER_BIT = 0x00000008;
        const STORAGE_TEXEL_BUFFER_BIT = 0x00000010;
        const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020;
        const VERTEX_BUFFER_BIT = 0x00000040;
        const COLOR_ATTACHMENT_BIT = 0x00000080;
        const COLOR_ATTACHMENT_BLEND_BIT = 0x00000100;
        const DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200;
        const BLIT_SRC_BIT = 0x00000400;
        const BLIT_DST_BIT = 0x00000800;
        const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000;

        /// See extension [`VK_IMG_filter_cubic`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_filter_cubic)
        const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000;

        /// See extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const TRANSFER_SRC_BIT_KHR = 0x00004000;

        /// See extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const TRANSFER_DST_BIT_KHR = 0x00008000;

        /// See extension [`VK_EXT_sampler_filter_minmax`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_sampler_filter_minmax)
        const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = 0x00010000;
    }
}

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags;

bitflags! {
    /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkImageUsageFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const TRANSFER_SRC_BIT = 0x00000001;
        const TRANSFER_DST_BIT = 0x00000002;
        const SAMPLED_BIT = 0x00000004;
        const STORAGE_BIT = 0x00000008;
        const COLOR_ATTACHMENT_BIT = 0x00000010;
        const DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020;
        const TRANSIENT_ATTACHMENT_BIT = 0x00000040;
        const INPUT_ATTACHMENT_BIT = 0x00000080;
    }
}

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub type VkImageUsageFlagBits = VkImageUsageFlags;

bitflags! {
    /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkImageCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const SPARSE_BINDING_BIT = 0x00000001;
        const SPARSE_RESIDENCY_BIT = 0x00000002;
        const SPARSE_ALIASED_BIT = 0x00000004;
        const MUTABLE_FORMAT_BIT = 0x00000008;
        const CUBE_COMPATIBLE_BIT = 0x00000010;

        /// See extension [`VK_KHR_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_maintenance1)
        const V_2D_ARRAY_COMPATIBLE_BIT_KHR = 0x00000020;
    }
}

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub type VkImageCreateFlagBits = VkImageCreateFlags;

bitflags! {
    /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSampleCountFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const V_1_BIT = 0x00000001;
        const V_2_BIT = 0x00000002;
        const V_4_BIT = 0x00000004;
        const V_8_BIT = 0x00000008;
        const V_16_BIT = 0x00000010;
        const V_32_BIT = 0x00000020;
        const V_64_BIT = 0x00000040;
    }
}

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub type VkSampleCountFlagBits = VkSampleCountFlags;

bitflags! {
    /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkQueueFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const GRAPHICS_BIT = 0x00000001;
        const COMPUTE_BIT = 0x00000002;
        const TRANSFER_BIT = 0x00000004;
        const SPARSE_BINDING_BIT = 0x00000008;
    }
}

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub type VkQueueFlagBits = VkQueueFlags;

bitflags! {
    /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkMemoryPropertyFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const DEVICE_LOCAL_BIT = 0x00000001;
        const HOST_VISIBLE_BIT = 0x00000002;
        const HOST_COHERENT_BIT = 0x00000004;
        const HOST_CACHED_BIT = 0x00000008;
        const LAZILY_ALLOCATED_BIT = 0x00000010;
    }
}

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags;

bitflags! {
    /// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkMemoryHeapFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const DEVICE_LOCAL_BIT = 0x00000001;
    }
}

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags;

bitflags! {
    /// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDeviceCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
pub type VkDeviceCreateFlagBits = VkDeviceCreateFlags;

bitflags! {
    /// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDeviceQueueCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags;

bitflags! {
    /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineStageFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const TOP_OF_PIPE_BIT = 0x00000001;
        const DRAW_INDIRECT_BIT = 0x00000002;
        const VERTEX_INPUT_BIT = 0x00000004;
        const VERTEX_SHADER_BIT = 0x00000008;
        const TESSELLATION_CONTROL_SHADER_BIT = 0x00000010;
        const TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020;
        const GEOMETRY_SHADER_BIT = 0x00000040;
        const FRAGMENT_SHADER_BIT = 0x00000080;
        const EARLY_FRAGMENT_TESTS_BIT = 0x00000100;
        const LATE_FRAGMENT_TESTS_BIT = 0x00000200;
        const COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400;
        const COMPUTE_SHADER_BIT = 0x00000800;
        const TRANSFER_BIT = 0x00001000;
        const BOTTOM_OF_PIPE_BIT = 0x00002000;
        const HOST_BIT = 0x00004000;
        const ALL_GRAPHICS_BIT = 0x00008000;
        const ALL_COMMANDS_BIT = 0x00010000;
    }
}

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub type VkPipelineStageFlagBits = VkPipelineStageFlags;

bitflags! {
    /// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkMemoryMapFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
pub type VkMemoryMapFlagBits = VkMemoryMapFlags;

bitflags! {
    /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkImageAspectFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const COLOR_BIT = 0x00000001;
        const DEPTH_BIT = 0x00000002;
        const STENCIL_BIT = 0x00000004;
        const METADATA_BIT = 0x00000008;
    }
}

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub type VkImageAspectFlagBits = VkImageAspectFlags;

bitflags! {
    /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSparseImageFormatFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const SINGLE_MIPTAIL_BIT = 0x00000001;
        const ALIGNED_MIP_SIZE_BIT = 0x00000002;
        const NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004;
    }
}

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags;

bitflags! {
    /// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSparseMemoryBindFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const METADATA_BIT = 0x00000001;
    }
}

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags;

bitflags! {
    /// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkFenceCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const SIGNALED_BIT = 0x00000001;
    }
}

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub type VkFenceCreateFlagBits = VkFenceCreateFlags;

bitflags! {
    /// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSemaphoreCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
pub type VkSemaphoreCreateFlagBits = VkSemaphoreCreateFlags;

bitflags! {
    /// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkEventCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
pub type VkEventCreateFlagBits = VkEventCreateFlags;

bitflags! {
    /// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkQueryPoolCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
pub type VkQueryPoolCreateFlagBits = VkQueryPoolCreateFlags;

bitflags! {
    /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkQueryPipelineStatisticFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001;
        const INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002;
        const VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004;
        const GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008;
        const GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010;
        const CLIPPING_INVOCATIONS_BIT = 0x00000020;
        const CLIPPING_PRIMITIVES_BIT = 0x00000040;
        const FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080;
        const TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100;
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200;
        const COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400;
    }
}

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags;

bitflags! {
    /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkQueryResultFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const V_64_BIT = 0x00000001;
        const WAIT_BIT = 0x00000002;
        const WITH_AVAILABILITY_BIT = 0x00000004;
        const PARTIAL_BIT = 0x00000008;
    }
}

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub type VkQueryResultFlagBits = VkQueryResultFlags;

bitflags! {
    /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkBufferCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const SPARSE_BINDING_BIT = 0x00000001;
        const SPARSE_RESIDENCY_BIT = 0x00000002;
        const SPARSE_ALIASED_BIT = 0x00000004;
    }
}

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub type VkBufferCreateFlagBits = VkBufferCreateFlags;

bitflags! {
    /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkBufferUsageFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const TRANSFER_SRC_BIT = 0x00000001;
        const TRANSFER_DST_BIT = 0x00000002;
        const UNIFORM_TEXEL_BUFFER_BIT = 0x00000004;
        const STORAGE_TEXEL_BUFFER_BIT = 0x00000008;
        const UNIFORM_BUFFER_BIT = 0x00000010;
        const STORAGE_BUFFER_BIT = 0x00000020;
        const INDEX_BUFFER_BIT = 0x00000040;
        const VERTEX_BUFFER_BIT = 0x00000080;
        const INDIRECT_BUFFER_BIT = 0x00000100;
    }
}

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub type VkBufferUsageFlagBits = VkBufferUsageFlags;

bitflags! {
    /// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkBufferViewCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
pub type VkBufferViewCreateFlagBits = VkBufferViewCreateFlags;

bitflags! {
    /// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkImageViewCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags;

bitflags! {
    /// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkShaderModuleCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags;

bitflags! {
    /// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineCacheCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags;

bitflags! {
    /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const DISABLE_OPTIMIZATION_BIT = 0x00000001;
        const ALLOW_DERIVATIVES_BIT = 0x00000002;
        const DERIVATIVE_BIT = 0x00000004;
    }
}

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags;

bitflags! {
    /// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineShaderStageCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags;

bitflags! {
    /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkShaderStageFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const VERTEX_BIT = 0x00000001;
        const TESSELLATION_CONTROL_BIT = 0x00000002;
        const TESSELLATION_EVALUATION_BIT = 0x00000004;
        const GEOMETRY_BIT = 0x00000008;
        const FRAGMENT_BIT = 0x00000010;
        const COMPUTE_BIT = 0x00000020;
        const ALL_GRAPHICS = 0x0000001f;
        const ALL = 0x7fffffff;
    }
}

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub type VkShaderStageFlagBits = VkShaderStageFlags;

bitflags! {
    /// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineVertexInputStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
pub type VkPipelineVertexInputStateCreateFlagBits = VkPipelineVertexInputStateCreateFlags;

bitflags! {
    /// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineInputAssemblyStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
pub type VkPipelineInputAssemblyStateCreateFlagBits = VkPipelineInputAssemblyStateCreateFlags;

bitflags! {
    /// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineTessellationStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
pub type VkPipelineTessellationStateCreateFlagBits = VkPipelineTessellationStateCreateFlags;

bitflags! {
    /// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineViewportStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
pub type VkPipelineViewportStateCreateFlagBits = VkPipelineViewportStateCreateFlags;

bitflags! {
    /// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineRasterizationStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
pub type VkPipelineRasterizationStateCreateFlagBits = VkPipelineRasterizationStateCreateFlags;

bitflags! {
    /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCullModeFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const NONE = 0;
        const FRONT_BIT = 0x00000001;
        const BACK_BIT = 0x00000002;
        const FRONT_AND_BACK = 0x00000003;
    }
}

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub type VkCullModeFlagBits = VkCullModeFlags;

bitflags! {
    /// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineMultisampleStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
pub type VkPipelineMultisampleStateCreateFlagBits = VkPipelineMultisampleStateCreateFlags;

bitflags! {
    /// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineDepthStencilStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
pub type VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlags;

bitflags! {
    /// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineColorBlendStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub type VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlags;

bitflags! {
    /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkColorComponentFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const R_BIT = 0x00000001;
        const G_BIT = 0x00000002;
        const B_BIT = 0x00000004;
        const A_BIT = 0x00000008;
    }
}

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub type VkColorComponentFlagBits = VkColorComponentFlags;

bitflags! {
    /// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineDynamicStateCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
pub type VkPipelineDynamicStateCreateFlagBits = VkPipelineDynamicStateCreateFlags;

bitflags! {
    /// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkPipelineLayoutCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
pub type VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlags;

bitflags! {
    /// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSamplerCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags;

bitflags! {
    /// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDescriptorSetLayoutCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;

        /// See extension [`VK_KHR_push_descriptor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_push_descriptor)
        const PUSH_DESCRIPTOR_BIT_KHR = 0x00000001;
    }
}

/// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDescriptorPoolCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const FREE_DESCRIPTOR_SET_BIT = 0x00000001;
    }
}

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDescriptorPoolResetFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
pub type VkDescriptorPoolResetFlagBits = VkDescriptorPoolResetFlags;

bitflags! {
    /// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkFramebufferCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags;

bitflags! {
    /// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkRenderPassCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags;

bitflags! {
    /// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkAttachmentDescriptionFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const MAY_ALIAS_BIT = 0x00000001;
    }
}

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags;

bitflags! {
    /// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkSubpassDescriptionFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
    }
}

/// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags;

bitflags! {
    /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkAccessFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const INDIRECT_COMMAND_READ_BIT = 0x00000001;
        const INDEX_READ_BIT = 0x00000002;
        const VERTEX_ATTRIBUTE_READ_BIT = 0x00000004;
        const UNIFORM_READ_BIT = 0x00000008;
        const INPUT_ATTACHMENT_READ_BIT = 0x00000010;
        const SHADER_READ_BIT = 0x00000020;
        const SHADER_WRITE_BIT = 0x00000040;
        const COLOR_ATTACHMENT_READ_BIT = 0x00000080;
        const COLOR_ATTACHMENT_WRITE_BIT = 0x00000100;
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200;
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400;
        const TRANSFER_READ_BIT = 0x00000800;
        const TRANSFER_WRITE_BIT = 0x00001000;
        const HOST_READ_BIT = 0x00002000;
        const HOST_WRITE_BIT = 0x00004000;
        const MEMORY_READ_BIT = 0x00008000;
        const MEMORY_WRITE_BIT = 0x00010000;

        /// See extension [`VK_EXT_blend_operation_advanced`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_blend_operation_advanced)
        const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000;
    }
}

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub type VkAccessFlagBits = VkAccessFlags;

bitflags! {
    /// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDependencyFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const BY_REGION_BIT = 0x00000001;
    }
}

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub type VkDependencyFlagBits = VkDependencyFlags;

bitflags! {
    /// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCommandPoolCreateFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const TRANSIENT_BIT = 0x00000001;
        const RESET_COMMAND_BUFFER_BIT = 0x00000002;
    }
}

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags;

bitflags! {
    /// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCommandPoolResetFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const RELEASE_RESOURCES_BIT = 0x00000001;
    }
}

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags;

bitflags! {
    /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCommandBufferUsageFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const ONE_TIME_SUBMIT_BIT = 0x00000001;
        const RENDER_PASS_CONTINUE_BIT = 0x00000002;
        const SIMULTANEOUS_USE_BIT = 0x00000004;
    }
}

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags;

bitflags! {
    /// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkQueryControlFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const PRECISE_BIT = 0x00000001;
    }
}

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub type VkQueryControlFlagBits = VkQueryControlFlags;

bitflags! {
    /// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkCommandBufferResetFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const RELEASE_RESOURCES_BIT = 0x00000001;
    }
}

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags;

bitflags! {
    /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkStencilFaceFlags: u32 {
        const MAX_ENUM = 0x7fffffff;
        const FRONT_BIT = 0x00000001;
        const BACK_BIT = 0x00000002;
        const FRONT_AND_BACK = 0x00000003;
    }
}

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub type VkStencilFaceFlagBits = VkStencilFaceFlags;

/// See [`PFN_vkAllocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkAllocationFunction)
pub type PFN_vkAllocationFunction = Option<unsafe extern "system" fn(pUserData: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void>;

/// See [`PFN_vkReallocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkReallocationFunction)
pub type PFN_vkReallocationFunction = Option<unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void>;

/// See [`PFN_vkFreeFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkFreeFunction)
pub type PFN_vkFreeFunction = Option<unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void)>;

/// See [`PFN_vkInternalAllocationNotification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkInternalAllocationNotification)
pub type PFN_vkInternalAllocationNotification = Option<unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope)>;

/// See [`PFN_vkInternalFreeNotification`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkInternalFreeNotification)
pub type PFN_vkInternalFreeNotification = Option<unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope)>;

/// See [`PFN_vkVoidFunction`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkVoidFunction)
pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

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
            sType: VkStructureType::APPLICATION_INFO,
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
            sType: VkStructureType::INSTANCE_CREATE_INFO,
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
            .field("pfnAllocation", &self.pfnAllocation.map(|pfnAllocation| pfnAllocation as *mut c_void))
            .field("pfnReallocation", &self.pfnReallocation.map(|pfnReallocation| pfnReallocation as *mut c_void))
            .field("pfnFree", &self.pfnFree.map(|pfnFree| pfnFree as *mut c_void))
            .field("pfnInternalAllocation", &self.pfnInternalAllocation.map(|pfnInternalAllocation| pfnInternalAllocation as *mut c_void))
            .field("pfnInternalFree", &self.pfnInternalFree.map(|pfnInternalFree| pfnInternalFree as *mut c_void))
            .finish()
    }
}

impl Default for VkAllocationCallbacks {
    fn default() -> Self {
        VkAllocationCallbacks {
            pUserData: ptr::null_mut(),
            pfnAllocation: None,
            pfnReallocation: None,
            pfnFree: None,
            pfnInternalAllocation: None,
            pfnInternalFree: None,
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
            sType: VkStructureType::DEVICE_QUEUE_CREATE_INFO,
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
            sType: VkStructureType::DEVICE_CREATE_INFO,
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
            sType: VkStructureType::SUBMIT_INFO,
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
            sType: VkStructureType::MEMORY_ALLOCATE_INFO,
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
            sType: VkStructureType::MAPPED_MEMORY_RANGE,
            pNext: ptr::null(),
            memory: Default::default(),
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
            memory: Default::default(),
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
            buffer: Default::default(),
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
            image: Default::default(),
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
            memory: Default::default(),
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
            image: Default::default(),
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
            sType: VkStructureType::BIND_SPARSE_INFO,
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
            sType: VkStructureType::FENCE_CREATE_INFO,
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
            sType: VkStructureType::SEMAPHORE_CREATE_INFO,
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
            sType: VkStructureType::EVENT_CREATE_INFO,
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
            sType: VkStructureType::QUERY_POOL_CREATE_INFO,
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
            sType: VkStructureType::BUFFER_CREATE_INFO,
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
            sType: VkStructureType::BUFFER_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            buffer: Default::default(),
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
            sType: VkStructureType::IMAGE_CREATE_INFO,
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
            sType: VkStructureType::IMAGE_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            image: Default::default(),
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
            sType: VkStructureType::SHADER_MODULE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_CACHE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            module: Default::default(),
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
            sType: VkStructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
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
            sType: VkStructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
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
            sType: VkStructureType::GRAPHICS_PIPELINE_CREATE_INFO,
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
            layout: Default::default(),
            renderPass: Default::default(),
            subpass: Default::default(),
            basePipelineHandle: Default::default(),
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
            sType: VkStructureType::COMPUTE_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            layout: Default::default(),
            basePipelineHandle: Default::default(),
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
            sType: VkStructureType::PIPELINE_LAYOUT_CREATE_INFO,
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
            sType: VkStructureType::SAMPLER_CREATE_INFO,
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
            sType: VkStructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
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
            sType: VkStructureType::DESCRIPTOR_POOL_CREATE_INFO,
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
            sType: VkStructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: ptr::null(),
            descriptorPool: Default::default(),
            descriptorSetCount: Default::default(),
            pSetLayouts: ptr::null(),
        }
    }
}

/// See [`VkDescriptorImageInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorImageInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

/// See [`VkDescriptorBufferInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorBufferInfo)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
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
            sType: VkStructureType::WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: Default::default(),
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
            sType: VkStructureType::COPY_DESCRIPTOR_SET,
            pNext: ptr::null(),
            srcSet: Default::default(),
            srcBinding: Default::default(),
            srcArrayElement: Default::default(),
            dstSet: Default::default(),
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
            sType: VkStructureType::FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            renderPass: Default::default(),
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
            sType: VkStructureType::RENDER_PASS_CREATE_INFO,
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
            sType: VkStructureType::COMMAND_POOL_CREATE_INFO,
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
            sType: VkStructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: Default::default(),
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
            sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_INFO,
            pNext: ptr::null(),
            renderPass: Default::default(),
            subpass: Default::default(),
            framebuffer: Default::default(),
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
            sType: VkStructureType::COMMAND_BUFFER_BEGIN_INFO,
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

/// See [`VkClearColorValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearColorValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl fmt::Debug for VkClearColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.debug_struct("VkClearColorValue")
                .field("float32", &"not shown")
                .field("int32", &self.int32)
                .field("uint32", &self.uint32)
                .finish()
        }
    }
}

impl Default for VkClearColorValue {
    fn default() -> Self {
        VkClearColorValue {
            uint32: Default::default(),
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

/// See [`VkClearValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearValue)
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

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

impl Default for VkClearValue {
    fn default() -> Self {
        VkClearValue {
            color: Default::default(),
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
            sType: VkStructureType::MEMORY_BARRIER,
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
            sType: VkStructureType::BUFFER_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: Default::default(),
            dstAccessMask: Default::default(),
            srcQueueFamilyIndex: Default::default(),
            dstQueueFamilyIndex: Default::default(),
            buffer: Default::default(),
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
            sType: VkStructureType::IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: Default::default(),
            dstAccessMask: Default::default(),
            oldLayout: Default::default(),
            newLayout: Default::default(),
            srcQueueFamilyIndex: Default::default(),
            dstQueueFamilyIndex: Default::default(),
            image: Default::default(),
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
            sType: VkStructureType::RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: Default::default(),
            framebuffer: Default::default(),
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
pub type PFN_vkCreateInstance = Option<unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult>;

/// See [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyInstance)
pub type PFN_vkDestroyInstance = Option<unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
pub type PFN_vkEnumeratePhysicalDevices = Option<unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult>;

/// See [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
pub type PFN_vkGetPhysicalDeviceFeatures = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures)>;

/// See [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties)>;

/// See [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult>;

/// See [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
pub type PFN_vkGetPhysicalDeviceProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties)>;

/// See [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties)>;

/// See [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties)>;

/// See [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetInstanceProcAddr)
pub type PFN_vkGetInstanceProcAddr = Option<unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction>;

/// See [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceProcAddr)
pub type PFN_vkGetDeviceProcAddr = Option<unsafe extern "system" fn(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction>;

/// See [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
pub type PFN_vkCreateDevice = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult>;

/// See [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDevice)
pub type PFN_vkDestroyDevice = Option<unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
pub type PFN_vkEnumerateInstanceExtensionProperties = Option<unsafe extern "system" fn(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult>;

/// See [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
pub type PFN_vkEnumerateDeviceExtensionProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult>;

/// See [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
pub type PFN_vkEnumerateInstanceLayerProperties = Option<unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult>;

/// See [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
pub type PFN_vkEnumerateDeviceLayerProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult>;

/// See [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
pub type PFN_vkGetDeviceQueue = Option<unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue)>;

/// See [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
pub type PFN_vkQueueSubmit = Option<unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult>;

/// See [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
pub type PFN_vkQueueWaitIdle = Option<unsafe extern "system" fn(queue: VkQueue) -> VkResult>;

/// See [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
pub type PFN_vkDeviceWaitIdle = Option<unsafe extern "system" fn(device: VkDevice) -> VkResult>;

/// See [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
pub type PFN_vkAllocateMemory = Option<unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult>;

/// See [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeMemory)
pub type PFN_vkFreeMemory = Option<unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
pub type PFN_vkMapMemory = Option<unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult>;

/// See [`vkUnmapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUnmapMemory)
pub type PFN_vkUnmapMemory = Option<unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory)>;

/// See [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
pub type PFN_vkFlushMappedMemoryRanges = Option<unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult>;

/// See [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
pub type PFN_vkInvalidateMappedMemoryRanges = Option<unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult>;

/// See [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
pub type PFN_vkGetDeviceMemoryCommitment = Option<unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize)>;

/// See [`vkBindBufferMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindBufferMemory)
pub type PFN_vkBindBufferMemory = Option<unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult>;

/// See [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
pub type PFN_vkBindImageMemory = Option<unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult>;

/// See [`vkGetBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetBufferMemoryRequirements)
pub type PFN_vkGetBufferMemoryRequirements = Option<unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements)>;

/// See [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
pub type PFN_vkGetImageMemoryRequirements = Option<unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements)>;

/// See [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
pub type PFN_vkGetImageSparseMemoryRequirements = Option<unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements)>;

/// See [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = Option<unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties)>;

/// See [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
pub type PFN_vkQueueBindSparse = Option<unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult>;

/// See [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
pub type PFN_vkCreateFence = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult>;

/// See [`vkDestroyFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFence)
pub type PFN_vkDestroyFence = Option<unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
pub type PFN_vkResetFences = Option<unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult>;

/// See [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
pub type PFN_vkGetFenceStatus = Option<unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult>;

/// See [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
pub type PFN_vkWaitForFences = Option<unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult>;

/// See [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
pub type PFN_vkCreateSemaphore = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult>;

/// See [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySemaphore)
pub type PFN_vkDestroySemaphore = Option<unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
pub type PFN_vkCreateEvent = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult>;

/// See [`vkDestroyEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyEvent)
pub type PFN_vkDestroyEvent = Option<unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkGetEventStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetEventStatus)
pub type PFN_vkGetEventStatus = Option<unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult>;

/// See [`vkSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkSetEvent)
pub type PFN_vkSetEvent = Option<unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult>;

/// See [`vkResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetEvent)
pub type PFN_vkResetEvent = Option<unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult>;

/// See [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
pub type PFN_vkCreateQueryPool = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult>;

/// See [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyQueryPool)
pub type PFN_vkDestroyQueryPool = Option<unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
pub type PFN_vkGetQueryPoolResults = Option<unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult>;

/// See [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
pub type PFN_vkCreateBuffer = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult>;

/// See [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBuffer)
pub type PFN_vkDestroyBuffer = Option<unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
pub type PFN_vkCreateBufferView = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult>;

/// See [`vkDestroyBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyBufferView)
pub type PFN_vkDestroyBufferView = Option<unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
pub type PFN_vkCreateImage = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult>;

/// See [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImage)
pub type PFN_vkDestroyImage = Option<unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
pub type PFN_vkGetImageSubresourceLayout = Option<unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout)>;

/// See [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
pub type PFN_vkCreateImageView = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult>;

/// See [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyImageView)
pub type PFN_vkDestroyImageView = Option<unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
pub type PFN_vkCreateShaderModule = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult>;

/// See [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyShaderModule)
pub type PFN_vkDestroyShaderModule = Option<unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
pub type PFN_vkCreatePipelineCache = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult>;

/// See [`vkDestroyPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineCache)
pub type PFN_vkDestroyPipelineCache = Option<unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
pub type PFN_vkGetPipelineCacheData = Option<unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult>;

/// See [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
pub type PFN_vkMergePipelineCaches = Option<unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult>;

/// See [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
pub type PFN_vkCreateGraphicsPipelines = Option<unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult>;

/// See [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
pub type PFN_vkCreateComputePipelines = Option<unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult>;

/// See [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipeline)
pub type PFN_vkDestroyPipeline = Option<unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
pub type PFN_vkCreatePipelineLayout = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult>;

/// See [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyPipelineLayout)
pub type PFN_vkDestroyPipelineLayout = Option<unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
pub type PFN_vkCreateSampler = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult>;

/// See [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroySampler)
pub type PFN_vkDestroySampler = Option<unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
pub type PFN_vkCreateDescriptorSetLayout = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult>;

/// See [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorSetLayout)
pub type PFN_vkDestroyDescriptorSetLayout = Option<unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
pub type PFN_vkCreateDescriptorPool = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult>;

/// See [`vkDestroyDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDescriptorPool)
pub type PFN_vkDestroyDescriptorPool = Option<unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
pub type PFN_vkResetDescriptorPool = Option<unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult>;

/// See [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
pub type PFN_vkAllocateDescriptorSets = Option<unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult>;

/// See [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
pub type PFN_vkFreeDescriptorSets = Option<unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult>;

/// See [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
pub type PFN_vkUpdateDescriptorSets = Option<unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet)>;

/// See [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
pub type PFN_vkCreateFramebuffer = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult>;

/// See [`vkDestroyFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyFramebuffer)
pub type PFN_vkDestroyFramebuffer = Option<unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
pub type PFN_vkCreateRenderPass = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult>;

/// See [`vkDestroyRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyRenderPass)
pub type PFN_vkDestroyRenderPass = Option<unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkGetRenderAreaGranularity`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetRenderAreaGranularity)
pub type PFN_vkGetRenderAreaGranularity = Option<unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D)>;

/// See [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
pub type PFN_vkCreateCommandPool = Option<unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult>;

/// See [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyCommandPool)
pub type PFN_vkDestroyCommandPool = Option<unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks)>;

/// See [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
pub type PFN_vkResetCommandPool = Option<unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult>;

/// See [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
pub type PFN_vkAllocateCommandBuffers = Option<unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult>;

/// See [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers)
pub type PFN_vkFreeCommandBuffers = Option<unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer)>;

/// See [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
pub type PFN_vkBeginCommandBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult>;

/// See [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
pub type PFN_vkEndCommandBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult>;

/// See [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
pub type PFN_vkResetCommandBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult>;

/// See [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
pub type PFN_vkCmdBindPipeline = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline)>;

/// See [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
pub type PFN_vkCmdSetViewport = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport)>;

/// See [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
pub type PFN_vkCmdSetScissor = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D)>;

/// See [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
pub type PFN_vkCmdSetLineWidth = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: f32)>;

/// See [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
pub type PFN_vkCmdSetDepthBias = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32)>;

/// See [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
pub type PFN_vkCmdSetBlendConstants = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const f32)>;

/// See [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
pub type PFN_vkCmdSetDepthBounds = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32)>;

/// See [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
pub type PFN_vkCmdSetStencilCompareMask = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32)>;

/// See [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
pub type PFN_vkCmdSetStencilWriteMask = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32)>;

/// See [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
pub type PFN_vkCmdSetStencilReference = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32)>;

/// See [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
pub type PFN_vkCmdBindDescriptorSets = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32)>;

/// See [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
pub type PFN_vkCmdBindIndexBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType)>;

/// See [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
pub type PFN_vkCmdBindVertexBuffers = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize)>;

/// See [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
pub type PFN_vkCmdDraw = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32)>;

/// See [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
pub type PFN_vkCmdDrawIndexed = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32)>;

/// See [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
pub type PFN_vkCmdDrawIndirect = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32)>;

/// See [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
pub type PFN_vkCmdDrawIndexedIndirect = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32)>;

/// See [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
pub type PFN_vkCmdDispatch = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32)>;

/// See [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
pub type PFN_vkCmdDispatchIndirect = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize)>;

/// See [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
pub type PFN_vkCmdCopyBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy)>;

/// See [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
pub type PFN_vkCmdCopyImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy)>;

/// See [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
pub type PFN_vkCmdBlitImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter)>;

/// See [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
pub type PFN_vkCmdCopyBufferToImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy)>;

/// See [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
pub type PFN_vkCmdCopyImageToBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy)>;

/// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
pub type PFN_vkCmdUpdateBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void)>;

/// See [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
pub type PFN_vkCmdFillBuffer = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32)>;

/// See [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
pub type PFN_vkCmdClearColorImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange)>;

/// See [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
pub type PFN_vkCmdClearDepthStencilImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange)>;

/// See [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
pub type PFN_vkCmdClearAttachments = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect)>;

/// See [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
pub type PFN_vkCmdResolveImage = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve)>;

/// See [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
pub type PFN_vkCmdSetEvent = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags)>;

/// See [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
pub type PFN_vkCmdResetEvent = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags)>;

/// See [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
pub type PFN_vkCmdWaitEvents = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier)>;

/// See [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
pub type PFN_vkCmdPipelineBarrier = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier)>;

/// See [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
pub type PFN_vkCmdBeginQuery = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags)>;

/// See [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
pub type PFN_vkCmdEndQuery = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32)>;

/// See [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
pub type PFN_vkCmdResetQueryPool = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32)>;

/// See [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
pub type PFN_vkCmdWriteTimestamp = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32)>;

/// See [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
pub type PFN_vkCmdCopyQueryPoolResults = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags)>;

/// See [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
pub type PFN_vkCmdPushConstants = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void)>;

/// See [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
pub type PFN_vkCmdBeginRenderPass = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents)>;

/// See [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
pub type PFN_vkCmdNextSubpass = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents)>;

/// See [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
pub type PFN_vkCmdEndRenderPass = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer)>;

/// See [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
pub type PFN_vkCmdExecuteCommands = Option<unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer)>;

#[cfg(feature = "function_prototypes")]
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

    /// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);

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
