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

//! [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)

use ::*;
use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &'static [u8; 21] = b"VK_KHR_win32_surface\x00";
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_win32_surface";

vks_bitflags! {
    /// See [`VkWin32SurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkWin32SurfaceCreateFlagsKHR: u32 {
        /// See [`VkWin32SurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateFlagsKHR)
        const VK_WIN32_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkWin32SurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateFlagsKHR)
pub type VkWin32SurfaceCreateFlagBitsKHR = VkWin32SurfaceCreateFlagsKHR;

/// See [`VkWin32SurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: win32_wrapper::HINSTANCE,
    pub hwnd: win32_wrapper::HWND,
}

impl Default for VkWin32SurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWin32SurfaceCreateInfoKHR  {
            sType: core::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            hinstance: ptr::null_mut(),
            hwnd: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWin32SurfaceKHR)
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

/// See [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWin32PresentationSupportKHR)
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32) -> core::VkBool32;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWin32SurfaceKHR)
    pub fn vkCreateWin32SurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;

    /// See [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWin32PresentationSupportKHR)
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32) -> core::VkBool32;
}
