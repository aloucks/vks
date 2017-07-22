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

//! [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)

use ::*;
use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_wayland_surface\x00";
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_wayland_surface";

vks_bitflags! {
    /// See [`VkWaylandSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkWaylandSurfaceCreateFlagsKHR: u32 {
        /// See [`VkWaylandSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateFlagsKHR)
        /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
        const VK_WAYLAND_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkWaylandSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateFlagsKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
pub type VkWaylandSurfaceCreateFlagBitsKHR = VkWaylandSurfaceCreateFlagsKHR;

/// See [`VkWaylandSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateInfoKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wayland_wrapper::wl_display,
    pub surface: *mut wayland_wrapper::wl_surface,
}

impl Default for VkWaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWaylandSurfaceCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            display: ptr::null_mut(),
            surface: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

/// See [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> core::VkBool32;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn vkCreateWaylandSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> core::VkBool32;
}
