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

//! [`VK_MVK_macos_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_MVK_macos_surface)

use core::ptr;
use khr_surface;
use libc::c_void;
use vk;

pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 2;
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &[u8; 21] = b"VK_MVK_macos_surface\x00";
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME_STR: &str = "VK_MVK_macos_surface";

bitflags! {
    /// See [`VkMacOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateFlagsMVK)
    #[repr(transparent)]
    #[derive(Default)]
    pub struct VkMacOSSurfaceCreateFlagsMVK: u32 {
        const MAX_ENUM_MVK = 0x7fffffff;
    }
}

/// See [`VkMacOSSurfaceCreateFlagsMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateFlagsMVK)
pub type VkMacOSSurfaceCreateFlagBitsMVK = VkMacOSSurfaceCreateFlagsMVK;

/// See [`VkMacOSSurfaceCreateInfoMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMacOSSurfaceCreateInfoMVK)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: vk::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

impl Default for VkMacOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        VkMacOSSurfaceCreateInfoMVK {
            sType: vk::VkStructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            pNext: ptr::null(),
            flags: Default::default(),
            pView: ptr::null(),
        }
    }
}

/// See [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
pub type PFN_vkCreateMacOSSurfaceMVK = Option<unsafe extern "system" fn(instance: vk::VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult>;

#[cfg(feature = "function_prototypes")]
extern "system" {
    /// See [`vkCreateMacOSSurfaceMVK`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMacOSSurfaceMVK)
    pub fn vkCreateMacOSSurfaceMVK(instance: vk::VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const vk::VkAllocationCallbacks, pSurface: *mut khr_surface::VkSurfaceKHR) -> vk::VkResult;
}
