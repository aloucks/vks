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

pub type VkBool32 = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

#[repr(C)]
pub struct VkInstance_T(c_void);
pub type VkInstance = *mut VkInstance_T;

#[repr(C)]
pub struct VkPhysicalDevice_T(c_void);
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;

#[repr(C)]
pub struct VkDevice_T(c_void);
pub type VkDevice = *mut VkDevice_T;

#[repr(C)]
pub struct VkQueue_T(c_void);
pub type VkQueue = *mut VkQueue_T;

#[repr(C)]
pub struct VkSemaphore_T(c_void);
pub type VkSemaphore = *mut VkSemaphore_T;

#[repr(C)]
pub struct VkCommandBuffer_T(c_void);
pub type VkCommandBuffer = *mut VkCommandBuffer_T;

#[repr(C)]
pub struct VkFence_T(c_void);
pub type VkFence = *mut VkFence_T;

#[repr(C)]
pub struct VkDeviceMemory_T(c_void);
pub type VkDeviceMemory = *mut VkDeviceMemory_T;

#[repr(C)]
pub struct VkBuffer_T(c_void);
pub type VkBuffer = *mut VkBuffer_T;

#[repr(C)]
pub struct VkImage_T(c_void);
pub type VkImage = *mut VkImage_T;

#[repr(C)]
pub struct VkEvent_T(c_void);
pub type VkEvent = *mut VkEvent_T;

#[repr(C)]
pub struct VkQueryPool_T(c_void);
pub type VkQueryPool = *mut VkQueryPool_T;

#[repr(C)]
pub struct VkBufferView_T(c_void);
pub type VkBufferView = *mut VkBufferView_T;

#[repr(C)]
pub struct VkImageView_T(c_void);
pub type VkImageView = *mut VkImageView_T;

#[repr(C)]
pub struct VkShaderModule_T(c_void);
pub type VkShaderModule = *mut VkShaderModule_T;

#[repr(C)]
pub struct VkPipelineCache_T(c_void);
pub type VkPipelineCache = *mut VkPipelineCache_T;

#[repr(C)]
pub struct VkPipelineLayout_T(c_void);
pub type VkPipelineLayout = *mut VkPipelineLayout_T;

#[repr(C)]
pub struct VkRenderPass_T(c_void);
pub type VkRenderPass = *mut VkRenderPass_T;

#[repr(C)]
pub struct VkPipeline_T(c_void);
pub type VkPipeline = *mut VkPipeline_T;

#[repr(C)]
pub struct VkDescriptorSetLayout_T(c_void);
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;

#[repr(C)]
pub struct VkSampler_T(c_void);
pub type VkSampler = *mut VkSampler_T;

#[repr(C)]
pub struct VkDescriptorPool_T(c_void);
pub type VkDescriptorPool = *mut VkDescriptorPool_T;

#[repr(C)]
pub struct VkDescriptorSet_T(c_void);
pub type VkDescriptorSet = *mut VkDescriptorSet_T;

#[repr(C)]
pub struct VkFramebuffer_T(c_void);
pub type VkFramebuffer = *mut VkFramebuffer_T;

#[repr(C)]
pub struct VkCommandPool_T(c_void);
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
    const VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1,
});

cenum!(VkResult: i32 {
    const VK_SUCCESS = 0,
    const VK_NOT_READY = 1,
    const VK_TIMEOUT = 2,
    const VK_EVENT_SET = 3,
    const VK_EVENT_RESET = 4,
    const VK_INCOMPLETE = 5,
    const VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    const VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    const VK_ERROR_INITIALIZATION_FAILED = -3,
    const VK_ERROR_DEVICE_LOST = -4,
    const VK_ERROR_MEMORY_MAP_FAILED = -5,
    const VK_ERROR_LAYER_NOT_PRESENT = -6,
    const VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    const VK_ERROR_FEATURE_NOT_PRESENT = -8,
    const VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    const VK_ERROR_TOO_MANY_OBJECTS = -10,
    const VK_ERROR_FORMAT_NOT_SUPPORTED = -11,

    #[cfg(feature = "core_1_0_22")]
    const VK_ERROR_FRAGMENTED_POOL = -12,

    #[cfg(feature = "khr_surface_25")]
    const VK_ERROR_SURFACE_LOST_KHR = -1000000000,

    #[cfg(feature = "khr_surface_25")]
    const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,

    #[cfg(feature = "khr_swapchain_67")]
    const VK_SUBOPTIMAL_KHR = 1000001003,

    #[cfg(feature = "khr_swapchain_67")]
    const VK_ERROR_OUT_OF_DATE_KHR = -1000001004,

    #[cfg(feature = "khr_display_swapchain_9")]
    const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,

    #[cfg(feature = "ext_debug_report_1")]
    const VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,

    #[cfg(feature = "nv_glsl_shader_1")]
    const VK_ERROR_INVALID_SHADER_NV = -1000012000,
});

