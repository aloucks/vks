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

pub const VK_KHR_WIN32_SURFACE_EXTENSION_SPEC_VERSION: u32 = 5;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &'static [u8; 21] = b"VK_KHR_win32_surface\x00";
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_win32_surface";

bitflags! {
    #[repr(C)]
    pub flags VkWin32SurfaceCreateFlagsKHR: u32 {
        const VK_WIN32_SURFACE_CREATE_DUMMY = 0x00000000,
    }
}
pub type VkWin32SurfaceCreateFlagBitsKHR = VkWin32SurfaceCreateFlagsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: win32_wrapper::HINSTANCE,
    pub hwnd: win32_wrapper::HWND,
}

pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;

#[link(name = "vulkan")]
extern "system" {
    pub fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
}
