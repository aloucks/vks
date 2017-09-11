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

//! [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)

use android_types;
use core;
use khr_surface;
use libc::c_void;
use std::ptr;

pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static [u8; 23] = b"VK_KHR_android_surface\x00";
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME_STR: &'static str = "VK_KHR_android_surface";

vks_bitflags! {
    /// See [`VkAndroidSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateFlagsKHR)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkAndroidSurfaceCreateFlagsKHR: u32 {
        /// See [`VkAndroidSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateFlagsKHR)
        const VK_ANDROID_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}

/// See [`VkAndroidSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateFlagsKHR)
pub type VkAndroidSurfaceCreateFlagBitsKHR = VkAndroidSurfaceCreateFlagsKHR;

/// See [`VkAndroidSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateInfoKHR)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub sType: core::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut android_types::ANativeWindow,
}

impl Default for VkAndroidSurfaceCreateInfoKHR {
    fn default() -> Self {
        VkAndroidSurfaceCreateInfoKHR {
            sType: core::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            window: ptr::null_mut(),
        }
    }
}

/// See [`vkCreateAndroidSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateAndroidSurfaceKHR)
pub type PFN_vkCreateAndroidSurfaceKHR = Option<unsafe extern "system" fn(instance: core::VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateAndroidSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateAndroidSurfaceKHR)
    pub fn vkCreateAndroidSurfaceKHR(instance: core::VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> core::VkResult;
}