cenum!(VkStructureType: u32 {
    const VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    const VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    const VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
    const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,

    #[cfg(feature = "khr_swapchain_67")]
    const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,

    #[cfg(feature = "khr_swapchain_67")]
    const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,

    #[cfg(feature = "khr_display_21")]
    const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,

    #[cfg(feature = "khr_display_21")]
    const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,

    #[cfg(feature = "khr_display_swapchain_9")]
    const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR = 1000003000,

    #[cfg(feature = "khr_xlib_surface_6")]
    const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,

    #[cfg(feature = "khr_xcb_surface_6")]
    const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,

    #[cfg(feature = "khr_wayland_surface_5")]
    const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,

    #[cfg(feature = "khr_mir_surface_4")]
    const VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,

    #[cfg(feature = "khr_android_surface_6")]
    const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,

    #[cfg(feature = "khr_win32_surface_5")]
    const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

    #[cfg(feature = "ext_debug_report_1")]
    const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT = 1000011000,

    #[cfg(feature = "ext_debug_report_2")]
    const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,

    #[cfg(feature = "amd_rasterization_order_1")]
    const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,

    #[cfg(feature = "ext_debug_marker_3")]
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,

    #[cfg(feature = "ext_debug_marker_3")]
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,

    #[cfg(feature = "ext_debug_marker_3")]
    const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001,

    #[cfg(feature = "nv_dedicated_allocation_1")]
    const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002,

    #[cfg(feature = "nv_external_memory_1")]
    const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000,

    #[cfg(feature = "nv_external_memory_1")]
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001,

    #[cfg(feature = "nv_external_memory_win32_1")]
    const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000,

    #[cfg(feature = "nv_external_memory_win32_1")]
    const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001,

    #[cfg(feature = "nv_win32_keyed_mutex_1")]
    const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000,
});

cenum!(VkSystemAllocationScope: u32 {
    const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    const VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
});

cenum!(VkInternalAllocationType: u32 {
    const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
});

