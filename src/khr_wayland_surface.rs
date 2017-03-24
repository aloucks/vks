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

pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_wayland_surface\x00";
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_wayland_surface";

bitflags! {
    #[repr(C)]
    pub flags VkWaylandSurfaceCreateFlagsKHR: u32 {
        const VK_WAYLAND_SURFACE_CREATE_DUMMY = 0x00000000,
    }
}
pub type VkWaylandSurfaceCreateFlagBitsKHR = VkWaylandSurfaceCreateFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wayland_wrapper::wl_display,
    pub surface: *mut wayland_wrapper::wl_surface,
}

pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> VkBool32;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_wrapper::wl_display) -> VkBool32;
}
