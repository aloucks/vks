// Copyright (c) 2018, Dennis Hamester <dennis.hamester@startmail.com>
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

//! [`VK_MVK_ios_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_ios_surface)

use core::ptr;
use khr_surface;
use libc::c_void;
use vk;

pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: u32 = 2;
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &[u8; 19] = b"VK_MVK_ios_surface\x00";
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME_STR: &str = "VK_MVK_ios_surface";

vks_bitflags! {
    /// See [`VkIOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIOSSurfaceCreateFlagsMVK)
    // #[repr(C)]
    // #[derive(Default)]
    pub struct VkIOSSurfaceCreateFlagsMVK: u32 {
        /// See [`VkIOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIOSSurfaceCreateFlagsMVK)
        const VK_IOS_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_MVK = 0x7fffffff;
    }
}

/// See [`VkIOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIOSSurfaceCreateFlagsMVK)
pub type VkIOSSurfaceCreateFlagBitsMVK = VkIOSSurfaceCreateFlagsMVK;

/// See [`VkIOSSurfaceCreateInfoMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIOSSurfaceCreateInfoMVK)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

impl Default for VkIOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        VkIOSSurfaceCreateInfoMVK {
            sType: vk::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK,
            pNext: ptr::null(),
            flags: Default::default(),
            pView: ptr::null(),
        }
    }
}

/// See [`vkCreateIOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIOSSurfaceMVK)
pub type PFN_vkCreateIOSSurfaceMVK = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateIOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateIOSSurfaceMVK)
    pub fn vkCreateIOSSurfaceMVK(instance: vk::VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;
}
