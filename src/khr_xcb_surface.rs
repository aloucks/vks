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

use ::*;
use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static [u8; 19] = b"VK_KHR_xcb_surface\x00";
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_xcb_surface";

vks_bitflags! {
    /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkXcbSurfaceCreateFlagsKHR: u32 {
        /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
        /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
        const VK_XCB_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
pub type VkXcbSurfaceCreateFlagBitsKHR = VkXcbSurfaceCreateFlagsKHR;

/// See [`VkXcbSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateInfoKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_wrapper::xcb_connection_t,
    pub window: xcb_wrapper::xcb_window_t,
}

impl Default for VkXcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXcbSurfaceCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            connection: ptr::null_mut(),
            window: Default::default(),
        }
    }
}

/// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

/// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_wrapper::xcb_connection_t, visual_id: xcb_wrapper::xcb_visualid_t) -> core::VkBool32;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub fn vkCreateXcbSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_wrapper::xcb_connection_t, visual_id: xcb_wrapper::xcb_visualid_t) -> core::VkBool32;
}