cenum!(VkFormat: u32 {
    const VK_FORMAT_UNDEFINED = 0,
    const VK_FORMAT_R4G4_UNORM_PACK8 = 1,
    const VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,
    const VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,
    const VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,
    const VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,
    const VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,
    const VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,
    const VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,
    const VK_FORMAT_R8_UNORM = 9,
    const VK_FORMAT_R8_SNORM = 10,
    const VK_FORMAT_R8_USCALED = 11,
    const VK_FORMAT_R8_SSCALED = 12,
    const VK_FORMAT_R8_UINT = 13,
    const VK_FORMAT_R8_SINT = 14,
    const VK_FORMAT_R8_SRGB = 15,
    const VK_FORMAT_R8G8_UNORM = 16,
    const VK_FORMAT_R8G8_SNORM = 17,
    const VK_FORMAT_R8G8_USCALED = 18,
    const VK_FORMAT_R8G8_SSCALED = 19,
    const VK_FORMAT_R8G8_UINT = 20,
    const VK_FORMAT_R8G8_SINT = 21,
    const VK_FORMAT_R8G8_SRGB = 22,
    const VK_FORMAT_R8G8B8_UNORM = 23,
    const VK_FORMAT_R8G8B8_SNORM = 24,
    const VK_FORMAT_R8G8B8_USCALED = 25,
    const VK_FORMAT_R8G8B8_SSCALED = 26,
    const VK_FORMAT_R8G8B8_UINT = 27,
    const VK_FORMAT_R8G8B8_SINT = 28,
    const VK_FORMAT_R8G8B8_SRGB = 29,
    const VK_FORMAT_B8G8R8_UNORM = 30,
    const VK_FORMAT_B8G8R8_SNORM = 31,
    const VK_FORMAT_B8G8R8_USCALED = 32,
    const VK_FORMAT_B8G8R8_SSCALED = 33,
    const VK_FORMAT_B8G8R8_UINT = 34,
    const VK_FORMAT_B8G8R8_SINT = 35,
    const VK_FORMAT_B8G8R8_SRGB = 36,
    const VK_FORMAT_R8G8B8A8_UNORM = 37,
    const VK_FORMAT_R8G8B8A8_SNORM = 38,
    const VK_FORMAT_R8G8B8A8_USCALED = 39,
    const VK_FORMAT_R8G8B8A8_SSCALED = 40,
    const VK_FORMAT_R8G8B8A8_UINT = 41,
    const VK_FORMAT_R8G8B8A8_SINT = 42,
    const VK_FORMAT_R8G8B8A8_SRGB = 43,
    const VK_FORMAT_B8G8R8A8_UNORM = 44,
    const VK_FORMAT_B8G8R8A8_SNORM = 45,
    const VK_FORMAT_B8G8R8A8_USCALED = 46,
    const VK_FORMAT_B8G8R8A8_SSCALED = 47,
    const VK_FORMAT_B8G8R8A8_UINT = 48,
    const VK_FORMAT_B8G8R8A8_SINT = 49,
    const VK_FORMAT_B8G8R8A8_SRGB = 50,
    const VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,
    const VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,
    const VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,
    const VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,
    const VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,
    const VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,
    const VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,
    const VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,
    const VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,
    const VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,
    const VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,
    const VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,
    const VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,
    const VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,
    const VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,
    const VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,
    const VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,
    const VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,
    const VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,
    const VK_FORMAT_R16_UNORM = 70,
    const VK_FORMAT_R16_SNORM = 71,
    const VK_FORMAT_R16_USCALED = 72,
    const VK_FORMAT_R16_SSCALED = 73,
    const VK_FORMAT_R16_UINT = 74,
    const VK_FORMAT_R16_SINT = 75,
    const VK_FORMAT_R16_SFLOAT = 76,
    const VK_FORMAT_R16G16_UNORM = 77,
    const VK_FORMAT_R16G16_SNORM = 78,
    const VK_FORMAT_R16G16_USCALED = 79,
    const VK_FORMAT_R16G16_SSCALED = 80,
    const VK_FORMAT_R16G16_UINT = 81,
    const VK_FORMAT_R16G16_SINT = 82,
    const VK_FORMAT_R16G16_SFLOAT = 83,
    const VK_FORMAT_R16G16B16_UNORM = 84,
    const VK_FORMAT_R16G16B16_SNORM = 85,
    const VK_FORMAT_R16G16B16_USCALED = 86,
    const VK_FORMAT_R16G16B16_SSCALED = 87,
    const VK_FORMAT_R16G16B16_UINT = 88,
    const VK_FORMAT_R16G16B16_SINT = 89,
    const VK_FORMAT_R16G16B16_SFLOAT = 90,
    const VK_FORMAT_R16G16B16A16_UNORM = 91,
    const VK_FORMAT_R16G16B16A16_SNORM = 92,
    const VK_FORMAT_R16G16B16A16_USCALED = 93,
    const VK_FORMAT_R16G16B16A16_SSCALED = 94,
    const VK_FORMAT_R16G16B16A16_UINT = 95,
    const VK_FORMAT_R16G16B16A16_SINT = 96,
    const VK_FORMAT_R16G16B16A16_SFLOAT = 97,
    const VK_FORMAT_R32_UINT = 98,
    const VK_FORMAT_R32_SINT = 99,
    const VK_FORMAT_R32_SFLOAT = 100,
    const VK_FORMAT_R32G32_UINT = 101,
    const VK_FORMAT_R32G32_SINT = 102,
    const VK_FORMAT_R32G32_SFLOAT = 103,
    const VK_FORMAT_R32G32B32_UINT = 104,
    const VK_FORMAT_R32G32B32_SINT = 105,
    const VK_FORMAT_R32G32B32_SFLOAT = 106,
    const VK_FORMAT_R32G32B32A32_UINT = 107,
    const VK_FORMAT_R32G32B32A32_SINT = 108,
    const VK_FORMAT_R32G32B32A32_SFLOAT = 109,
    const VK_FORMAT_R64_UINT = 110,
    const VK_FORMAT_R64_SINT = 111,
    const VK_FORMAT_R64_SFLOAT = 112,
    const VK_FORMAT_R64G64_UINT = 113,
    const VK_FORMAT_R64G64_SINT = 114,
    const VK_FORMAT_R64G64_SFLOAT = 115,
    const VK_FORMAT_R64G64B64_UINT = 116,
    const VK_FORMAT_R64G64B64_SINT = 117,
    const VK_FORMAT_R64G64B64_SFLOAT = 118,
    const VK_FORMAT_R64G64B64A64_UINT = 119,
    const VK_FORMAT_R64G64B64A64_SINT = 120,
    const VK_FORMAT_R64G64B64A64_SFLOAT = 121,
    const VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,
    const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,
    const VK_FORMAT_D16_UNORM = 124,
    const VK_FORMAT_X8_D24_UNORM_PACK32 = 125,
    const VK_FORMAT_D32_SFLOAT = 126,
    const VK_FORMAT_S8_UINT = 127,
    const VK_FORMAT_D16_UNORM_S8_UINT = 128,
    const VK_FORMAT_D24_UNORM_S8_UINT = 129,
    const VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
    const VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,
    const VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,
    const VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,
    const VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,
    const VK_FORMAT_BC2_UNORM_BLOCK = 135,
    const VK_FORMAT_BC2_SRGB_BLOCK = 136,
    const VK_FORMAT_BC3_UNORM_BLOCK = 137,
    const VK_FORMAT_BC3_SRGB_BLOCK = 138,
    const VK_FORMAT_BC4_UNORM_BLOCK = 139,
    const VK_FORMAT_BC4_SNORM_BLOCK = 140,
    const VK_FORMAT_BC5_UNORM_BLOCK = 141,
    const VK_FORMAT_BC5_SNORM_BLOCK = 142,
    const VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,
    const VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,
    const VK_FORMAT_BC7_UNORM_BLOCK = 145,
    const VK_FORMAT_BC7_SRGB_BLOCK = 146,
    const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,
    const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,
    const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
    const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
    const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
    const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
    const VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,
    const VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,
    const VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,
    const VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,
    const VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,
    const VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,
    const VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,
    const VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,
    const VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,
    const VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,
    const VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,
    const VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,
    const VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,
    const VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,
    const VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,
    const VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,
    const VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,
    const VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,
    const VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,
    const VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,
    const VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,
    const VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,
    const VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,
    const VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,
    const VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,
    const VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,
    const VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,
    const VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,
    const VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,
    const VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,
    const VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,
    const VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,

    #[cfg(feature = "img_format_pvrtc_1")]
    const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,
});

