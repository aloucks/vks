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

//! [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)

use core::ptr;
use khr_surface;
use libc::c_void;
use vk;
use xcb_types;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static [u8; 19] = b"VK_KHR_xcb_surface\x00";
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_xcb_surface";

vks_bitflags! {
    /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkXcbSurfaceCreateFlagsKHR: u32 {
        /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
        const VK_XCB_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
pub type VkXcbSurfaceCreateFlagBitsKHR = VkXcbSurfaceCreateFlagsKHR;

/// See [`VkXcbSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_types::xcb_connection_t,
    pub window: xcb_types::xcb_window_t,
}

impl Default for VkXcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXcbSurfaceCreateInfoKHR {
            sType: vk::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            connection: ptr::null_mut(),
            window: Default::default(),
        }
    }
}

/// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
pub type PFN_vkCreateXcbSurfaceKHR = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

/// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<unsafe extern "system" fn(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_types::xcb_connection_t, visual_id: xcb_types::xcb_visualid_t) -> vk::VkBool32>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
    pub fn vkCreateXcbSurfaceKHR(instance: vk::VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;

    /// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: vk::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_types::xcb_connection_t, visual_id: xcb_types::xcb_visualid_t) -> vk::VkBool32;
}
