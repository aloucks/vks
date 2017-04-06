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

pub const VK_KHR_XLIB_SURFACE_EXTENSION_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static [u8; 20] = b"VK_KHR_xlib_surface\x00";
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_xlib_surface";

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub flags VkXlibSurfaceCreateFlagsKHR: u32 {
        const VK_XLIB_SURFACE_CREATE_DUMMY = 0x00000000,
    }
}
pub type VkXlibSurfaceCreateFlagBitsKHR = VkXlibSurfaceCreateFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut xlib_wrapper::Display,
    pub window: xlib_wrapper::Window,
}

pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut xlib_wrapper::Display, visualID: xlib_wrapper::VisualID) -> VkBool32;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut xlib_wrapper::Display, visualID: xlib_wrapper::VisualID) -> VkBool32;
}