cenum!(VkImageType: u32 {
    const VK_IMAGE_TYPE_1D = 0,
    const VK_IMAGE_TYPE_2D = 1,
    const VK_IMAGE_TYPE_3D = 2,
});

cenum!(VkImageTiling: u32 {
    const VK_IMAGE_TILING_OPTIMAL = 0,
    const VK_IMAGE_TILING_LINEAR = 1,
});

cenum!(VkPhysicalDeviceType: u32 {
    const VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    const VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
});

cenum!(VkQueryType: u32 {
    const VK_QUERY_TYPE_OCCLUSION = 0,
    const VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    const VK_QUERY_TYPE_TIMESTAMP = 2,
});

cenum!(VkSharingMode: u32 {
    const VK_SHARING_MODE_EXCLUSIVE = 0,
    const VK_SHARING_MODE_CONCURRENT = 1,
});

cenum!(VkImageLayout: u32 {
    const VK_IMAGE_LAYOUT_UNDEFINED = 0,
    const VK_IMAGE_LAYOUT_GENERAL = 1,
    const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    const VK_IMAGE_LAYOUT_PREINITIALIZED = 8,

    #[cfg(feature = "khr_swapchain_67")]
    const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
});

cenum!(VkImageViewType: u32 {
    const VK_IMAGE_VIEW_TYPE_1D = 0,
    const VK_IMAGE_VIEW_TYPE_2D = 1,
    const VK_IMAGE_VIEW_TYPE_3D = 2,
    const VK_IMAGE_VIEW_TYPE_CUBE = 3,
    const VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
    const VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
    const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,
});

cenum!(VkComponentSwizzle: u32 {
    const VK_COMPONENT_SWIZZLE_IDENTITY = 0,
    const VK_COMPONENT_SWIZZLE_ZERO = 1,
    const VK_COMPONENT_SWIZZLE_ONE = 2,
    const VK_COMPONENT_SWIZZLE_R = 3,
    const VK_COMPONENT_SWIZZLE_G = 4,
    const VK_COMPONENT_SWIZZLE_B = 5,
    const VK_COMPONENT_SWIZZLE_A = 6,
});

cenum!(VkVertexInputRate: u32 {
    const VK_VERTEX_INPUT_RATE_VERTEX = 0,
    const VK_VERTEX_INPUT_RATE_INSTANCE = 1,
});

cenum!(VkPrimitiveTopology: u32 {
    const VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    const VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
    const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
    const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
});

cenum!(VkPolygonMode: u32 {
    const VK_POLYGON_MODE_FILL = 0,
    const VK_POLYGON_MODE_LINE = 1,
    const VK_POLYGON_MODE_POINT = 2,
});

cenum!(VkFrontFace: u32 {
    const VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    const VK_FRONT_FACE_CLOCKWISE = 1,
});

cenum!(VkCompareOp: u32 {
    const VK_COMPARE_OP_NEVER = 0,
    const VK_COMPARE_OP_LESS = 1,
    const VK_COMPARE_OP_EQUAL = 2,
    const VK_COMPARE_OP_LESS_OR_EQUAL = 3,
    const VK_COMPARE_OP_GREATER = 4,
    const VK_COMPARE_OP_NOT_EQUAL = 5,
    const VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
    const VK_COMPARE_OP_ALWAYS = 7,
});

cenum!(VkStencilOp: u32 {
    const VK_STENCIL_OP_KEEP = 0,
    const VK_STENCIL_OP_ZERO = 1,
    const VK_STENCIL_OP_REPLACE = 2,
    const VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
    const VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
    const VK_STENCIL_OP_INVERT = 5,
    const VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
    const VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
});

