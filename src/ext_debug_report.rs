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
use libc::{c_char, c_void};
use std::fmt;
use std::mem;
use std::ptr;

#[cfg(feature = "ext_debug_report_8")]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 8;

#[cfg(all(feature = "ext_debug_report_6", not(feature = "ext_debug_report_8")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 6;

#[cfg(all(feature = "ext_debug_report_5", not(feature = "ext_debug_report_6")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 5;

#[cfg(all(feature = "ext_debug_report_4", not(feature = "ext_debug_report_5")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 4;

#[cfg(all(feature = "ext_debug_report_3", not(feature = "ext_debug_report_4")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 3;

#[cfg(all(feature = "ext_debug_report_2", not(feature = "ext_debug_report_3")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 2;

#[cfg(all(feature = "ext_debug_report_1", not(feature = "ext_debug_report_2")))]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 1;

pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &'static [u8; 20] = b"VK_EXT_debug_report\x00";
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME_STR: &'static str = "VK_EXT_debug_report";

/// See [`VkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
#[repr(C)]
pub struct VkDebugReportCallbackEXT_T(c_void);

/// See [`VkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type VkDebugReportCallbackEXT = *mut VkDebugReportCallbackEXT_T;

cenum!(VkDebugReportObjectTypeEXT: u32 {
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,

    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT = 28,

    #[cfg(feature = "ext_debug_report_8")]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT = 28,

    #[cfg(all(feature = "ext_debug_report_4", feature = "khr_display_21"))]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extensions [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report),
    /// [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT = 29,

    #[cfg(all(feature = "ext_debug_report_4", feature = "khr_display_21"))]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extensions [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report),
    /// [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT = 30,

    #[cfg(all(feature = "ext_debug_report_4", feature = "nvx_device_generated_commands_1"))]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extensions [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report),
    /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT = 31,

    #[cfg(all(feature = "ext_debug_report_4", feature = "nvx_device_generated_commands_1"))]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extensions [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report),
    /// [`VK_NVX_device_generated_commands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NVX_device_generated_commands)
    const VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT = 32,

    #[cfg(all(feature = "ext_debug_report_6", feature = "khr_descriptor_update_template_1"))]
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    /// and extensions [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report),
    /// [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
    const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT = 1000085000,
});

#[cfg(not(feature = "ext_debug_report_8"))]
cenum!(VkDebugReportErrorEXT: u32 {
    /// See [`VkDebugReportErrorEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportErrorEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_ERROR_NONE_EXT = 0,

    /// See [`VkDebugReportErrorEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportErrorEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    const VK_DEBUG_REPORT_ERROR_CALLBACK_REF_EXT = 1,
});

bitflags! {
    /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkDebugReportFlagsEXT: u32 {
        /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
        /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,

        /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
        /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,

        /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
        /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,

        /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
        /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,

        /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
        /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
        const VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
    }
}

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT;

/// See [`PFN_vkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkDebugReportCallbackEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char, pUserData: *mut c_void) -> VkBool32;

/// See [`VkDebugReportCallbackCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackCreateInfoEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugReportFlagsEXT,
    pub pfnCallback: PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut c_void,
}

impl Copy for VkDebugReportCallbackCreateInfoEXT { }

impl Clone for VkDebugReportCallbackCreateInfoEXT {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for VkDebugReportCallbackCreateInfoEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VkDebugReportCallbackCreateInfoEXT")
            .field("sType", &self.sType)
            .field("pNext", &self.pNext)
            .field("flags", &self.flags)
            .field("pfnCallback", &(self.pfnCallback as *mut c_void))
            .field("pUserData", &self.pUserData)
            .finish()
    }
}

impl Default for VkDebugReportCallbackCreateInfoEXT {
    fn default() -> Self {
        VkDebugReportCallbackCreateInfoEXT  {
            // VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT is an alias for
            // VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT, which is available since
            // revision 1.
            sType: VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            pfnCallback: unsafe { mem::transmute(0usize) },
            pUserData: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;

/// See [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);

/// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
/// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn vkCreateDebugReportCallbackEXT(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;

    /// See [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn vkDestroyDebugReportCallbackEXT(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);

    /// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn vkDebugReportMessageEXT(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);
}
