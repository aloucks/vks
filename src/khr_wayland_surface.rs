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

#[cfg(feature = "khr_wayland_surface_6")]
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;

#[cfg(all(feature = "khr_wayland_surface_5", not(feature = "khr_wayland_surface_6")))]
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 5;

pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_wayland_surface\x00";
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_wayland_surface";

bitflags! {
    /// See [`VkWaylandSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateFlagsKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    #[repr(C)]
    #[derive(Default)]
    pub struct VkWaylandSurfaceCreateFlagsKHR: u32 {
        const VK_WAYLAND_SURFACE_CREATE_DUMMY = 0x00000000;
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
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wayland_wrapper::wl_display,
    pub surface: *mut wayland_wrapper::wl_surface,
}

impl Default for VkWaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWaylandSurfaceCreateInfoKHR  {
            sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            display: ptr::null_mut(),
            surface: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

/// See [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
/// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> VkBool32;

#[cfg_attr(not(windows), link(name = "vulkan"))]
#[cfg_attr(windows, link(name = "vulkan-1"))]
extern "system" {
    /// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// See [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> VkBool32;
}