cenum!(VkLogicOp: u32 {
    const VK_LOGIC_OP_CLEAR = 0,
    const VK_LOGIC_OP_AND = 1,
    const VK_LOGIC_OP_AND_REVERSE = 2,
    const VK_LOGIC_OP_COPY = 3,
    const VK_LOGIC_OP_AND_INVERTED = 4,
    const VK_LOGIC_OP_NO_OP = 5,
    const VK_LOGIC_OP_XOR = 6,
    const VK_LOGIC_OP_OR = 7,
    const VK_LOGIC_OP_NOR = 8,
    const VK_LOGIC_OP_EQUIVALENT = 9,
    const VK_LOGIC_OP_INVERT = 10,
    const VK_LOGIC_OP_OR_REVERSE = 11,
    const VK_LOGIC_OP_COPY_INVERTED = 12,
    const VK_LOGIC_OP_OR_INVERTED = 13,
    const VK_LOGIC_OP_NAND = 14,
    const VK_LOGIC_OP_SET = 15,
});

cenum!(VkBlendFactor: u32 {
    const VK_BLEND_FACTOR_ZERO = 0,
    const VK_BLEND_FACTOR_ONE = 1,
    const VK_BLEND_FACTOR_SRC_COLOR = 2,
    const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    const VK_BLEND_FACTOR_DST_COLOR = 4,
    const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    const VK_BLEND_FACTOR_SRC_ALPHA = 6,
    const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    const VK_BLEND_FACTOR_DST_ALPHA = 8,
    const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
    const VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
    const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
    const VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
    const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
    const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
    const VK_BLEND_FACTOR_SRC1_COLOR = 15,
    const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
    const VK_BLEND_FACTOR_SRC1_ALPHA = 17,
    const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
});

cenum!(VkBlendOp: u32 {
    const VK_BLEND_OP_ADD = 0,
    const VK_BLEND_OP_SUBTRACT = 1,
    const VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    const VK_BLEND_OP_MIN = 3,
    const VK_BLEND_OP_MAX = 4,
});

cenum!(VkDynamicState: u32 {
    const VK_DYNAMIC_STATE_VIEWPORT = 0,
    const VK_DYNAMIC_STATE_SCISSOR = 1,
    const VK_DYNAMIC_STATE_LINE_WIDTH = 2,
    const VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
    const VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
    const VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
    const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
    const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
    const VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,
});

cenum!(VkFilter: u32 {
    const VK_FILTER_NEAREST = 0,
    const VK_FILTER_LINEAR = 1,

    #[cfg(feature = "img_filter_cubic_1")]
    const VK_FILTER_CUBIC_IMG = 1000015000,
});

cenum!(VkSamplerMipmapMode: u32 {
    const VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    const VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
});

cenum!(VkSamplerAddressMode: u32 {
    const VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
    const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,

    #[cfg(any(all(feature = "core_1_0_3", not(feature = "core_1_0_4")),
              all(feature = "core_1_0_4", feature = "khr_sampler_mirror_clamp_to_edge_1")))]
    const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
});

cenum!(VkBorderColor: u32 {
    const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    const VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    const VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
});

cenum!(VkDescriptorType: u32 {
    const VK_DESCRIPTOR_TYPE_SAMPLER = 0,
    const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
});

cenum!(VkAttachmentLoadOp: u32 {
    const VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    const VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    const VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
});

cenum!(VkAttachmentStoreOp: u32 {
    const VK_ATTACHMENT_STORE_OP_STORE = 0,
    const VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
});

cenum!(VkPipelineBindPoint: u32 {
    const VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    const VK_PIPELINE_BIND_POINT_COMPUTE = 1,
});

cenum!(VkCommandBufferLevel: u32 {
    const VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    const VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
});

cenum!(VkIndexType: u32 {
    const VK_INDEX_TYPE_UINT16 = 0,
    const VK_INDEX_TYPE_UINT32 = 1,
});

cenum!(VkSubpassContents: u32 {
    const VK_SUBPASS_CONTENTS_INLINE = 0,
    const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
});

bitflags! {
    #[repr(C)]
    pub flags VkInstanceCreateFlags: u32 {
        const VK_INSTANCE_CREATE_DUMMY = 0,
    }
}

bitflags! {
    #[repr(C)]
    pub flags VkFormatFeatureFlags: u32 {
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001,
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002,
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,
        const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010,
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,
        const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040,
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080,
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,
        const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,
        const VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400,
        const VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800,
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,

        #[cfg(feature = "img_filter_cubic_1")]
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000,
    }
}
pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags;

