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

//! [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)

use core::fmt;
use core::ptr;
use libc::{c_char, c_void};
use vk;

pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 8;
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_debug_report\x00";
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME_STR: &str = "VK_EXT_debug_report";

define_non_dispatchable_handle! {
    /// See [`VkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackEXT)
    struct VkDebugReportCallbackEXT;
}

vks_enum! {
    /// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
    pub VkDebugReportObjectTypeEXT: u32 {
        const UNKNOWN_EXT = 0;
        const INSTANCE_EXT = 1;
        const PHYSICAL_DEVICE_EXT = 2;
        const DEVICE_EXT = 3;
        const QUEUE_EXT = 4;
        const SEMAPHORE_EXT = 5;
        const COMMAND_BUFFER_EXT = 6;
        const FENCE_EXT = 7;
        const DEVICE_MEMORY_EXT = 8;
        const BUFFER_EXT = 9;
        const IMAGE_EXT = 10;
        const EVENT_EXT = 11;
        const QUERY_POOL_EXT = 12;
        const BUFFER_VIEW_EXT = 13;
        const IMAGE_VIEW_EXT = 14;
        const SHADER_MODULE_EXT = 15;
        const PIPELINE_CACHE_EXT = 16;
        const PIPELINE_LAYOUT_EXT = 17;
        const RENDER_PASS_EXT = 18;
        const PIPELINE_EXT = 19;
        const DESCRIPTOR_SET_LAYOUT_EXT = 20;
        const SAMPLER_EXT = 21;
        const DESCRIPTOR_POOL_EXT = 22;
        const DESCRIPTOR_SET_EXT = 23;
        const FRAMEBUFFER_EXT = 24;
        const COMMAND_POOL_EXT = 25;

        /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
        const SURFACE_KHR_EXT = 26;

        /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
        const SWAPCHAIN_KHR_EXT = 27;

        const DEBUG_REPORT_EXT = 28;
        const DEBUG_REPORT_CALLBACK_EXT_EXT = 28;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_KHR_EXT = 29;

        /// See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
        const DISPLAY_MODE_KHR_EXT = 30;

        /// See extension [`VK_KHR_descriptor_update_template`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_descriptor_update_template)
        const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT = 1000085000;
    }
}

vks_enum! {
    /// See [`VkDebugReportErrorEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportErrorEXT)
    pub VkDebugReportErrorEXT: u32 {
        const NONE_EXT = 0;
        const CALLBACK_REF_EXT = 1;
    }
}

bitflags! {
    /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkDebugReportFlagsEXT: u32 {
        const MAX_ENUM_EXT = 0x7fffffff;
        const INFORMATION_BIT_EXT = 0x00000001;
        const WARNING_BIT_EXT = 0x00000002;
        const PERFORMANCE_WARNING_BIT_EXT = 0x00000004;
        const ERROR_BIT_EXT = 0x00000008;
        const DEBUG_BIT_EXT = 0x00000010;
    }
}

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub type VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT;

/// See [`PFN_vkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkDebugReportCallbackEXT)
pub type PFN_vkDebugReportCallbackEXT = Option<unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char, pUserData: *mut c_void) -> vk::VkBool32>;

/// See [`VkDebugReportCallbackCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackCreateInfoEXT)
#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugReportFlagsEXT,
    pub pfnCallback: PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut c_void,
}

impl Copy for VkDebugReportCallbackCreateInfoEXT {}

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
            .field("pfnCallback", &self.pfnCallback.map(|pfnCallback| pfnCallback as *mut c_void))
            .field("pUserData", &self.pUserData)
            .finish()
    }
}

impl Default for VkDebugReportCallbackCreateInfoEXT {
    fn default() -> Self {
        VkDebugReportCallbackCreateInfoEXT {
            sType: vk::VkStructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            pfnCallback: None,
            pUserData: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
pub type PFN_vkCreateDebugReportCallbackEXT = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const vk::VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> vk::VkResult>;

/// See [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
pub type PFN_vkDestroyDebugReportCallbackEXT = Option<unsafe extern "system" fn(instance: vk::VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const vk::VkAllocationCallbacks)>;

/// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
pub type PFN_vkDebugReportMessageEXT = Option<unsafe extern "system" fn(instance: vk::VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char)>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
    pub fn vkCreateDebugReportCallbackEXT(instance: vk::VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const vk::VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> vk::VkResult;

    /// See [`vkDestroyDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDestroyDebugReportCallbackEXT)
    pub fn vkDestroyDebugReportCallbackEXT(instance: vk::VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const vk::VkAllocationCallbacks);

    /// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
    pub fn vkDebugReportMessageEXT(instance: vk::VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);
}
