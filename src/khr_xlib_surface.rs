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
use core;
use libc::c_void;
use std::ptr;

pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHR_xlib_surface\x00";
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_xlib_surface";

vks_bitflags! {
    /// See [`VkXlibSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkXlibSurfaceCreateFlagsKHR: u32 {
        /// See [`VkXlibSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateFlagsKHR)
        /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
        const VK_XLIB_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkXlibSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateFlagsKHR)
/// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
pub type VkXlibSurfaceCreateFlagBitsKHR = VkXlibSurfaceCreateFlagsKHR;

/// See [`VkXlibSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateInfoKHR)
/// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut xlib_wrapper::Display,
    pub window: xlib_wrapper::Window,
}

impl Default for VkXlibSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkXlibSurfaceCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            dpy: ptr::null_mut(),
            window: Default::default(),
        }
    }
}

/// See [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
/// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> core::VkResult;

/// See [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXlibPresentationSupportKHR)
/// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut xlib_wrapper::Display, visualID: xlib_wrapper::VisualID) -> core::VkBool32;

#[cfg(not(feature = "no_function_prototypes"))]
extern "system" {
    /// See [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub fn vkCreateXlibSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXlibPresentationSupportKHR)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut xlib_wrapper::Display, visualID: xlib_wrapper::VisualID) -> core::VkBool32;
}