bitflags! {
    #[repr(C)]
    pub flags VkImageUsageFlags: u32 {
        const VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
        const VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
        const VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
        const VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
        const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
        const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
        const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
        const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
    }
}
pub type VkImageUsageFlagBits = VkImageUsageFlags;

bitflags! {
    #[repr(C)]
    pub flags VkImageCreateFlags: u32 {
        const VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
        const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
        const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
        const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
        const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010,
    }
}
pub type VkImageCreateFlagBits = VkImageCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSampleCountFlags: u32 {
        const VK_SAMPLE_COUNT_1_BIT = 0x00000001,
        const VK_SAMPLE_COUNT_2_BIT = 0x00000002,
        const VK_SAMPLE_COUNT_4_BIT = 0x00000004,
        const VK_SAMPLE_COUNT_8_BIT = 0x00000008,
        const VK_SAMPLE_COUNT_16_BIT = 0x00000010,
        const VK_SAMPLE_COUNT_32_BIT = 0x00000020,
        const VK_SAMPLE_COUNT_64_BIT = 0x00000040,
    }
}
pub type VkSampleCountFlagBits = VkSampleCountFlags;

bitflags! {
    #[repr(C)]
    pub flags VkQueueFlags: u32 {
        const VK_QUEUE_GRAPHICS_BIT = 0x00000001,
        const VK_QUEUE_COMPUTE_BIT = 0x00000002,
        const VK_QUEUE_TRANSFER_BIT = 0x00000004,
        const VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
    }
}
pub type VkQueueFlagBits = VkQueueFlags;

bitflags! {
    #[repr(C)]
    pub flags VkMemoryPropertyFlags: u32 {
        const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
        const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
        const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
        const VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
        const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
    }
}
pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags;

bitflags! {
    #[repr(C)]
    pub flags VkMemoryHeapFlags: u32 {
        const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
    }
}
pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDeviceCreateFlags: u32 {
        const VK_DEVICE_CREATE_DUMMY = 0,
    }
}
pub type VkDeviceCreateFlagBits = VkDeviceCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDeviceQueueCreateFlags: u32 {
        const VK_DEVICE_QUEUE_CREATE_DUMMY = 0,
    }
}
pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineStageFlags: u32 {
        const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
        const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
        const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
        const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
        const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
        const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
        const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
        const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
        const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
        const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
        const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
        const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
        const VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
        const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
        const VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
        const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
        const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
    }
}
pub type VkPipelineStageFlagBits = VkPipelineStageFlags;

bitflags! {
    #[repr(C)]
    pub flags VkMemoryMapFlags: u32 {
        const VK_MEMORY_MAP_DUMMY = 0,
    }
}
pub type VkMemoryMapFlagBits = VkMemoryMapFlags;

bitflags! {
    #[repr(C)]
    pub flags VkImageAspectFlags: u32 {
        const VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
        const VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
        const VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
        const VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008,
    }
}
pub type VkImageAspectFlagBits = VkImageAspectFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSparseImageFormatFlags: u32 {
        const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,
        const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,
        const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004,
    }
}
pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSparseMemoryBindFlags: u32 {
        const VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001,
    }
}
pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags;

bitflags! {
    #[repr(C)]
    pub flags VkFenceCreateFlags: u32 {
        const VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
    }
}
pub type VkFenceCreateFlagBits = VkFenceCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSemaphoreCreateFlags: u32 {
        const VK_SEMAPHORE_CREATE_DUMMY = 0,
    }
}
pub type VkSemaphoreCreateFlagBits = VkSemaphoreCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkEventCreateFlags: u32 {
        const VK_EVENT_CREATE_DUMMY = 0,
    }
}
pub type VkEventCreateFlagBits = VkEventCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkQueryPoolCreateFlags: u32 {
        const VK_QUERY_POOL_CREATE_DUMMY = 0,
    }
}
pub type VkQueryPoolCreateFlagBits = VkQueryPoolCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkQueryPipelineStatisticFlags: u32 {
        const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
        const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
        const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
        const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
        const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
        const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
        const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
        const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
        const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
        const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
        const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
    }
}
pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags;

bitflags! {
    #[repr(C)]
    pub flags VkQueryResultFlags: u32 {
        const VK_QUERY_RESULT_64_BIT = 0x00000001,
        const VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
        const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
        const VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008,
    }
}
pub type VkQueryResultFlagBits = VkQueryResultFlags;

bitflags! {
    #[repr(C)]
    pub flags VkBufferCreateFlags: u32 {
        const VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
        const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
        const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
    }
}
pub type VkBufferCreateFlagBits = VkBufferCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkBufferUsageFlags: u32 {
        const VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
        const VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
        const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
        const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
        const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
        const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
        const VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
        const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
        const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
    }
}
pub type VkBufferUsageFlagBits = VkBufferUsageFlags;

