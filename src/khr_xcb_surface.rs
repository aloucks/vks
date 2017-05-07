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

pub const VK_KHR_XCB_SURFACE_EXTENSION_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static [u8; 19] = b"VK_KHR_xcb_surface\x00";
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_xcb_surface";

bitflags! {
    /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    #[repr(C)]
    #[derive(Default)]
    pub flags VkXcbSurfaceCreateFlagsKHR: u32 {
        const VK_XCB_SURFACE_CREATE_DUMMY = 0x00000000,
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
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_wrapper::xcb_connection_t,
    pub window: xcb_wrapper::xcb_window_t,
}

impl Default for VkXcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXcbSurfaceCreateInfoKHR  {
            sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            connection: ptr::null_mut(),
            window: Default::default(),
        }
    }
}

/// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

/// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
/// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_wrapper::xcb_connection_t, visual_id: xcb_wrapper::xcb_visualid_t) -> VkBool32;

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_wrapper::xcb_connection_t, visual_id: xcb_wrapper::xcb_visualid_t) -> VkBool32;
}