bitflags! {
    #[repr(C)]
    pub flags VkBufferViewCreateFlags: u32 {
        const VK_BUFFER_VIEW_CREATE_DUMMY = 0,
    }
}
pub type VkBufferViewCreateFlagBits = VkBufferViewCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkImageViewCreateFlags: u32 {
        const VK_IMAGE_VIEW_CREATE_DUMMY = 0,
    }
}
pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkShaderModuleCreateFlags: u32 {
        const VK_SHADER_MODULE_CREATE_DUMMY = 0,
    }
}
pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineCacheCreateFlags: u32 {
        const VK_PIPELINE_CACHE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineCreateFlags: u32 {
        const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
        const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
        const VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004,
    }
}
pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineShaderStageCreateFlags: u32 {
        const VK_PIPELINE_SHADER_STAGE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkShaderStageFlags: u32 {
        const VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
        const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
        const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
        const VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
        const VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
        const VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
        const VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
        const VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
    }
}
pub type VkShaderStageFlagBits = VkShaderStageFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineVertexInputStateCreateFlags: u32 {
        const VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineVertexInputStateCreateFlagBits = VkPipelineVertexInputStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineInputAssemblyStateCreateFlags: u32 {
        const VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineInputAssemblyStateCreateFlagBits = VkPipelineInputAssemblyStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineTessellationStateCreateFlags: u32 {
        const VK_PIPELINE_TESSELLATION_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineTessellationStateCreateFlagBits = VkPipelineTessellationStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineViewportStateCreateFlags: u32 {
        const VK_PIPELINE_VIEWPORT_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineViewportStateCreateFlagBits = VkPipelineViewportStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineRasterizationStateCreateFlags: u32 {
        const VK_PIPELINE_RASTERIZATION_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineRasterizationStateCreateFlagBits = VkPipelineRasterizationStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkCullModeFlags: u32 {
        const VK_CULL_MODE_NONE = 0,
        const VK_CULL_MODE_FRONT_BIT = 0x00000001,
        const VK_CULL_MODE_BACK_BIT = 0x00000002,
        const VK_CULL_MODE_FRONT_AND_BACK = 0x00000003,
    }
}
pub type VkCullModeFlagBits = VkCullModeFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineMultisampleStateCreateFlags: u32 {
        const VK_PIPELINE_MULTISAMPLE_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineMultisampleStateCreateFlagBits = VkPipelineMultisampleStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineDepthStencilStateCreateFlags: u32 {
        const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineColorBlendStateCreateFlags: u32 {
        const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkColorComponentFlags: u32 {
        const VK_COLOR_COMPONENT_R_BIT = 0x00000001,
        const VK_COLOR_COMPONENT_G_BIT = 0x00000002,
        const VK_COLOR_COMPONENT_B_BIT = 0x00000004,
        const VK_COLOR_COMPONENT_A_BIT = 0x00000008,
    }
}
pub type VkColorComponentFlagBits = VkColorComponentFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineDynamicStateCreateFlags: u32 {
        const VK_PIPELINE_DYNAMIC_STATE_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineDynamicStateCreateFlagBits = VkPipelineDynamicStateCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkPipelineLayoutCreateFlags: u32 {
        const VK_PIPELINE_LAYOUT_CREATE_DUMMY = 0,
    }
}
pub type VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSamplerCreateFlags: u32 {
        const VK_SAMPLER_CREATE_DUMMY = 0,
    }
}
pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDescriptorSetLayoutCreateFlags: u32 {
        const VK_DESCRIPTOR_SET_LAYOUT_CREATE_DUMMY = 0,
    }
}
pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDescriptorPoolCreateFlags: u32 {
        const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
    }
}
pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDescriptorPoolResetFlags: u32 {
        const VK_DESCRIPTOR_POOL_RESET_DUMMY = 0,
    }
}
pub type VkDescriptorPoolResetFlagBits = VkDescriptorPoolResetFlags;

bitflags! {
    #[repr(C)]
    pub flags VkFramebufferCreateFlags: u32 {
        const VK_FRAMEBUFFER_CREATE_DUMMY = 0,
    }
}
pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkRenderPassCreateFlags: u32 {
        const VK_RENDER_PASS_CREATE_FUMMY = 0x00000001,
    }
}
pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkAttachmentDescriptionFlags: u32 {
        const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001,
    }
}
pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags;

bitflags! {
    #[repr(C)]
    pub flags VkSubpassDescriptionFlags: u32 {
        const VK_SUBPASS_DESCRIPTION_DUMMY = 0,
    }
}
pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags;

bitflags! {
    #[repr(C)]
    pub flags VkAccessFlags: u32 {
        const VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
        const VK_ACCESS_INDEX_READ_BIT = 0x00000002,
        const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
        const VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
        const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
        const VK_ACCESS_SHADER_READ_BIT = 0x00000020,
        const VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
        const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
        const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
        const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
        const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
        const VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
        const VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
        const VK_ACCESS_HOST_READ_BIT = 0x00002000,
        const VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
        const VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
        const VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,
    }
}
pub type VkAccessFlagBits = VkAccessFlags;

bitflags! {
    #[repr(C)]
    pub flags VkDependencyFlags: u32 {
        const VK_DEPENDENCY_BY_REGION_BIT = 0x00000001,
    }
}
pub type VkDependencyFlagBits = VkDependencyFlags;

bitflags! {
    #[repr(C)]
    pub flags VkCommandPoolCreateFlags: u32 {
        const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
        const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    }
}
pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags;

bitflags! {
    #[repr(C)]
    pub flags VkCommandPoolResetFlags: u32 {
        const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}
pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags;

bitflags! {
    #[repr(C)]
    pub flags VkCommandBufferUsageFlags: u32 {
        const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
        const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
        const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,
    }
}
pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags;

bitflags! {
    #[repr(C)]
    pub flags VkQueryControlFlags: u32 {
        const VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001,
    }
}
pub type VkQueryControlFlagBits = VkQueryControlFlags;

bitflags! {
    #[repr(C)]
    pub flags VkCommandBufferResetFlags: u32 {
        const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}
pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags;

bitflags! {
    #[repr(C)]
    pub flags VkStencilFaceFlags: u32 {
        const VK_STENCIL_FACE_FRONT_BIT = 0x00000001,
        const VK_STENCIL_FACE_BACK_BIT = 0x00000002,
        const VK_STENCIL_FRONT_AND_BACK = 0x00000003,
    }
}
pub type VkStencilFaceFlagBits = VkStencilFaceFlags;

pub type PFN_vkAllocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkFreeFunction = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type PFN_vkVoidFunction = unsafe extern "system" fn();

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

impl ::std::fmt::Debug for VkAllocationCallbacks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

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

impl<'a> ::std::fmt::Debug for DeviceNameDebugHelper<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl ::std::fmt::Debug for VkPhysicalDeviceProperties {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

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

impl<'a> ::std::fmt::Debug for ExtensionNameDebugHelper<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl ::std::fmt::Debug for VkExtensionProperties {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("VkExtensionProperties")
            .field("extensionName", &ExtensionNameDebugHelper(&self.extensionName))
            .field("specVersion", &self.specVersion)
            .finish()
    }
}

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

impl<'a> ::std::fmt::Debug for LayerNameDebugHelper<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

struct DescriptionDebugHelper<'a>(&'a [c_char; VK_MAX_DESCRIPTION_SIZE]);

impl<'a> ::std::fmt::Debug for DescriptionDebugHelper<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_list().entries(&self.0[..]).finish()
    }
}

impl ::std::fmt::Debug for VkLayerProperties {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("VkLayerProperties")
            .field("layerName", &LayerNameDebugHelper(&self.layerName))
            .field("specVersion", &self.specVersion)
            .field("implementationVersion", &self.implementationVersion)
            .field("description", &DescriptionDebugHelper(&self.description))
            .finish()
    }
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlags,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPoolSize {
    pub type_: VkDescriptorType,
    pub descriptorCount: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl ::std::fmt::Debug for VkClearColorValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        unsafe {
            f.debug_struct("VkClearColorValue")
                .field("float32", &self.float32)
                .field("int32", &self.int32)
                .field("uint32", &self.uint32)
                .finish()
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

impl ::std::fmt::Debug for VkClearValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        unsafe {
            f.debug_struct("VkClearValue")
                .field("color", &self.color)
                .field("depthStencil", &self.depthStencil)
                .finish()
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

pub type PFN_vkCreateInstance = unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkCreateDevice = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
pub type PFN_vkFreeMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkMapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
pub type PFN_vkCreateFence = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkDestroyFence = unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;
pub type PFN_vkWaitForFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateEvent = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateImage = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
pub type PFN_vkDestroyImage = unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
pub type PFN_vkCreateImageView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateSampler = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
pub type PFN_vkDestroySampler = unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);
pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const f32);
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
pub type PFN_vkCmdDraw = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, x: u32, y: u32, z: u32);
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

#[cfg(feature = "core_1_0_19")]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);

#[cfg(all(feature = "core_1_0_3", not(feature = "core_1_0_19")))]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const u32);

pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
    pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
    pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32);
    pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const f32);
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, x: u32, y: u32, z: u32);
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

    #[cfg(feature = "core_1_0_19")]
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);

    #[cfg(all(feature = "core_1_0_3", not(feature = "core_1_0_19")))]
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const u32);

    pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
}
